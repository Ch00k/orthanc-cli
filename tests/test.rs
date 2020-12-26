use dicom_object::{open_file, Error as DicomError, Tag};
use orthanc::*;
use regex::{Regex, RegexBuilder};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use zip;

const DEFAULT_DINO_HOST: &str = "dino"; // docker-compose
const DEFAULT_DINO_PORT: &str = "5252";
const DEFAULT_DINO_AET: &str = "DINO";

const ORTHANC_ID_PATTERN: &str = r"(([0-9a-f]{8}-){4}[0-9a-f]{8})";
const ORTHANC_DICOM_UID_PATTERN: &str = r"1\.2\.276\.0\.7230010\.3\.1\.[2|3]\.([\d|\.]+)";
const ANONYMIZED_PATIENT_ID_PATTERN: &str = r"([0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12})";
const ANONYMIZED_PATIENT_NAME_PATTERN: &str = r"Anonymized(\d+)";
const TRAILING_WHITESPACE_PATTERN: &str = r"([ ]+$)";
const VERSION_PATTERN: &str = r"orthanc \d+\.\d+\.\d+$";
const NEW_ENTITY_ID_PATTERN: &str =
    r"\s*New (?:Patient|Study|Series) ID\s*(([0-9a-f]{8}-){4}[0-9a-f]{8})";
const PARENT_ENTITY_ID_PATTERN: &str =
    r"\s*(?:Patient|Study|Series) ID\s*(([0-9a-f]{8}-){4}[0-9a-f]{8})";

const SOP_INSTANCE_UID: &str = "1.3.46.670589.11.1.5.0.3724.2011072815265975004";
const SOP_INSTANCE_UID_DELETE: &str = "1.3.46.670589.11.1.5.0.7080.2012100313435153441";
const SERIES_INSTANCE_UID: &str = "1.3.46.670589.11.1.5.0.3724.2011072815265926000";
const STUDY_INSTANCE_UID: &str = "1.3.46.670589.11.1.5.0.6560.2011072814060507000";
const PATIENT_ID: &str = "patient_2";

const REPLACEMENT_ORTHANC_ID: &str = "00000000-00000000-00000000-00000000-00000000";
const REPLACEMENT_ORTHANC_DICOM_UID: &str = "0.00.000.0000.00000.000000";
const REPLACEMENT_ANONYMIZED_PATIENT_ID: &str = "00000000-0000-0000-0000-000000000000";

#[derive(Debug)]
struct CommandResult {
    exit_code: i32,
    stdout: String,
    stderr: String,
}

impl CommandResult {
    fn new(exit_code: i32, stdout: String, stderr: String) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
        }
    }

    fn new_entity_id(&self) -> String {
        let new_entity_id_regex = Regex::new(NEW_ENTITY_ID_PATTERN).unwrap();
        let caps = new_entity_id_regex.captures(&self.stdout).unwrap();
        caps.get(1).unwrap().as_str().to_string()
    }

    fn parent_entity_id(&self) -> String {
        let parent_entity_id_regex = Regex::new(PARENT_ENTITY_ID_PATTERN).unwrap();
        let caps = parent_entity_id_regex.captures(&self.stdout).unwrap();
        caps.get(1).unwrap().as_str().to_string()
    }
}

impl PartialEq for CommandResult {
    fn eq(&self, other: &Self) -> bool {
        println!("EXIT_CODE\n{}", self.exit_code);
        println!("STDOUT\n{}", self.stdout);
        println!("STDERR\n{}", self.stderr);

        if self.exit_code != other.exit_code {
            return false;
        };

        let self_stdout = fixup_output(&self.stdout);
        let other_stdout = fixup_output(&other.stdout);
        if self_stdout != other_stdout {
            let self_stdout = sort_output_lines(&self_stdout);
            let other_stdout = sort_output_lines(&other_stdout);
            if self_stdout != other_stdout {
                return false;
            }
        }

        let self_stderr = fixup_output(&self.stderr);
        let other_stderr = fixup_output(&other.stderr);
        if self_stderr != other_stderr {
            let self_stderr = sort_output_lines(&self_stderr);
            let other_stderr = sort_output_lines(&other_stderr);
            if self_stderr != other_stderr {
                return false;
            }
        }
        return true;
    }
}

fn client() -> Client {
    Client::new(env::var("ORC_ORTHANC_SERVER").unwrap()).auth(
        env::var("ORC_ORTHANC_USERNAME").unwrap(),
        env::var("ORC_ORTHANC_PASSWORD").unwrap(),
    )
}

fn find_instance_by_sop_instance_uid(sop_instance_uid: &str) -> Option<Instance> {
    let instances = client().instances_expanded().unwrap();
    for i in instances {
        if i.main_dicom_tags["SOPInstanceUID"] == sop_instance_uid {
            return Some(i);
        }
    }
    return None;
}

fn find_series_by_series_instance_uid(series_instance_uid: &str) -> Option<Series> {
    let series = client().series_expanded().unwrap();
    for s in series {
        if s.main_dicom_tags["SeriesInstanceUID"] == series_instance_uid {
            return Some(s);
        }
    }
    return None;
}

fn find_study_by_study_instance_uid(study_instance_uid: &str) -> Option<Study> {
    let studies = client().studies_expanded().unwrap();
    for s in studies {
        if s.main_dicom_tags["StudyInstanceUID"] == study_instance_uid {
            return Some(s);
        }
    }
    return None;
}

fn find_patient_by_patient_id(patient_id: &str) -> Option<Patient> {
    let patients = client().patients_expanded().unwrap();
    for p in patients {
        if p.main_dicom_tags["PatientID"] == patient_id {
            return Some(p);
        }
    }
    return None;
}

fn fixup_output(output: &str) -> String {
    // TODO: Try using https://crates.io/crates/nom here
    let orthanc_id_regex = RegexBuilder::new(ORTHANC_ID_PATTERN)
        .multi_line(true)
        .build()
        .unwrap();
    let trailing_whitespace_regex = RegexBuilder::new(TRAILING_WHITESPACE_PATTERN)
        .multi_line(true)
        .build()
        .unwrap();
    let version_regex = RegexBuilder::new(VERSION_PATTERN)
        .multi_line(true)
        .build()
        .unwrap();
    let anonymized_patient_id_regex = RegexBuilder::new(ANONYMIZED_PATIENT_ID_PATTERN)
        .multi_line(true)
        .build()
        .unwrap();
    let anonymized_patient_name_regex = RegexBuilder::new(ANONYMIZED_PATIENT_NAME_PATTERN)
        .multi_line(true)
        .build()
        .unwrap();
    let orthanc_dicom_uid_regex = RegexBuilder::new(ORTHANC_DICOM_UID_PATTERN)
        .multi_line(true)
        .build()
        .unwrap();

    let no_orthanc_ids = orthanc_id_regex
        .replace_all(output, REPLACEMENT_ORTHANC_ID)
        .to_string();
    let no_trailing_whitespace = trailing_whitespace_regex
        .replace_all(&no_orthanc_ids, "")
        .to_string();
    let no_version = version_regex
        .replace_all(&no_trailing_whitespace, "orthanc x.y.z")
        .to_string();
    let no_anonymized_patient_id = anonymized_patient_id_regex
        .replace_all(&no_version, REPLACEMENT_ANONYMIZED_PATIENT_ID)
        .to_string();

    let no_anonymized_patient_name = anonymized_patient_name_regex
        .replace_all(&no_anonymized_patient_id, "Anonymized000")
        .to_string();
    let no_orthanc_dicom_uid = orthanc_dicom_uid_regex
        .replace_all(&no_anonymized_patient_name, REPLACEMENT_ORTHANC_DICOM_UID)
        .to_string();
    no_orthanc_dicom_uid
}

fn sort_output_lines(output: &str) -> String {
    let mut output_lines = output.split("\n").collect::<Vec<&str>>();
    output_lines.sort();
    output_lines.join("\n")
}

fn executable_path() -> PathBuf {
    // Adapted from
    // https://github.com/assert-rs/assert_cmd/blob/d9fcca1ac40496afbcdaea719082e5d7f105f4d9/src/cargo.rs#L188
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("orthanc")
}

fn run_command(args: Vec<&str>) -> CommandResult {
    let res = Command::new(executable_path())
        .args(&args)
        .output()
        .unwrap();
    CommandResult::new(
        res.status.code().unwrap(),
        String::from_utf8(res.stdout).unwrap(),
        String::from_utf8(res.stderr).unwrap(),
    )
}

fn assert_result(args: Vec<&str>, expected_result: CommandResult) {
    let res = run_command(args);
    assert!(res == expected_result);
}

#[test]
fn test_server_option() {
    assert_result(
        vec!["--server", "http://localhost:8901", "patient", "list"],
        CommandResult::new(
            1,
            "".to_string(),
            " Error   error sending request for url (http://localhost:8901/patients?expand): error trying to connect: tcp connect error: Connection refused (os error 111)\n".to_string(),
        ),
    );
}

#[test]
fn test_username_option() {
    assert_result(
        vec!["--username", "foo", "patient", "list"],
        CommandResult::new(
            1,
            "".to_string(),
            " Error   API error: 401 Unauthorized \n".to_string(),
        ),
    );
}

#[test]
fn test_password_option() {
    assert_result(
        vec!["--password", "foo", "patient", "list"],
        CommandResult::new(
            1,
            "".to_string(),
            " Error   API error: 401 Unauthorized \n".to_string(),
        ),
    );
}

#[test]
fn _test_server_username_password_options() {
    assert_result(
        vec![
            "--server",
            "http://localhost:8028",
            "--username",
            "orthanc",
            "--password",
            "orthanc",
            "patient",
            "list",
        ],
        CommandResult::new(
            0,
            include_str!("data/patient_list.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_list_patients() {
    assert_result(
        vec!["patient", "list"],
        CommandResult::new(
            0,
            include_str!("data/patient_list.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_list_patients_error() {
    assert_result(
        vec!["-s", "http://example.com", "patient", "list"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/json_parse_error.stderr").to_string(),
        ),
    );
}

#[test]
fn _test_list_studies() {
    assert_result(
        vec!["study", "list"],
        CommandResult::new(
            0,
            include_str!("data/study_list.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_list_studies_error() {
    assert_result(
        vec!["-s", "http://example.com", "study", "list"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/json_parse_error.stderr").to_string(),
        ),
    );
}

#[test]
fn _test_list_series() {
    assert_result(
        vec!["series", "list"],
        CommandResult::new(
            0,
            include_str!("data/series_list.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_list_series_error() {
    assert_result(
        vec!["-s", "http://example.com", "series", "list"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/json_parse_error.stderr").to_string(),
        ),
    );
}

#[test]
fn _test_list_instances() {
    assert_result(
        vec!["instance", "list"],
        CommandResult::new(
            0,
            include_str!("data/instance_list.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_list_instances_error() {
    assert_result(
        vec!["-s", "http://example.com", "instance", "list"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/json_parse_error.stderr").to_string(),
        ),
    );
}

#[test]
fn _test_show_patient() {
    let patient = find_patient_by_patient_id(PATIENT_ID).unwrap();
    assert_result(
        vec!["patient", "show", &patient.id],
        CommandResult::new(
            0,
            include_str!("data/patient_show.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_show_patient_error() {
    assert_result(
        vec!["patient", "show", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn _test_show_study() {
    let study = find_study_by_study_instance_uid(STUDY_INSTANCE_UID).unwrap();
    assert_result(
        vec!["study", "show", &study.id],
        CommandResult::new(
            0,
            include_str!("data/study_show.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_show_study_error() {
    assert_result(
        vec!["study", "show", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn _test_show_series() {
    let series = find_series_by_series_instance_uid(SERIES_INSTANCE_UID).unwrap();
    assert_result(
        vec!["series", "show", &series.id],
        CommandResult::new(
            0,
            include_str!("data/series_show.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_show_series_error() {
    assert_result(
        vec!["series", "show", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn _test_show_instance() {
    let instance = find_instance_by_sop_instance_uid(SOP_INSTANCE_UID).unwrap();
    assert_result(
        vec!["instance", "show", &instance.id],
        CommandResult::new(
            0,
            include_str!("data/instance_show.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn _test_show_instance_error() {
    assert_result(
        vec!["instance", "show", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_download_patient() {
    let patient = find_patient_by_patient_id(PATIENT_ID).unwrap();
    assert_result(
        vec!["patient", "download", &patient.id, "-o", "/tmp/patient.zip"],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    let file = fs::File::open("/tmp/patient.zip").unwrap();
    let reader = BufReader::new(file);
    let zip = zip::ZipArchive::new(reader).unwrap();
    let mut files: Vec<&str> = zip.file_names().collect();
    files.sort();

    assert_eq!(
        files,
        vec![
            "patient_2 Patient 2/REMOVED Study 1/MR Series 1/MR000000.dcm",
            "patient_2 Patient 2/REMOVED Study 1/PR/PR000000.dcm",
        ]
    );
}

#[test]
fn test_download_patient_error() {
    assert_result(
        vec!["patient", "download", "foobar", "-o", "/tmp/patient.zip"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_download_study() {
    let study = find_study_by_study_instance_uid(STUDY_INSTANCE_UID).unwrap();
    assert_result(
        vec!["study", "download", &study.id, "-o", "/tmp/study.zip"],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    let file = fs::File::open("/tmp/study.zip").unwrap();
    let reader = BufReader::new(file);
    let zip = zip::ZipArchive::new(reader).unwrap();
    let mut files: Vec<&str> = zip.file_names().collect();
    files.sort();

    assert_eq!(
        files,
        vec![
            "patient_2 Patient 2/REMOVED Study 1/MR Series 1/MR000000.dcm",
            "patient_2 Patient 2/REMOVED Study 1/PR/PR000000.dcm",
        ]
    );
}

#[test]
fn test_download_study_error() {
    assert_result(
        vec!["study", "download", "foobar", "-o", "/tmp/study.zip"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_download_series() {
    let series = find_series_by_series_instance_uid(SERIES_INSTANCE_UID).unwrap();
    assert_result(
        vec!["series", "download", &series.id, "-o", "/tmp/series.zip"],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    let file = fs::File::open("/tmp/series.zip").unwrap();
    let reader = BufReader::new(file);
    let zip = zip::ZipArchive::new(reader).unwrap();
    let mut files: Vec<&str> = zip.file_names().collect();
    files.sort();

    assert_eq!(
        files,
        vec!["patient_2 Patient 2/REMOVED Study 1/MR Series 1/MR000000.dcm",]
    );
}

#[test]
fn test_download_series_error() {
    assert_result(
        vec!["series", "download", "foobar", "-o", "/tmp/series.zip"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_download_instance() {
    let instance = find_instance_by_sop_instance_uid(SOP_INSTANCE_UID).unwrap();
    assert_result(
        vec![
            "instance",
            "download",
            &instance.id,
            "-o",
            "/tmp/instance.dcm",
        ],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    assert!(Path::new("/tmp/instance.dcm").exists());
    // TODO: At least check that it is a DICOM file.
    // Perhaps also check that it contains some DICOM
    // tags.
}

#[test]
fn test_download_instance_error() {
    assert_result(
        vec!["instance", "download", "foobar", "-o", "/tmp/instance.zip"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_patient_no_customization() {
    let patient = find_patient_by_patient_id(PATIENT_ID).unwrap();
    let res = run_command(vec!["patient", "anonymize", &patient.id]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_patient.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_patient_id = res.new_entity_id();
    assert_result(
        vec!["patient", "show", &new_patient_id],
        CommandResult::new(
            0,
            include_str!("data/patient_show_anonymized_no_config.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_study_no_customization() {
    let study = find_study_by_study_instance_uid(STUDY_INSTANCE_UID).unwrap();
    let res = run_command(vec!["study", "anonymize", &study.id]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_study.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_study_id = res.new_entity_id();
    assert_result(
        vec!["study", "show", &new_study_id],
        CommandResult::new(
            0,
            include_str!("data/study_show_anonymized_no_config.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_series_no_customization() {
    let series = find_series_by_series_instance_uid(SERIES_INSTANCE_UID).unwrap();
    let res = run_command(vec!["series", "anonymize", &series.id]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_series.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_series_id = res.new_entity_id();
    assert_result(
        vec!["series", "show", &new_series_id],
        CommandResult::new(
            0,
            include_str!("data/series_show_anonymized_no_config.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_instance_no_customization() {
    let instance = find_instance_by_sop_instance_uid(SOP_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "instance",
        "anonymize",
        &instance.id,
        "-o",
        "/tmp/anonymized_instance.dcm",
    ]);
    assert!(res == CommandResult::new(0, "".to_string(), "".to_string()));
    let obj = open_file("/tmp/anonymized_instance.dcm").unwrap();
    assert!(obj
        .element_by_name("PatientName")
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with("Anonymized"));

    let anonymized_patient_id_regex = RegexBuilder::new(ANONYMIZED_PATIENT_ID_PATTERN)
        .multi_line(true)
        .build()
        .unwrap();
    let patient_id = obj.element_by_name("PatientID").unwrap().to_str().unwrap();
    assert!(anonymized_patient_id_regex.is_match(&patient_id));
}

#[test]
fn test_anonymize_patient_with_config() {
    let mut file = fs::File::create("/tmp/patient_anon_config.yml").unwrap();
    file.write_all(include_bytes!("data/patient_anonymization_config.yml"))
        .unwrap();
    let patient = find_patient_by_patient_id(PATIENT_ID).unwrap();
    let res = run_command(vec![
        "patient",
        "anonymize",
        &patient.id,
        "-c",
        "/tmp/patient_anon_config.yml",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_patient.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_patient_id = res.new_entity_id();
    assert_result(
        vec!["patient", "show", &new_patient_id],
        CommandResult::new(
            0,
            include_str!("data/patient_show_anonymized_with_config.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_patient_with_cmd_options() {
    let patient = find_patient_by_patient_id(PATIENT_ID).unwrap();
    let res = run_command(vec![
        "patient",
        "anonymize",
        &patient.id,
        "-r",
        "PatientID=gazorpazorp",
        "PatientName=Morty Smith",
        "-k",
        "PatientSex",
        "PatientBirthDate",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_patient.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_patient_id = res.new_entity_id();
    assert_result(
        vec!["patient", "show", &new_patient_id],
        CommandResult::new(
            0,
            include_str!("data/patient_show_anonymized_with_config.stdout")
                .to_string()
                // TODO: This is getting out of hand
                .replace("Number of Studies   2", "Number of Studies   1"),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_study_with_config() {
    let mut file = fs::File::create("/tmp/study_anon_config.yml").unwrap();
    file.write_all(include_bytes!("data/study_anonymization_config.yml"))
        .unwrap();
    let study = find_study_by_study_instance_uid(STUDY_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "study",
        "anonymize",
        &study.id,
        "-c",
        "/tmp/study_anon_config.yml",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_study.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_study_id = res.new_entity_id();
    assert_result(
        vec!["study", "show", &new_study_id],
        CommandResult::new(
            0,
            include_str!("data/study_show_anonymized_with_config.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_study_with_cmd_options() {
    let study = find_study_by_study_instance_uid(STUDY_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "study",
        "anonymize",
        &study.id,
        "-r",
        "StudyID=gazorpazorp",
        "AccessionNumber=C137",
        "-k",
        "StudyDate",
        "StudyTime",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_study.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_study_id = res.new_entity_id();
    assert_result(
        vec!["study", "show", &new_study_id],
        CommandResult::new(
            0,
            include_str!("data/study_show_anonymized_with_config.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_series_with_config() {
    let mut file = fs::File::create("/tmp/series_anon_config.yml").unwrap();
    file.write_all(include_bytes!("data/series_anonymization_config.yml"))
        .unwrap();
    let series = find_series_by_series_instance_uid(SERIES_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "series",
        "anonymize",
        &series.id,
        "-c",
        "/tmp/series_anon_config.yml",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_series.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_series_id = res.new_entity_id();
    assert_result(
        vec!["series", "show", &new_series_id],
        CommandResult::new(
            0,
            include_str!("data/series_show_anonymized_with_config.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_series_with_cmd_options() {
    let series = find_series_by_series_instance_uid(SERIES_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "series",
        "anonymize",
        &series.id,
        "-r",
        "SeriesNumber=42",
        "BodyPartExamined=PINKY",
        "-k",
        "SeriesDescription",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_series.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_series_id = res.new_entity_id();
    assert_result(
        vec!["series", "show", &new_series_id],
        CommandResult::new(
            0,
            include_str!("data/series_show_anonymized_with_config.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_anonymize_instance_with_config() {
    let mut file = fs::File::create("/tmp/instance_anon_config.yml").unwrap();
    file.write_all(include_bytes!("data/instance_anonymization_config.yml"))
        .unwrap();
    let instance = find_instance_by_sop_instance_uid(SOP_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "instance",
        "anonymize",
        &instance.id,
        "-c",
        "/tmp/instance_anon_config.yml",
        "-o",
        "/tmp/anonymized_instance.dcm",
    ]);
    assert!(res == CommandResult::new(0, "".to_string(), "".to_string()));
    let obj = open_file("/tmp/anonymized_instance.dcm").unwrap();
    assert_eq!(
        obj.element_by_name("PatientID").unwrap().to_str().unwrap(),
        "C137"
    );
    assert_eq!(
        obj.element_by_name("PatientName")
            .unwrap()
            .to_str()
            .unwrap(),
        "Patient 2 " // TODO: Why is there a trailing space?
    );
    assert!(
        matches!(obj.element(Tag::from((0x1235, 0x0042))).unwrap_err(), DicomError::NoSuchDataElementTag{..})
    );
}

#[test]
fn test_anonymize_instance_with_cmd_options() {
    let instance = find_instance_by_sop_instance_uid(SOP_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "instance",
        "anonymize",
        &instance.id,
        "-r",
        "PatientID=C137",
        "-k",
        "PatientName",
        "-o",
        "/tmp/anonymized_instance.dcm",
        "-p",
    ]);
    assert!(res == CommandResult::new(0, "".to_string(), "".to_string()));
    let obj = open_file("/tmp/anonymized_instance.dcm").unwrap();
    assert_eq!(
        obj.element_by_name("PatientID").unwrap().to_str().unwrap(),
        "C137"
    );
    assert_eq!(
        obj.element_by_name("PatientName")
            .unwrap()
            .to_str()
            .unwrap(),
        "Patient 2 " // TODO: Why is there a trailing space?
    );
    assert_eq!(
        obj.element(Tag::from((0x1235, 0x0042)))
            .unwrap()
            .to_str()
            .unwrap(),
        "bazqux"
    );
}

#[test]
fn test_anonymize_patient_error() {
    assert_result(
        vec!["patient", "anonymize", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_patient_config_file_not_found() {
    assert_result(
        vec!["patient", "anonymize", "foobar", "-c", "/tmp/foo.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/no_such_file_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_patient_yaml_parse_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"garble").unwrap();
    assert_result(
        vec![
            "patient",
            "anonymize",
            "foobar",
            "-c",
            "/tmp/mod_config.yml",
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/yaml_parse_error_anonymization.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_study_error() {
    assert_result(
        vec!["study", "anonymize", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_study_config_file_not_found() {
    assert_result(
        vec!["study", "anonymize", "foobar", "-c", "/tmp/foo.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/no_such_file_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_study_yaml_parse_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"garble").unwrap();
    assert_result(
        vec!["study", "anonymize", "foobar", "-c", "/tmp/mod_config.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/yaml_parse_error_anonymization.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_series_error() {
    assert_result(
        vec!["series", "anonymize", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_series_config_file_not_found() {
    assert_result(
        vec!["series", "anonymize", "foobar", "-c", "/tmp/foo.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/no_such_file_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_series_yaml_parse_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"garble").unwrap();
    assert_result(
        vec!["series", "anonymize", "foobar", "-c", "/tmp/mod_config.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/yaml_parse_error_anonymization.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_instance_error() {
    assert_result(
        vec!["instance", "anonymize", "foobar", "-o", "/tmp/instance.dcm"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_instance_config_file_not_found() {
    assert_result(
        vec![
            "instance",
            "anonymize",
            "foobar",
            "-c",
            "/tmp/foo.yml",
            "-o",
            "/tmp/instance.dcm",
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/no_such_file_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_anonymize_instance_yaml_parse_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"garble").unwrap();
    assert_result(
        vec![
            "instance",
            "anonymize",
            "foobar",
            "-c",
            "/tmp/mod_config.yml",
            "-o",
            "/tmp/instance.dcm",
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/yaml_parse_error_anonymization.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_patient() {
    let mut file = fs::File::create("/tmp/patient_mod_config.yml").unwrap();
    file.write_all(include_bytes!("data/patient_modification_config.yml"))
        .unwrap();
    let patient = find_patient_by_patient_id(PATIENT_ID).unwrap();
    let res = run_command(vec![
        "patient",
        "modify",
        &patient.id,
        "-c",
        "/tmp/patient_mod_config.yml",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_patient.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_patient_id = res.new_entity_id();
    assert_result(
        vec!["patient", "show", &new_patient_id],
        CommandResult::new(
            0,
            include_str!("data/patient_show_modified.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_modify_study() {
    let mut file = fs::File::create("/tmp/study_mod_config.yml").unwrap();
    file.write_all(include_bytes!("data/study_modification_config.yml"))
        .unwrap();
    let study = find_study_by_study_instance_uid(STUDY_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "study",
        "modify",
        &study.id,
        "-c",
        "/tmp/study_mod_config.yml",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_study.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_study_id = res.new_entity_id();
    assert_result(
        vec!["study", "show", &new_study_id],
        CommandResult::new(
            0,
            include_str!("data/study_show_modified.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_modify_series() {
    let mut file = fs::File::create("/tmp/series_mod_config.yml").unwrap();
    file.write_all(include_bytes!("data/series_modification_config.yml"))
        .unwrap();
    let series = find_series_by_series_instance_uid(SERIES_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "series",
        "modify",
        &series.id,
        "-c",
        "/tmp/series_mod_config.yml",
    ]);
    assert!(
        res == CommandResult::new(
            0,
            include_str!("data/anonymize_series.stdout").to_string(),
            "".to_string(),
        ),
    );
    let new_series_id = res.new_entity_id();
    assert_result(
        vec!["series", "show", &new_series_id],
        CommandResult::new(
            0,
            include_str!("data/series_show_modified.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_modify_instance() {
    let mut file = fs::File::create("/tmp/instance_mod_config.yml").unwrap();
    file.write_all(include_bytes!("data/instance_modification_config.yml"))
        .unwrap();
    let instance = find_instance_by_sop_instance_uid(SOP_INSTANCE_UID).unwrap();
    let res = run_command(vec![
        "instance",
        "modify",
        &instance.id,
        "-c",
        "/tmp/instance_mod_config.yml",
        "-o",
        "/tmp/modified_instance.dcm",
    ]);
    assert!(res == CommandResult::new(0, "".to_string(), "".to_string()));
    let obj = open_file("/tmp/modified_instance.dcm").unwrap();
    assert_eq!(
        obj.element_by_name("SpecificCharacterSet")
            .unwrap()
            .to_str()
            .unwrap(),
        "ISO_IR 13 "
    );
    assert_eq!(
        obj.element_by_name("OperatorsName")
            .unwrap()
            .to_str()
            .unwrap(),
        "Summer Smith"
    );
}

#[test]
fn test_modify_patient_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"remove:\nreplace:").unwrap();
    assert_result(
        vec!["patient", "modify", "foobar", "-c", "/tmp/mod_config.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_patient_config_file_not_found() {
    assert_result(
        vec!["patient", "modify", "foobar", "-c", "/tmp/foo.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/no_such_file_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_patient_yaml_parse_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"garble").unwrap();
    assert_result(
        vec!["patient", "modify", "foobar", "-c", "/tmp/mod_config.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/yaml_parse_error_modification.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_study_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"remove:\nreplace:").unwrap();
    assert_result(
        vec!["study", "modify", "foobar", "-c", "/tmp/mod_config.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_study_config_file_not_found() {
    assert_result(
        vec!["study", "modify", "foobar", "-c", "/tmp/foo.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/no_such_file_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_study_yaml_parse_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"garble").unwrap();
    assert_result(
        vec!["study", "modify", "foobar", "-c", "/tmp/mod_config.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/yaml_parse_error_modification.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_series_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"remove:\nreplace:").unwrap();
    assert_result(
        vec!["series", "modify", "foobar", "-c", "/tmp/mod_config.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_series_config_file_not_found() {
    assert_result(
        vec!["series", "modify", "foobar", "-c", "/tmp/foo.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/no_such_file_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_series_yaml_parse_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"garble").unwrap();
    assert_result(
        vec!["series", "modify", "foobar", "-c", "/tmp/mod_config.yml"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/yaml_parse_error_modification.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_instance_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"remove:\nreplace:").unwrap();
    assert_result(
        vec![
            "instance",
            "modify",
            "foobar",
            "-o",
            "/tmp/instance.dcm",
            "-c",
            "/tmp/mod_config.yml",
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_instance_config_file_not_found() {
    assert_result(
        vec![
            "instance",
            "modify",
            "foobar",
            "-c",
            "/tmp/foo.yml",
            "-o",
            "/tmp/instance.dcm",
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/no_such_file_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modify_instance_yaml_parse_error() {
    let mut file = fs::File::create("/tmp/mod_config.yml").unwrap();
    file.write(b"garble").unwrap();
    assert_result(
        vec![
            "instance",
            "modify",
            "foobar",
            "-c",
            "/tmp/mod_config.yml",
            "-o",
            "/tmp/instance.dcm",
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/yaml_parse_error_modification.stderr").to_string(),
        ),
    );
}

#[test]
fn test_delete() {
    let instance = find_instance_by_sop_instance_uid(SOP_INSTANCE_UID_DELETE).unwrap();
    let series = run_command(vec!["instance", "show", &instance.id]).parent_entity_id();
    let study = run_command(vec!["series", "show", &series]).parent_entity_id();
    let patient = run_command(vec!["study", "show", &study]).parent_entity_id();

    // Instance
    assert_result(
        vec!["instance", "delete", &instance.id],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    assert_result(
        vec!["instance", "show", &instance.id],
        CommandResult::new(
            1,
            "".to_string(),
            " Error   API error: 404 Not Found \n".to_string(),
        ),
    );

    // Series
    assert_result(
        vec!["series", "delete", &series],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    assert_result(
        vec!["series", "show", &series],
        CommandResult::new(
            1,
            "".to_string(),
            " Error   API error: 404 Not Found \n".to_string(),
        ),
    );

    // Study
    assert_result(
        vec!["study", "delete", &study],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    assert_result(
        vec!["study", "show", &study],
        CommandResult::new(
            1,
            "".to_string(),
            " Error   API error: 404 Not Found \n".to_string(),
        ),
    );

    // Patient
    assert_result(
        vec!["patient", "delete", &patient],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    assert_result(
        vec!["patient", "show", &patient],
        CommandResult::new(
            1,
            "".to_string(),
            " Error   API error: 404 Not Found \n".to_string(),
        ),
    );
}

#[test]
fn test_delete_patient_error() {
    assert_result(
        vec!["patient", "delete", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_delete_study_error() {
    assert_result(
        vec!["study", "delete", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_delete_series_error() {
    assert_result(
        vec!["series", "delete", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_delete_instance_error() {
    assert_result(
        vec!["instance", "delete", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modalities() {
    // Get system info
    let sysinfo = client().system().unwrap();

    // Create
    assert_result(
        vec![
            "modality", "create", "foo", "--aet", "FOO", "--host", "4.3.2.1", "--port", "42",
        ],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    assert_result(
        vec![
            "modality", "create", "bar", "--aet", "BAR", "--host", "1.2.3.4", "--port", "1717",
        ],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );

    // List
    assert_result(
        vec!["modality", "list"],
        CommandResult::new(
            0,
            include_str!("data/modality_list.stdout").to_string(),
            "".to_string(),
        ),
    );

    // Modify
    assert_result(
        vec![
            "modality", "modify", "bar", "--aet", "BAZ", "--host", "9.8.7.6", "--port", "1717",
        ],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );

    // Show
    if sysinfo.api_version > 6 {
        assert_result(
            vec!["modality", "show", "bar"],
            CommandResult::new(
                0,
                include_str!("data/modality_show.stdout").to_string(),
                "".to_string(),
            ),
        );
    } else {
        assert_result(
            vec!["modality", "show", "bar"],
            CommandResult::new(
                0,
                include_str!("data/modality_show_1.6.stdout").to_string(),
                "".to_string(),
            ),
        );
    };

    // Delete
    assert_result(
        vec!["modality", "delete", "bar"],
        CommandResult::new(0, "".to_string(), "".to_string()),
    );
    assert_result(
        vec!["modality", "show", "bar"],
        CommandResult::new(
            1,
            "".to_string(),
            " Error   Modality bar not found \n".to_string(),
        ),
    );
}

#[test]
fn modality_create_error() {
    assert_result(
        vec![
            "-s",
            "http://example.com",
            "modality",
            "create",
            "foo",
            "--aet",
            "FOO",
            "--host",
            "4.3.2.1",
            "--port",
            "42",
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/json_parse_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modality_list_error() {
    assert_result(
        vec!["-s", "http://example.com", "modality", "list"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/json_parse_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_modality_show_error() {
    assert_result(
        vec!["modality", "show", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/modality_not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn modality_modify_error() {
    assert_result(
        vec![
            "-s",
            "http://example.com",
            "modality",
            "modify",
            "foo",
            "--aet",
            "FOO",
            "--host",
            "4.3.2.1",
            "--port",
            "42",
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/json_parse_error.stderr").to_string(),
        ),
    );
}

// Orthanc always returns a 200 even if the modality does not exist
//#[test]
//fn test_modality_delete_error() {
//    assert_result(
//        vec!["modality", "delete", "garble"],
//        CommandResult::new(
//            1,
//            "".to_string(),
//            include_str!("data/modality_not_found_error.stderr").to_string(),
//        ),
//    );
//}

#[test]
fn test_modality_store() {
    let modality = Modality {
        aet: env::var("DINO_SCP_AET").unwrap_or(DEFAULT_DINO_AET.to_string()),
        host: DEFAULT_DINO_HOST.to_string(),
        port: env::var("DINO_SCP_PORT")
            .unwrap_or(DEFAULT_DINO_PORT.to_string())
            .parse::<i32>()
            .unwrap(),
        manufacturer: None,
        allow_c_echo: None,
        allow_c_find: None,
        allow_c_get: None,
        allow_c_move: None,
        allow_c_store: None,
        allow_n_action: None,
        allow_n_event_report: None,
        allow_transcoding: None,
    };
    client().create_modality("dino", modality).unwrap();

    assert_result(
        vec![
            "modality",
            "store",
            "dino",
            "-e",
            &find_study_by_study_instance_uid(STUDY_INSTANCE_UID)
                .unwrap()
                .id,
        ],
        CommandResult::new(
            0,
            include_str!("data/modality_store.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_modality_store_error() {
    assert_result(
        vec![
            "modality",
            "store",
            "garble",
            "-e",
            &find_study_by_study_instance_uid(STUDY_INSTANCE_UID)
                .unwrap()
                .id,
        ],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/modality_delete_not_found_error.stderr").to_string(),
        ),
    );
}

#[test]
fn test_instance_tags() {
    assert_result(
        vec![
            "instance",
            "tags",
            &find_instance_by_sop_instance_uid(SOP_INSTANCE_UID)
                .unwrap()
                .id,
        ],
        CommandResult::new(
            0,
            include_str!("data/instance_tags.stdout").to_string(),
            "".to_string(),
        ),
    );
}

#[test]
fn test_instance_tags_error() {
    assert_result(
        vec!["instance", "tags", "foobar"],
        CommandResult::new(
            1,
            "".to_string(),
            include_str!("data/not_found_with_message_error.stderr").to_string(),
        ),
    );
}

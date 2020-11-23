use comfy_table::{ColumnConstraint, ContentArrangement, Table};
use orthanc::{Anonymization, Client, Error, Modality, Modification};
use serde_json::Value;
use serde_yaml;
use std::{env, fs, process, result};

const TABLE_PRESET: &str = "     --            ";
const ID_COLUMN_WIDTH: u16 = 46;
const ABSENT_DICOM_TAG_PLACEHOLDER: &str = "undefined";

const PATIENTS_LIST_HEADER: [&str; 4] = ["ID", "PatientID", "PatientName", "Number of studies"];
const PATIENTS_LIST_DICOM_TAGS: [&str; 2] = ["PatientID", "PatientName"];
const PATIENT_DICOM_TAGS: [&str; 4] =
    ["PatientID", "PatientName", "PatientSex", "PatientBirthDate"];

const STUDIES_LIST_HEADER: [&str; 8] = [
    "ID",
    "PatientID",
    "AccessionNumber",
    "StudyInstanceUID",
    "StudyDescription",
    "StudyDate",
    "StudyTime",
    "Number of series",
];
const STUDIES_LIST_DICOM_TAGS: [&str; 6] = [
    "PatientID",
    "AccessionNumber",
    "StudyInstanceUID",
    "StudyDescription",
    "StudyDate",
    "StudyTime",
];
const STUDY_DICOM_TAGS: [&str; 7] = [
    "PatientID",
    "StudyID",
    "AccessionNumber",
    "StudyInstanceUID",
    "StudyDescription",
    "StudyDate",
    "StudyTime",
];

const SERIES_LIST_HEADER: [&str; 6] = [
    "ID",
    "SeriesInstanceUID",
    "SeriesDescription",
    "Modality",
    "BodyPartExamined",
    "Number of instances",
];
const SERIES_LIST_DICOM_TAGS: [&str; 4] = [
    "SeriesInstanceUID",
    "SeriesDescription",
    "Modality",
    "BodyPartExamined",
];
const SERIES_DICOM_TAGS: [&str; 5] = [
    "SeriesInstanceUID",
    "SeriesNumber",
    "SeriesDescription",
    "Modality",
    "BodyPartExamined",
];

const INSTANCES_LIST_HEADER: [&str; 7] = [
    "ID",
    "SOPInstanceUID",
    "InstanceNumber",
    "InstanceCreationDate",
    "InstanceCreationTime",
    "Index in series",
    "File size",
];
const INSTANCES_LIST_DICOM_TAGS: [&str; 4] = [
    "SOPInstanceUID",
    "InstanceNumber",
    "InstanceCreationDate",
    "InstanceCreationTime",
];
const INSTANCE_DICOM_TAGS: [&str; 4] = [
    "SOPInstanceUID",
    "InstanceNumber",
    "InstanceCreationDate",
    "InstanceCreationTime",
];

const MODALITIES_LIST_HEADER: [&str; 5] = ["Name", "AET", "Host", "Port", "Manufacturer"];

type Result<T> = result::Result<T, CliError>;

#[derive(Debug)]
pub struct Orthanc {
    pub client: Client,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CliError {
    error: String,
    message: Option<String>,
    details: Option<String>,
}

impl CliError {
    pub fn new(error: &str, message: Option<&str>, details: Option<&str>) -> CliError {
        CliError {
            error: error.to_string(),
            message: message.map(String::from),
            details: details.map(String::from),
        }
    }
}

impl From<Error> for CliError {
    fn from(e: Error) -> Self {
        let mut err = CliError::new(&e.message.to_string(), None, None);
        match e.details {
            Some(d) => {
                err.message = Some(d.message);
                match d.details {
                    Some(d) => err.details = Some(d),
                    None => (),
                }
            }
            None => (),
        };
        err
    }
}

impl Orthanc {
    pub fn new(
        server_address: String,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Orthanc> {
        let mut client = Client::new(server_address);
        client = match (username, password) {
            (Some(u), Some(p)) => client.auth(u, p),
            _ => client,
        };
        Ok(Orthanc { client })
    }

    ////////// PATIENT //////////

    pub fn list_patients(&self) -> Result<Table> {
        let patients = self.client.patients_expanded()?;

        let mut table = create_table(Some(&PATIENTS_LIST_HEADER));
        for p in patients {
            let mut row: Vec<&str> = vec![&p.id];
            for t in PATIENTS_LIST_DICOM_TAGS.iter() {
                let val = p.main_dicom_tag(t).unwrap_or(ABSENT_DICOM_TAG_PLACEHOLDER);
                row.push(val);
            }
            let num_studies = format!("{}", p.studies.len());
            row.push(&num_studies);
            table.add_row(row.iter());
        }
        let id_column = table.get_column_mut(0).unwrap();
        id_column.set_constraint(ColumnConstraint::MinWidth(ID_COLUMN_WIDTH));
        Ok(table)
    }

    pub fn show_patient(&self, patient_id: &str) -> Result<Table> {
        let patient = self.client.patient(patient_id)?;
        let mut table = create_table(None);
        table.add_row(["ID", &patient.id].iter());

        for t in PATIENT_DICOM_TAGS.iter() {
            table.add_row(
                [
                    t,
                    &patient
                        .main_dicom_tag(t)
                        .unwrap_or(ABSENT_DICOM_TAG_PLACEHOLDER),
                ]
                .iter(),
            );
        }
        let num_studies = format!("{}", patient.studies.len());
        table.add_row(["Number of studies", &num_studies].iter());
        Ok(table)
    }

    pub fn anonymize_patient(&self, id: &str, config_file: Option<&str>) -> Result<Table> {
        let anonymization = match config_file {
            Some(c) => {
                let yaml = fs::read(c).unwrap();
                let mut a: Anonymization = serde_yaml::from_slice(&yaml).unwrap();
                a.force = Some(true);
                Some(a)
            }
            None => None,
        };
        match self.client.anonymize_patient(id, anonymization) {
            Ok(r) => {
                let mut table = create_table(None);
                table.add_row(["New patient ID", &r.id].iter());
                Ok(table)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn modify_patient(&self, id: &str, config_file: &str) -> Result<Table> {
        let yaml = fs::read(config_file).unwrap();
        let modification: Modification = serde_yaml::from_slice(&yaml).unwrap();
        match self.client.modify_patient(id, modification) {
            Ok(r) => {
                let mut table = create_table(None);
                table.add_row(["New patient ID", &r.id].iter());
                Ok(table)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn download_patient(&self, id: &str, output_file: &str) -> Result<()> {
        let mut file = fs::File::create(output_file).unwrap();
        self.client
            .patient_dicom(id, &mut file)
            .map_err(Into::<_>::into)
    }

    pub fn delete_patient(&self, id: &str) -> Result<()> {
        match self.client.delete_patient(id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    ////////// STUDY //////////

    pub fn list_studies(&self, patient_id: Option<&str>) -> Result<Table> {
        let mut studies = self.client.studies_expanded()?;
        match patient_id {
            Some(p) => studies.retain(|s| s.parent_patient == p),
            None => (),
        }

        let mut table = create_table(Some(&STUDIES_LIST_HEADER));
        for s in studies {
            let mut row: Vec<&str> = vec![&s.id];
            for t in STUDIES_LIST_DICOM_TAGS.iter() {
                let val = s.main_dicom_tag(t).unwrap_or(ABSENT_DICOM_TAG_PLACEHOLDER);
                row.push(val);
            }
            let num_series = format!("{}", s.series.len());
            row.push(&num_series);
            table.add_row(row.iter());
        }
        let id_column = table.get_column_mut(0).unwrap();
        id_column.set_constraint(ColumnConstraint::MinWidth(ID_COLUMN_WIDTH));
        Ok(table)
    }

    pub fn show_study(&self, study_id: &str) -> Result<Table> {
        let study = self.client.study(study_id)?;
        let mut table = create_table(None);
        table.add_row(["ID", &study.id].iter());
        table.add_row(["Patient ID", &study.parent_patient].iter());

        for t in STUDY_DICOM_TAGS.iter() {
            table.add_row(
                [
                    t,
                    &study
                        .main_dicom_tag(t)
                        .unwrap_or(ABSENT_DICOM_TAG_PLACEHOLDER),
                ]
                .iter(),
            );
        }
        let num_series = format!("{}", study.series.len());
        table.add_row(["Number of series", &num_series].iter());
        Ok(table)
    }

    pub fn anonymize_study(&self, id: &str, config_file: Option<&str>) -> Result<Table> {
        let anonymization = match config_file {
            Some(c) => {
                let yaml = fs::read(c).unwrap();
                Some(serde_yaml::from_slice(&yaml).unwrap())
            }
            None => None,
        };
        match self.client.anonymize_study(id, anonymization) {
            Ok(r) => {
                let mut table = create_table(None);
                table.add_row(["New study ID", &r.id].iter());
                table.add_row(["Patient ID", &r.patient_id].iter());
                Ok(table)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn modify_study(&self, id: &str, config_file: &str) -> Result<Table> {
        let yaml = fs::read(config_file).unwrap();
        let modification: Modification = serde_yaml::from_slice(&yaml).unwrap();
        match self.client.modify_study(id, modification) {
            Ok(r) => {
                let mut table = create_table(None);
                table.add_row(["New study ID", &r.id].iter());
                table.add_row(["Patient ID", &r.patient_id].iter());
                Ok(table)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn download_study(&self, id: &str, output_file: &str) -> Result<()> {
        let mut file = fs::File::create(output_file).unwrap();
        self.client
            .study_dicom(id, &mut file)
            .map_err(Into::<_>::into)
    }

    pub fn delete_study(&self, id: &str) -> Result<()> {
        match self.client.delete_study(id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    ////////// SERIES //////////

    pub fn list_series(&self, study_id: Option<&str>) -> Result<Table> {
        let mut series = self.client.series_expanded()?;
        match study_id {
            Some(p) => series.retain(|s| s.parent_study == p),
            None => (),
        }

        let mut table = create_table(Some(&SERIES_LIST_HEADER));
        for s in series {
            let mut row: Vec<&str> = vec![&s.id];
            for t in SERIES_LIST_DICOM_TAGS.iter() {
                let val = s.main_dicom_tag(t).unwrap_or(ABSENT_DICOM_TAG_PLACEHOLDER);
                row.push(val);
            }
            let num_instances = format!("{}", s.instances.len());
            row.push(&num_instances);
            table.add_row(row.iter());
        }
        let id_column = table.get_column_mut(0).unwrap();
        id_column.set_constraint(ColumnConstraint::MinWidth(ID_COLUMN_WIDTH));
        Ok(table)
    }

    pub fn show_series(&self, series_id: &str) -> Result<Table> {
        let series = self.client.series(series_id)?;
        let mut table = create_table(None);
        table.add_row(["ID", &series.id].iter());
        table.add_row(["Study ID", &series.parent_study].iter());

        for t in SERIES_DICOM_TAGS.iter() {
            table.add_row(
                [
                    t,
                    &series
                        .main_dicom_tag(t)
                        .unwrap_or(ABSENT_DICOM_TAG_PLACEHOLDER),
                ]
                .iter(),
            );
        }
        let num_instances = format!("{}", series.instances.len());
        table.add_row(["Number of instances", &num_instances].iter());
        Ok(table)
    }

    pub fn anonymize_series(&self, id: &str, config_file: Option<&str>) -> Result<Table> {
        let anonymization = match config_file {
            Some(c) => {
                let yaml = fs::read(c).unwrap();
                Some(serde_yaml::from_slice(&yaml).unwrap())
            }
            None => None,
        };
        match self.client.anonymize_series(id, anonymization) {
            Ok(r) => {
                let mut table = create_table(None);
                table.add_row(["New series ID", &r.id].iter());
                table.add_row(["Patient ID", &r.patient_id].iter());
                Ok(table)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn modify_series(&self, id: &str, config_file: &str) -> Result<Table> {
        let yaml = fs::read(config_file).unwrap();
        let modification: Modification = serde_yaml::from_slice(&yaml).unwrap();
        match self.client.modify_series(id, modification) {
            Ok(r) => {
                let mut table = create_table(None);
                table.add_row(["New series ID", &r.id].iter());
                table.add_row(["Patient ID", &r.patient_id].iter());
                Ok(table)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn download_series(&self, id: &str, output_file: &str) -> Result<()> {
        let mut file = fs::File::create(output_file).unwrap();
        self.client
            .series_dicom(id, &mut file)
            .map_err(Into::<_>::into)
    }

    pub fn delete_series(&self, id: &str) -> Result<()> {
        match self.client.delete_series(id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    ////////// INSTANCE //////////

    pub fn list_instances(&self, study_id: Option<&str>) -> Result<Table> {
        let mut instance = self.client.instances_expanded()?;
        match study_id {
            Some(p) => instance.retain(|s| s.parent_series == p),
            None => (),
        }

        let mut table = create_table(Some(&INSTANCES_LIST_HEADER));
        for s in instance {
            let mut row: Vec<&str> = vec![&s.id];
            for t in INSTANCES_LIST_DICOM_TAGS.iter() {
                let val = s.main_dicom_tag(t).unwrap_or(ABSENT_DICOM_TAG_PLACEHOLDER);
                row.push(val);
            }
            let index_in_series = match s.index_in_series {
                Some(i) => format!("{}", i),
                None => "".to_string(),
            };
            let file_size = &format!("{}", s.file_size);
            row.push(&index_in_series);
            row.push(&file_size);
            table.add_row(row.iter());
        }
        let id_column = table.get_column_mut(0).unwrap();
        id_column.set_constraint(ColumnConstraint::MinWidth(ID_COLUMN_WIDTH));
        Ok(table)
    }

    pub fn show_instance(&self, instance_id: &str) -> Result<Table> {
        let instance = self.client.instance(instance_id)?;
        let mut table = create_table(None);
        table.add_row(["ID", &instance.id].iter());
        table.add_row(["Series ID", &instance.parent_series].iter());

        for t in INSTANCE_DICOM_TAGS.iter() {
            table.add_row(
                [
                    t,
                    &instance
                        .main_dicom_tag(t)
                        .unwrap_or(ABSENT_DICOM_TAG_PLACEHOLDER),
                ]
                .iter(),
            );
        }
        let index_in_series = match instance.index_in_series {
            Some(i) => format!("{}", i),
            None => "".to_string(),
        };
        let file_size = &format!("{}", instance.file_size);
        table.add_row(["Index in series", &index_in_series].iter());
        table.add_row(["File size", file_size].iter());
        Ok(table)
    }

    pub fn anonymize_instance(
        &self,
        id: &str,
        config_file: Option<&str>,
        path: &str,
    ) -> Result<()> {
        let anonymization = match config_file {
            Some(c) => {
                let yaml = fs::read(c).unwrap();
                Some(serde_yaml::from_slice(&yaml).unwrap())
            }
            None => None,
        };
        let mut file = fs::File::create(path).unwrap();
        self.client
            .anonymize_instance(id, anonymization, &mut file)
            .map_err(Into::<_>::into)
    }

    pub fn modify_instance(&self, id: &str, config_file: &str, path: &str) -> Result<()> {
        let yaml = fs::read(config_file).unwrap();
        let modification: Modification = serde_yaml::from_slice(&yaml).unwrap();
        let mut file = fs::File::create(path).unwrap();
        self.client
            .modify_instance(id, modification, &mut file)
            .map_err(Into::<_>::into)
    }

    pub fn download_instance(&self, id: &str, output_file: &str) -> Result<()> {
        let mut file = fs::File::create(output_file).unwrap();
        self.client
            .instance_dicom(id, &mut file)
            .map_err(Into::<_>::into)
    }

    pub fn delete_instance(&self, id: &str) -> Result<()> {
        match self.client.delete_instance(id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn show_instance_tags(&self, id: &str) -> Result<Table> {
        match self.client.instance_tags_expanded(id) {
            Ok(tags) => {
                let mut table = create_table(None);
                match tags {
                    Value::Object(map) => {
                        for (k, v) in map.iter() {
                            match v {
                                Value::Object(map1) => match map1.get("Value").unwrap() {
                                    // Only one-level (String) values are supported
                                    Value::String(value) => {
                                        match map1.get("Name").unwrap() {
                                            Value::String(name) => {
                                                table.add_row(vec![k, name, value].iter())
                                            }
                                            _ => &table,
                                        };
                                    }
                                    _ => (),
                                },
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
                Ok(table)
            }
            Err(e) => Err(e.into()),
        }
    }

    ////////// MODALITY //////////

    pub fn do_store(&self, modality: &str, ids: &Vec<&str>) -> Result<Table> {
        match self.client.store(modality, ids) {
            Ok(r) => {
                let mut table = create_table(None);
                table.add_row(["Remote AET", &r.remote_aet].iter());
                table.add_row(["Instances sent", &format!("{}", r.instances_count)].iter());
                table
                    .add_row(["Instances failed", &format!("{}", r.failed_instances_count)].iter());
                Ok(table)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn list_modalities(&self) -> Result<Table> {
        let modalities = self.client.modalities_expanded()?;

        let mut table = create_table(Some(&MODALITIES_LIST_HEADER));
        for (m_name, m_config) in modalities {
            let row = vec![
                m_name,
                m_config.aet,
                m_config.host,
                format!("{}", m_config.port),
                m_config.manufacturer.unwrap(),
            ];
            table.add_row(row.iter());
        }
        Ok(table)
    }

    pub fn show_modality(&self, name: &str) -> Result<Table> {
        for (m_name, m_config) in self.client.modalities_expanded()? {
            if m_name == name {
                let mut table = create_table(None);
                table.add_row(vec!["Name", &m_name].iter());
                table.add_row(vec!["AET", &m_config.aet].iter());
                table.add_row(vec!["Host", &m_config.host].iter());
                table.add_row(vec!["Port", &format!("{}", m_config.port)].iter());
                table.add_row(vec!["Manufacturer", &m_config.manufacturer.unwrap()].iter());
                table.add_row(
                    vec![
                        "Transcoding",
                        &format!("{}", m_config.allow_transcoding.unwrap()),
                    ]
                    .iter(),
                );
                table
                    .add_row(vec!["C-ECHO", &format!("{}", m_config.allow_c_echo.unwrap())].iter());
                table
                    .add_row(vec!["C-FIND", &format!("{}", m_config.allow_c_find.unwrap())].iter());
                table.add_row(vec!["C-GET", &format!("{}", m_config.allow_c_get.unwrap())].iter());
                table
                    .add_row(vec!["C-MOVE", &format!("{}", m_config.allow_c_move.unwrap())].iter());
                table.add_row(
                    vec!["C-STORE", &format!("{}", m_config.allow_c_store.unwrap())].iter(),
                );
                table.add_row(
                    vec!["N-ACTION", &format!("{}", m_config.allow_n_action.unwrap())].iter(),
                );
                table.add_row(
                    vec![
                        "N-EVENT-REPORT",
                        &format!("{}", m_config.allow_n_event_report.unwrap()),
                    ]
                    .iter(),
                );
                return Ok(table);
            }
        }
        return Err(CliError::new(
            &format!("Modality {} not found", name),
            None,
            None,
        ));
    }

    pub fn create_modality(&self, name: &str, aet: &str, host: &str, port: i32) -> Result<()> {
        let config = Modality {
            aet: aet.to_string(),
            host: host.to_string(),
            port,
            manufacturer: None,
            allow_transcoding: None,
            allow_c_echo: None,
            allow_c_find: None,
            allow_c_get: None,
            allow_c_move: None,
            allow_c_store: None,
            allow_n_action: None,
            allow_n_event_report: None,
        };
        self.client
            .create_modality(name, config)
            .map_err(Into::<_>::into)
    }

    pub fn modify_modality(&self, name: &str, aet: &str, host: &str, port: i32) -> Result<()> {
        let config = Modality {
            aet: aet.to_string(),
            host: host.to_string(),
            port,
            manufacturer: None,
            allow_transcoding: None,
            allow_c_echo: None,
            allow_c_find: None,
            allow_c_get: None,
            allow_c_move: None,
            allow_c_store: None,
            allow_n_action: None,
            allow_n_event_report: None,
        };
        self.client
            .modify_modality(name, config)
            .map_err(Into::<_>::into)
    }

    pub fn delete_modality(&self, name: &str) -> Result<()> {
        self.client.delete_modality(name).map_err(Into::<_>::into)
    }
}

fn create_table(header: Option<&[&str]>) -> Table {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.load_preset(TABLE_PRESET);
    match header {
        Some(h) => table.set_header(h.iter()),
        None => &table,
    };
    table
}

fn create_error_table(error: CliError) -> Table {
    let mut table = create_table(None);
    table.add_row(["Error", &error.error].iter());
    match error.message {
        Some(m) => {
            table.add_row(["Message", &m].iter());
        }
        None => (),
    };
    match error.details {
        Some(d) => {
            table.add_row(["Details", &d].iter());
        }
        None => (),
    };
    table
}

pub fn get_server_address(cmd_option: Option<&str>) -> result::Result<String, CliError> {
    match cmd_option {
        Some(s) => Ok(s.to_string()),
        None => match env::var("ORC_ORTHANC_ADDRESS") {
            Ok(s) => Ok(s),
            Err(e) => Err(CliError::new(
                "Command error",
                Some("Neither --server-address nor ORC_ORTHANC_ADDRESS are set"),
                Some(&format!("{}", e)),
            )),
        },
    }
}

pub fn get_username(cmd_option: Option<&str>) -> Option<String> {
    match cmd_option {
        Some(s) => Some(s.to_string()),
        None => match env::var("ORC_ORTHANC_USERNAME") {
            Ok(s) => Some(s),
            Err(_) => None, // TODO: This will hide the error
        },
    }
}

pub fn get_password(cmd_option: Option<&str>) -> Option<String> {
    match cmd_option {
        Some(s) => Some(s.to_string()),
        None => match env::var("ORC_ORTHANC_PASSWORD") {
            Ok(s) => Some(s),
            Err(_) => None, // TODO: This will hide the error
        },
    }
}

pub fn print(table: Table) {
    println!("{}", table);
}

pub fn exit_with_error(error: CliError) {
    let output = create_error_table(error);
    eprintln!("{}", output);
    process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{remove_var, set_var};

    #[test]
    fn test_create_error_table() {
        assert_eq!(
            format!(
                "{}",
                create_error_table(CliError::new("error", Some("message"), Some("details")))
            ),
            " Error     error   \n Message   message \n Details   details "
        );
        assert_eq!(
            format!(
                "{}",
                create_error_table(CliError::new("error", None, Some("details")))
            ),
            " Error     error   \n Details   details "
        );
        assert_eq!(
            format!(
                "{}",
                create_error_table(CliError::new("error", Some("message"), None))
            ),
            " Error     error   \n Message   message "
        );
        assert_eq!(
            format!("{}", create_error_table(CliError::new("error", None, None))),
            " Error   error "
        );
    }
    #[test]
    fn test_get_server() {
        remove_var("ORC_ORTHANC_ADDRESS");
        assert_eq!(get_server_address(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(
            get_server_address(None).unwrap_err(),
            CliError::new(
                "Command error",
                Some("Neither --server-address nor ORC_ORTHANC_ADDRESS are set"),
                Some("environment variable not found"),
            )
        );
        set_var("ORC_ORTHANC_ADDRESS", "bar");
        assert_eq!(get_server_address(None).unwrap(), "bar".to_string());
        assert_eq!(get_server_address(Some("baz")).unwrap(), "baz".to_string())
    }

    #[test]
    fn test_get_username() {
        remove_var("ORC_ORTHANC_USERNAME");
        assert_eq!(get_username(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(get_username(None), None);
        set_var("ORC_ORTHANC_USERNAME", "bar");
        assert_eq!(get_username(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(get_username(None).unwrap(), "bar".to_string());
    }

    #[test]
    fn test_get_password() {
        remove_var("ORC_ORTHANC_PASSWORD");
        assert_eq!(get_password(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(get_password(None), None);
        set_var("ORC_ORTHANC_PASSWORD", "bar");
        assert_eq!(get_password(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(get_password(None).unwrap(), "bar".to_string());
    }
}

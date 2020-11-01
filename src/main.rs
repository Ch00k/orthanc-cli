mod lib;

use clap::{crate_authors, crate_description, crate_version, App, Arg};
use comfy_table::Table;
use lib::*;
use orthanc::Error;
use std::process;

fn print(table: Table) {
    println!("{}", table);
}

pub fn exit_with_api_error(error: Error) {
    let output = create_api_error_table(error);
    eprintln!("{}", output);
    process::exit(1);
}

pub fn exit_with_cli_error(error: OrthancError) {
    let output = create_cli_error_table(error);
    eprintln!("{}", output);
    process::exit(1);
}

fn main() {
    let orthanc = App::new("orthanc")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::new("server")
                .display_order(0)
                .about("Orthanc server address")
                .takes_value(true)
                .short('s')
                .long("server"),
        )
        .arg(
            Arg::new("username")
                .display_order(1)
                .about("Orthanc username")
                .takes_value(true)
                .short('u')
                .long("username"),
        )
        .arg(
            Arg::new("password")
                .display_order(2)
                .about("Orthanc password")
                .takes_value(true)
                .short('p')
                .long("password"),
        )
        .subcommand(
            App::new("patient")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(0)
                .about("Patient-level commands")
                .subcommand(App::new("list").display_order(0).about("List all patients"))
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show patient details")
                        .arg(Arg::new("id").about("Patient ID").required(true)),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize patient")
                        .arg(Arg::new("id").about("Patient ID").required(true))
                        .arg(Arg::new("config").about("Anonymization configuration file")),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify patient")
                        .arg(Arg::new("id").about("Patient ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Modifications configuration file")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download patient")
                        .arg(Arg::new("id").about("Patient ID").required(true))
                        .arg(Arg::new("output").about("Output file path").required(true)),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(4)
                        .about("Delete patient")
                        .arg(Arg::new("id").about("Patient ID").required(true)),
                ),
        )
        .subcommand(
            App::new("study")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(1)
                .about("Study-level commands")
                .subcommand(
                    App::new("list")
                        .display_order(0)
                        .about("List all studies")
                        .arg(
                            Arg::new("patient_id")
                                .takes_value(true)
                                .short('i')
                                .long("patient-id")
                                .about("Show only studies, belonging to specified patient"),
                        ),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show study details")
                        .arg(Arg::new("id").about("Study ID").required(true)),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize study")
                        .arg(Arg::new("id").about("Study ID").required(true))
                        .arg(Arg::new("config").about("Anonymization configuration file")),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify study")
                        .arg(Arg::new("id").about("Study ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download study")
                        .arg(Arg::new("id").about("Study ID").required(true))
                        .arg(Arg::new("output").about("Output file path").required(true)),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(4)
                        .about("Delete study")
                        .arg(Arg::new("id").about("Study ID").required(true)),
                ),
        )
        .subcommand(
            App::new("series")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(2)
                .about("Series-level commands")
                .subcommand(
                    App::new("list")
                        .display_order(0)
                        .about("List all series")
                        .arg(
                            Arg::new("study_id")
                                .takes_value(true)
                                .short('i')
                                .long("study-id")
                                .about("Show only series, belonging to specified study"),
                        ),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show series details")
                        .arg(Arg::new("id").about("Series ID").required(true)),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize series")
                        .arg(Arg::new("id").about("Series ID").required(true))
                        .arg(Arg::new("config").about("Anonymization configuration file")),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify series")
                        .arg(Arg::new("id").about("Series ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download series")
                        .arg(Arg::new("id").about("Series ID").required(true))
                        .arg(Arg::new("output").about("Output file path").required(true)),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(4)
                        .about("Delete series")
                        .arg(Arg::new("id").about("Series ID").required(true)),
                ),
        )
        .subcommand(
            App::new("instance")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(3)
                .about("Instance-level commands")
                .subcommand(
                    App::new("list")
                        .display_order(0)
                        .about("List all instances")
                        .arg(
                            Arg::new("series_id")
                                .takes_value(true)
                                .short('i')
                                .long("series-id")
                                .about("Show only instances, belonging to specified series"),
                        ),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show instance details")
                        .arg(Arg::new("id").about("Instance ID").required(true)),
                )
                .subcommand(
                    App::new("tags")
                        .display_order(1)
                        .about("Show instance tags")
                        .arg(Arg::new("id").about("Instance ID").required(true)),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize instance")
                        .arg(Arg::new("id").about("Instance ID").required(true))
                        .arg(
                            Arg::new("output")
                                .about("File path to save the new instance into")
                                .required(true),
                        )
                        .arg(Arg::new("config").about("Anonymization configuration file")),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify instance")
                        .arg(Arg::new("id").about("Instance ID").required(true))
                        .arg(
                            Arg::new("output")
                                .about("File path to save the new instance into")
                                .required(true),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download instance")
                        .arg(Arg::new("id").about("Instance ID").required(true))
                        .arg(Arg::new("output").about("Output file path").required(true)),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(4)
                        .about("Delete instance")
                        .arg(Arg::new("id").about("Instance ID").required(true)),
                ),
        )
        .subcommand(
            App::new("store")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(3)
                .about("Send entities (patients, studies, series or instances) to a modality")
                .arg(
                    Arg::new("modality")
                        .about("Modality ID (name)")
                        .required(true),
                )
                .arg(
                    Arg::new("ids")
                        .about("Entity IDs")
                        .required(true)
                        .multiple_occurrences(true)
                        .multiple_values(true),
                ),
        )
        .get_matches();

    let o = match Orthanc::new(
        orthanc.value_of("server"),
        orthanc.value_of("username"),
        orthanc.value_of("password"),
    ) {
        Ok(o) => o,
        Err(e) => return exit_with_cli_error(e),
    };

    match orthanc.subcommand() {
        Some(("patient", patient)) => match patient.subcommand() {
            Some(("list", _)) => match o.list_patients() {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("show", show)) => match o.show_patient(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("anonymize", anonymize)) => match o.anonymize_patient(
                anonymize.value_of("id").unwrap(),
                anonymize.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("modify", modify)) => match o.modify_patient(
                modify.value_of("id").unwrap(),
                modify.value_of("config").unwrap(),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("download", download)) => match o.download_patient(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            Some(("delete", delete)) => match o.delete_patient(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            _ => {}
        },
        Some(("study", study)) => match study.subcommand() {
            Some(("list", list)) => match o.list_studies(list.value_of("patient_id")) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("show", show)) => match o.show_study(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("anonymize", anonymize)) => match o.anonymize_study(
                anonymize.value_of("id").unwrap(),
                anonymize.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("modify", modify)) => match o.modify_study(
                modify.value_of("id").unwrap(),
                modify.value_of("config").unwrap(),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("download", download)) => match o.download_study(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            Some(("delete", delete)) => match o.delete_study(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            _ => {}
        },
        Some(("series", series)) => match series.subcommand() {
            Some(("list", list)) => match o.list_series(list.value_of("study_id")) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("show", show)) => match o.show_series(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("anonymize", anonymize)) => match o.anonymize_series(
                anonymize.value_of("id").unwrap(),
                anonymize.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("modify", modify)) => match o.modify_series(
                modify.value_of("id").unwrap(),
                modify.value_of("config").unwrap(),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("download", download)) => match o.download_series(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            Some(("delete", delete)) => match o.delete_series(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            _ => {}
        },
        Some(("instance", instance)) => match instance.subcommand() {
            Some(("list", list)) => match o.list_instances(list.value_of("series_id")) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("show", show)) => match o.show_instance(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("anonymize", anonymize)) => match o.anonymize_instance(
                anonymize.value_of("id").unwrap(),
                anonymize.value_of("config"),
                anonymize.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            Some(("modify", modify)) => match o.modify_instance(
                modify.value_of("id").unwrap(),
                modify.value_of("config").unwrap(),
                modify.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            Some(("tags", tags)) => match o.show_instance_tags(tags.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            },
            Some(("download", download)) => match o.download_instance(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            Some(("delete", delete)) => match o.delete_instance(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_api_error(e),
            },
            _ => {}
        },
        Some(("store", store)) => {
            let ids: Vec<&str> = store.values_of("ids").unwrap().collect();
            match o.do_store(store.value_of("modality").unwrap(), &ids) {
                Ok(t) => print(t),
                Err(e) => exit_with_api_error(e),
            }
        }
        _ => {}
    }
}

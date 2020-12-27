use cli::*;
use orthanc_cli::*;
use utils::*;

fn main() {
    let matches = build_cli().get_matches();

    let mut server_address = "".to_string();
    match get_server_address(matches.value_of("server")) {
        Ok(s) => server_address = s,
        Err(e) => exit_with_error(e),
    };
    let o = match Orthanc::new(
        server_address,
        get_username(matches.value_of("username")),
        get_password(matches.value_of("password")),
    ) {
        Ok(o) => o,
        Err(e) => return exit_with_error(e),
    };

    match matches.subcommand() {
        Some(("patient", patient)) => match patient.subcommand() {
            Some(("list", _)) => match o.list_patients() {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_patient(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("anonymize", anonymize)) => {
                let mut keep_private_tags = None;
                if anonymize.is_present("keep_private_tags") {
                    keep_private_tags = Some(true);
                }
                match o.anonymize_patient(
                    anonymize.value_of("id").unwrap(),
                    anonymize.values_of("replace").map(|r| r.collect()),
                    anonymize.values_of("keep").map(|k| k.collect()),
                    keep_private_tags,
                    anonymize.value_of("config"),
                ) {
                    Ok(t) => print(t),
                    Err(e) => exit_with_error(e),
                }
            }
            Some(("modify", modify)) => match o.modify_patient(
                modify.value_of("id").unwrap(),
                modify.values_of("replace").map(|r| r.collect()),
                modify.values_of("remove").map(|r| r.collect()),
                modify.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("download", download)) => match o.download_patient(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("delete", delete)) => match o.delete_patient(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        Some(("study", study)) => match study.subcommand() {
            Some(("list", _)) => match o.list_studies() {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_study(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("anonymize", anonymize)) => {
                let mut keep_private_tags = None;
                if anonymize.is_present("keep_private_tags") {
                    keep_private_tags = Some(true);
                }
                match o.anonymize_study(
                    anonymize.value_of("id").unwrap(),
                    anonymize.values_of("replace").map(|r| r.collect()),
                    anonymize.values_of("keep").map(|k| k.collect()),
                    keep_private_tags,
                    anonymize.value_of("config"),
                ) {
                    Ok(t) => print(t),
                    Err(e) => exit_with_error(e),
                }
            }
            Some(("modify", modify)) => match o.modify_study(
                modify.value_of("id").unwrap(),
                modify.values_of("replace").map(|r| r.collect()),
                modify.values_of("remove").map(|r| r.collect()),
                modify.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("download", download)) => match o.download_study(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("delete", delete)) => match o.delete_study(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        Some(("series", series)) => match series.subcommand() {
            Some(("list", _)) => match o.list_series() {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_series(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("anonymize", anonymize)) => {
                let mut keep_private_tags = None;
                if anonymize.is_present("keep_private_tags") {
                    keep_private_tags = Some(true);
                }
                match o.anonymize_series(
                    anonymize.value_of("id").unwrap(),
                    anonymize.values_of("replace").map(|r| r.collect()),
                    anonymize.values_of("keep").map(|k| k.collect()),
                    keep_private_tags,
                    anonymize.value_of("config"),
                ) {
                    Ok(t) => print(t),
                    Err(e) => exit_with_error(e),
                }
            }
            Some(("modify", modify)) => match o.modify_series(
                modify.value_of("id").unwrap(),
                modify.values_of("replace").map(|r| r.collect()),
                modify.values_of("remove").map(|r| r.collect()),
                modify.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("download", download)) => match o.download_series(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("delete", delete)) => match o.delete_series(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        Some(("instance", instance)) => match instance.subcommand() {
            Some(("list", _)) => match o.list_instances() {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_instance(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("anonymize", anonymize)) => {
                let mut keep_private_tags = None;
                if anonymize.is_present("keep_private_tags") {
                    keep_private_tags = Some(true);
                }
                match o.anonymize_instance(
                    anonymize.value_of("id").unwrap(),
                    anonymize.values_of("replace").map(|r| r.collect()),
                    anonymize.values_of("keep").map(|k| k.collect()),
                    keep_private_tags,
                    anonymize.value_of("config"),
                    anonymize.value_of("output").unwrap(),
                ) {
                    Ok(_) => (),
                    Err(e) => exit_with_error(e),
                }
            }
            Some(("modify", modify)) => match o.modify_instance(
                modify.value_of("id").unwrap(),
                modify.values_of("replace").map(|r| r.collect()),
                modify.values_of("remove").map(|r| r.collect()),
                modify.value_of("config"),
                modify.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("tags", tags)) => match o.show_instance_tags(tags.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("download", download)) => match o.download_instance(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("delete", delete)) => match o.delete_instance(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        Some(("modality", modality)) => match modality.subcommand() {
            Some(("list", _)) => match o.list_modalities() {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_modality(show.value_of("name").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("create", create)) => match o.create_modality(
                create.value_of("name").unwrap(),
                create.value_of("aet").unwrap(),
                create.value_of("host").unwrap(),
                create.value_of("port").unwrap().parse::<i32>().unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("modify", modify)) => match o.modify_modality(
                modify.value_of("name").unwrap(),
                modify.value_of("aet").unwrap(),
                modify.value_of("host").unwrap(),
                modify.value_of("port").unwrap().parse::<i32>().unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("store", store)) => {
                let ids: Vec<&str> = store.values_of("ids").unwrap().collect();
                match o.do_store(store.value_of("name").unwrap(), &ids) {
                    Ok(t) => print(t),
                    Err(e) => exit_with_error(e),
                }
            }
            Some(("delete", delete)) => match o.delete_modality(delete.value_of("name").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        _ => {}
    }
}

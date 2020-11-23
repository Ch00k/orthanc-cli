use clap::{crate_authors, crate_description, crate_version, App, Arg};

pub fn build_cli() -> App<'static> {
    App::new("orthanc")
        .bin_name("orthanc")
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
                .long("server")
                .value_name("SERVER"),
        )
        .arg(
            Arg::new("username")
                .display_order(1)
                .about("Orthanc username")
                .takes_value(true)
                .short('u')
                .long("username")
                .value_name("USERNAME"),
        )
        .arg(
            Arg::new("password")
                .display_order(2)
                .about("Orthanc password")
                .takes_value(true)
                .short('p')
                .long("password")
                .value_name("PASSWORD"),
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
                        .arg(
                            Arg::new("id")
                                .about("Patient ID")
                                .required(true)
                                .value_name("ID"),
                        ),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize patient")
                        .arg(
                            Arg::new("id")
                                .about("Patient ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Anonymization configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .value_name("CONFIG"),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify patient")
                        .arg(
                            Arg::new("id")
                                .about("Patient ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .required(true)
                                .value_name("CONFIG"),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download patient")
                        .arg(
                            Arg::new("id")
                                .about("Patient ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true)
                                .value_name("OUTPUT"),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(5)
                        .about("Delete patient")
                        .arg(
                            Arg::new("id")
                                .about("Patient ID")
                                .required(true)
                                .value_name("ID"),
                        ),
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
                                .about("Show only studies, belonging to specified patient")
                                .value_name("ID"),
                        ),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show study details")
                        .arg(
                            Arg::new("id")
                                .about("Study ID")
                                .required(true)
                                .value_name("ID"),
                        ),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize study")
                        .arg(
                            Arg::new("id")
                                .about("Study ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Anonymization configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .value_name("CONFIG"),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify study")
                        .arg(
                            Arg::new("id")
                                .about("Study ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .required(true)
                                .value_name("CONFIG"),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download study")
                        .arg(Arg::new("id").about("Study ID").required(true))
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true)
                                .value_name("OUTPUT"),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(5)
                        .about("Delete study")
                        .arg(
                            Arg::new("id")
                                .about("Study ID")
                                .required(true)
                                .value_name("ID"),
                        ),
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
                        .arg(
                            Arg::new("id")
                                .about("Series ID")
                                .required(true)
                                .value_name("ID"),
                        ),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize series")
                        .arg(
                            Arg::new("id")
                                .about("Series ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Anonymization configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .value_name("CONFIG"),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify series")
                        .arg(
                            Arg::new("id")
                                .about("Series ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .required(true)
                                .value_name("CONFIG"),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download series")
                        .arg(
                            Arg::new("id")
                                .about("Series ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true)
                                .value_name("OUTPUT"),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(5)
                        .about("Delete series")
                        .arg(
                            Arg::new("id")
                                .about("Series ID")
                                .required(true)
                                .value_name("ID"),
                        ),
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
                        .arg(
                            Arg::new("id")
                                .about("Instance ID")
                                .required(true)
                                .value_name("ID"),
                        ),
                )
                .subcommand(
                    App::new("tags")
                        .display_order(2)
                        .about("Show instance tags")
                        .arg(
                            Arg::new("id")
                                .about("Instance ID")
                                .required(true)
                                .value_name("ID"),
                        ),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(3)
                        .about("Anonymize instance")
                        .arg(
                            Arg::new("id")
                                .about("Instance ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Anonymization configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .value_name("CONFIG"),
                        )
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true)
                                .value_name("OUTPUT"),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(4)
                        .about("Modify instance")
                        .arg(
                            Arg::new("id")
                                .about("Instance ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .required(true)
                                .value_name("CONFIG"),
                        )
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true)
                                .value_name("OUTPUT"),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(5)
                        .about("Download instance")
                        .arg(
                            Arg::new("id")
                                .about("Instance ID")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true)
                                .value_name("OUTPUT"),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(6)
                        .about("Delete instance")
                        .arg(
                            Arg::new("id")
                                .about("Instance ID")
                                .required(true)
                                .value_name("ID"),
                        ),
                ),
        )
        .subcommand(
            App::new("modality")
                .display_order(3)
                .about("Modality-level commands")
                .subcommand(
                    App::new("list")
                        .display_order(0)
                        .about("List all modalities"),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show modality details")
                        .arg(
                            Arg::new("name")
                                .about("Modality name")
                                .required(true)
                                .value_name("NAME"),
                        ),
                )
                .subcommand(
                    App::new("create")
                        .display_order(2)
                        .about("Create a modality")
                        .arg(
                            Arg::new("name")
                                .about("Modality name")
                                .required(true)
                                .value_name("NAME"),
                        )
                        .arg(
                            Arg::new("aet")
                                .about("Modality AET")
                                .takes_value(true)
                                .short('a')
                                .long("aet")
                                .required(true)
                                .value_name("AET"),
                        )
                        .arg(
                            Arg::new("host")
                                .about("Modality host")
                                .takes_value(true)
                                .short('h')
                                .long("host")
                                .required(true)
                                .value_name("HOST"),
                        )
                        .arg(
                            Arg::new("port")
                                .about("Modality port")
                                .takes_value(true)
                                .short('p')
                                .long("port")
                                .required(true)
                                .value_name("PORT"),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify a modality")
                        .arg(
                            Arg::new("name")
                                .about("Modality name")
                                .required(true)
                                .value_name("ID"),
                        )
                        .arg(
                            Arg::new("aet")
                                .about("Modality AET")
                                .takes_value(true)
                                .short('a')
                                .long("aet")
                                .required(true)
                                .value_name("AET"),
                        )
                        .arg(
                            Arg::new("host")
                                .about("Modality host")
                                .takes_value(true)
                                .short('h')
                                .long("host")
                                .required(true)
                                .value_name("HOST"),
                        )
                        .arg(
                            Arg::new("port")
                                .about("Modality port")
                                .takes_value(true)
                                .short('p')
                                .long("port")
                                .required(true)
                                .value_name("PORT"),
                        ),
                )
                .subcommand(
                    App::new("echo")
                        .display_order(4)
                        .about("Send a C-ECHO request to a modality")
                        .arg(
                            Arg::new("modality")
                                .about("Modality name")
                                .required(true)
                                .value_name("NAME"),
                        ),
                )
                .subcommand(
                    App::new("store")
                        .display_order(5)
                        .about("Send a C-STORE request to a modality")
                        .arg(
                            Arg::new("name")
                                .about("Modality name")
                                .required(true)
                                .value_name("NAME"),
                        )
                        .arg(
                            Arg::new("ids")
                                .about("Entity IDs")
                                .takes_value(true)
                                .short('e')
                                .long("entity-ids")
                                .required(true)
                                .multiple_occurrences(true)
                                .multiple_values(true)
                                .value_name("IDS"),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(6)
                        .about("Delete modality")
                        .arg(
                            Arg::new("name")
                                .about("Modality name")
                                .required(true)
                                .value_name("NAME"),
                        ),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use difference::{Changeset, Difference};
    use regex::RegexBuilder;
    use std::io::Write;

    #[test]
    fn test_help() {
        let mut w = Vec::new();
        let mut app = build_cli();

        write!(&mut w, "========== {} ==========\n", app.get_name()).unwrap();
        app.write_help(&mut w).unwrap();
        write!(&mut w, "\n").unwrap();

        for sc in app.get_subcommands_mut() {
            write!(&mut w, "========== {} ==========\n", sc.get_name()).unwrap();
            sc.write_help(&mut w).unwrap();
            write!(&mut w, "\n").unwrap();

            for nested_sc in sc.get_subcommands_mut() {
                write!(&mut w, "========== {} ==========\n", nested_sc.get_name()).unwrap();
                nested_sc.write_help(&mut w).unwrap();
                write!(&mut w, "\n").unwrap();
            }
        }

        let help_str = String::from_utf8(w).unwrap();
        let no_trailing_whitespace = RegexBuilder::new(r"([ ]+$)")
            .multi_line(true)
            .build()
            .unwrap()
            .replace_all(&help_str, "");

        let Changeset { diffs, .. } = Changeset::new(
            &no_trailing_whitespace.trim(),
            include_str!("../tests/data/all_help.stdout").trim(),
            "\n",
        );

        let mut failed = false;

        for i in 0..diffs.len() {
            match diffs[i] {
                Difference::Same(_) => {}
                Difference::Add(ref x) => {
                    println!("+{}", x);
                    failed = true;
                }
                Difference::Rem(ref x) => {
                    println!("-{}", x);
                    failed = true;
                }
            }
        }

        if failed {
            panic!("test failed");
        }
    }
}

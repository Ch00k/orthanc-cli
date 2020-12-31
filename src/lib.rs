use comfy_table::Table;
use constants::*;
use orthanc::client::Client;
use orthanc::entity::*;
use orthanc::error::Error;
use orthanc::models::Modality;
use serde_json::Value;
use serde_yaml;
use std::{fs, io, result};
use utils::*;

pub mod cli;
mod constants;
pub mod utils;

pub type Result<T> = result::Result<T, CliError>;

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

impl From<serde_yaml::Error> for CliError {
    fn from(e: serde_yaml::Error) -> Self {
        CliError::new(&e.to_string(), None, None)
    }
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        CliError::new(&e.to_string(), None, None)
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
        Ok(utils::create_list_table(
            self.client.patients_expanded()?,
            &PATIENTS_LIST_HEADER,
            &PATIENTS_LIST_DICOM_TAGS,
        ))
    }

    pub fn show_patient(&self, patient_id: &str) -> Result<Table> {
        Ok(create_show_table(
            self.client.patient(patient_id)?,
            &PATIENT_DICOM_TAGS,
        ))
    }

    pub fn anonymize_patient(
        &self,
        id: &str,
        replace: Option<Vec<&str>>,
        keep: Option<Vec<&str>>,
        keep_private_tags: Option<bool>,
        config_file: Option<&str>,
    ) -> Result<Table> {
        match self.client.anonymize_patient(
            id,
            get_anonymization_config(replace, keep, keep_private_tags, config_file)?,
        ) {
            Ok(r) => Ok(create_new_entity_table(r)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn modify_patient(
        &self,
        id: &str,
        replace: Option<Vec<&str>>,
        remove: Option<Vec<&str>>,
        config_file: Option<&str>,
    ) -> Result<Table> {
        match self
            .client
            .modify_patient(id, get_modification_config(replace, remove, config_file)?)
        {
            Ok(r) => Ok(create_new_entity_table(r)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn download_patient(&self, id: &str, output_file: &str) -> Result<()> {
        let mut file = fs::File::create(output_file)?;
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

    pub fn list_studies(&self) -> Result<Table> {
        Ok(create_list_table(
            self.client.studies_expanded()?,
            &STUDIES_LIST_HEADER,
            &STUDIES_LIST_DICOM_TAGS,
        ))
    }

    pub fn show_study(&self, study_id: &str) -> Result<Table> {
        Ok(create_show_table(
            self.client.study(study_id)?,
            &STUDY_DICOM_TAGS,
        ))
    }

    pub fn anonymize_study(
        &self,
        id: &str,
        replace: Option<Vec<&str>>,
        keep: Option<Vec<&str>>,
        keep_private_tags: Option<bool>,
        config_file: Option<&str>,
    ) -> Result<Table> {
        match self.client.anonymize_study(
            id,
            get_anonymization_config(replace, keep, keep_private_tags, config_file)?,
        ) {
            Ok(r) => Ok(create_new_entity_table(r)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn modify_study(
        &self,
        id: &str,
        replace: Option<Vec<&str>>,
        remove: Option<Vec<&str>>,
        config_file: Option<&str>,
    ) -> Result<Table> {
        match self
            .client
            .modify_study(id, get_modification_config(replace, remove, config_file)?)
        {
            Ok(r) => Ok(create_new_entity_table(r)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn download_study(&self, id: &str, output_file: &str) -> Result<()> {
        let mut file = fs::File::create(output_file)?;
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

    pub fn list_series(&self) -> Result<Table> {
        Ok(create_list_table(
            self.client.series_expanded()?,
            &SERIES_LIST_HEADER,
            &SERIES_LIST_DICOM_TAGS,
        ))
    }

    pub fn show_series(&self, series_id: &str) -> Result<Table> {
        Ok(create_show_table(
            self.client.series(series_id)?,
            &SERIES_DICOM_TAGS,
        ))
    }

    pub fn anonymize_series(
        &self,
        id: &str,
        replace: Option<Vec<&str>>,
        keep: Option<Vec<&str>>,
        keep_private_tags: Option<bool>,
        config_file: Option<&str>,
    ) -> Result<Table> {
        match self.client.anonymize_series(
            id,
            get_anonymization_config(replace, keep, keep_private_tags, config_file)?,
        ) {
            Ok(r) => Ok(create_new_entity_table(r)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn modify_series(
        &self,
        id: &str,
        replace: Option<Vec<&str>>,
        remove: Option<Vec<&str>>,
        config_file: Option<&str>,
    ) -> Result<Table> {
        match self
            .client
            .modify_series(id, get_modification_config(replace, remove, config_file)?)
        {
            Ok(r) => Ok(create_new_entity_table(r)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn download_series(&self, id: &str, output_file: &str) -> Result<()> {
        let mut file = fs::File::create(output_file)?;
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

    pub fn list_instances(&self) -> Result<Table> {
        Ok(create_list_table(
            self.client.instances_expanded()?,
            &INSTANCES_LIST_HEADER,
            &INSTANCES_LIST_DICOM_TAGS,
        ))
    }

    pub fn show_instance(&self, instance_id: &str) -> Result<Table> {
        Ok(create_show_table(
            self.client.instance(instance_id)?,
            &INSTANCE_DICOM_TAGS,
        ))
    }

    pub fn anonymize_instance(
        &self,
        id: &str,
        replace: Option<Vec<&str>>,
        keep: Option<Vec<&str>>,
        keep_private_tags: Option<bool>,
        config_file: Option<&str>,
        path: &str,
    ) -> Result<()> {
        let mut file = fs::File::create(path)?;
        self.client
            .anonymize_instance(
                id,
                get_anonymization_config(replace, keep, keep_private_tags, config_file)?,
                &mut file,
            )
            .map_err(Into::<_>::into)
    }

    pub fn modify_instance(
        &self,
        id: &str,
        replace: Option<Vec<&str>>,
        remove: Option<Vec<&str>>,
        config_file: Option<&str>,
        path: &str,
    ) -> Result<()> {
        let mut file = fs::File::create(path)?;
        self.client
            .modify_instance(
                id,
                get_modification_config(replace, remove, config_file)?,
                &mut file,
            )
            .map_err(Into::<_>::into)
    }

    pub fn download_instance(&self, id: &str, output_file: &str) -> Result<()> {
        let mut file = fs::File::create(output_file)?;
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
                if let Some(_) = m_config.allow_transcoding {
                    table.add_row(
                        vec![
                            "Transcoding",
                            &format!("{}", m_config.allow_transcoding.unwrap()),
                        ]
                        .iter(),
                    );
                };
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

    pub fn search_patients(&self, query: Vec<&str>) -> Result<Table> {
        let patients: Vec<Patient> = self.client.search(parse_tag_kv_pairs(query)?)?;
        Ok(utils::create_list_table(
            patients,
            &PATIENTS_LIST_HEADER,
            &PATIENTS_LIST_DICOM_TAGS,
        ))
    }

    pub fn search_studies(&self, query: Vec<&str>) -> Result<Table> {
        let studies: Vec<Study> = self.client.search(parse_tag_kv_pairs(query)?)?;
        Ok(utils::create_list_table(
            studies,
            &STUDIES_LIST_HEADER,
            &STUDIES_LIST_DICOM_TAGS,
        ))
    }

    pub fn search_series(&self, query: Vec<&str>) -> Result<Table> {
        let series: Vec<Series> = self.client.search(parse_tag_kv_pairs(query)?)?;
        Ok(utils::create_list_table(
            series,
            &SERIES_LIST_HEADER,
            &SERIES_LIST_DICOM_TAGS,
        ))
    }

    pub fn search_instances(&self, query: Vec<&str>) -> Result<Table> {
        let instances: Vec<Instance> = self.client.search(parse_tag_kv_pairs(query)?)?;
        Ok(utils::create_list_table(
            instances,
            &INSTANCES_LIST_HEADER,
            &INSTANCES_LIST_DICOM_TAGS,
        ))
    }
}

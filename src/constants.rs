pub const TABLE_PRESET: &str = "     --            ";
pub const ID_COLUMN_WIDTH: u16 = 46;
pub const ABSENT_DICOM_TAG_PLACEHOLDER: &str = "undefined";

pub const PATIENTS_LIST_HEADER: [&str; 4] = ["ID", "PatientID", "PatientName", "Number of Studies"];
pub const PATIENTS_LIST_DICOM_TAGS: [&str; 2] = ["PatientID", "PatientName"];
pub const PATIENT_DICOM_TAGS: [&str; 4] =
    ["PatientID", "PatientName", "PatientSex", "PatientBirthDate"];

pub const STUDIES_LIST_HEADER: [&str; 8] = [
    "ID",
    "PatientID",
    "AccessionNumber",
    "StudyInstanceUID",
    "StudyDescription",
    "StudyDate",
    "StudyTime",
    "Number of Series",
];
pub const STUDIES_LIST_DICOM_TAGS: [&str; 6] = [
    "PatientID",
    "AccessionNumber",
    "StudyInstanceUID",
    "StudyDescription",
    "StudyDate",
    "StudyTime",
];
pub const STUDY_DICOM_TAGS: [&str; 7] = [
    "PatientID",
    "StudyID",
    "AccessionNumber",
    "StudyInstanceUID",
    "StudyDescription",
    "StudyDate",
    "StudyTime",
];

pub const SERIES_LIST_HEADER: [&str; 6] = [
    "ID",
    "SeriesInstanceUID",
    "SeriesDescription",
    "Modality",
    "BodyPartExamined",
    "Number of Instances",
];
pub const SERIES_LIST_DICOM_TAGS: [&str; 4] = [
    "SeriesInstanceUID",
    "SeriesDescription",
    "Modality",
    "BodyPartExamined",
];
pub const SERIES_DICOM_TAGS: [&str; 5] = [
    "SeriesInstanceUID",
    "SeriesNumber",
    "SeriesDescription",
    "Modality",
    "BodyPartExamined",
];

pub const INSTANCES_LIST_HEADER: [&str; 7] = [
    "ID",
    "SOPInstanceUID",
    "InstanceNumber",
    "InstanceCreationDate",
    "InstanceCreationTime",
    "Index in series",
    "File size",
];
pub const INSTANCES_LIST_DICOM_TAGS: [&str; 4] = [
    "SOPInstanceUID",
    "InstanceNumber",
    "InstanceCreationDate",
    "InstanceCreationTime",
];
pub const INSTANCE_DICOM_TAGS: [&str; 4] = [
    "SOPInstanceUID",
    "InstanceNumber",
    "InstanceCreationDate",
    "InstanceCreationTime",
];

pub const MODALITIES_LIST_HEADER: [&str; 5] = ["Name", "AET", "Host", "Port", "Manufacturer"];

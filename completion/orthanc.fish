complete -c orthanc -n "__fish_use_subcommand" -s s -l server -d 'Orthanc server address' -r
complete -c orthanc -n "__fish_use_subcommand" -s u -l username -d 'Orthanc username' -r
complete -c orthanc -n "__fish_use_subcommand" -s p -l password -d 'Orthanc password' -r
complete -c orthanc -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_use_subcommand" -f -a "patient" -d 'Patient-level commands'
complete -c orthanc -n "__fish_use_subcommand" -f -a "study" -d 'Study-level commands'
complete -c orthanc -n "__fish_use_subcommand" -f -a "series" -d 'Series-level commands'
complete -c orthanc -n "__fish_use_subcommand" -f -a "instance" -d 'Instance-level commands'
complete -c orthanc -n "__fish_use_subcommand" -f -a "modality" -d 'Modality-level commands'
complete -c orthanc -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -f -a "list" -d 'List all patients'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -f -a "show" -d 'Show patient details'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -f -a "list-studies" -d 'List all studies of a patient'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -f -a "search" -d 'Search for patients'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -f -a "anonymize" -d 'Anonymize patient'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -f -a "modify" -d 'Modify patient'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -f -a "download" -d 'Download patient'
complete -c orthanc -n "__fish_seen_subcommand_from patient" -f -a "delete" -d 'Delete patient'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID PatientName' -r
complete -c orthanc -n "__fish_seen_subcommand_from list" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -d 'Patient ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from show" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from list-studies" -d 'Patient ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from list-studies" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID AccessionNumber StudyDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from list-studies" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from list-studies" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from list-studies" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s q -l query -d 'Search query terms. Space-separted pairs TagName=TagValue. Wildcards are allowed. Example: PatientSex=F PatientName=*Sanchez*' -r
complete -c orthanc -n "__fish_seen_subcommand_from search" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID PatientName' -r
complete -c orthanc -n "__fish_seen_subcommand_from search" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -d 'Patient ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s r -l replace -d 'DICOM tags that should be replaced with the values specified. Space-separted pairs TagName=TagValue. Example: PatientName=REMOVED AccessionNumber=42' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s k -l keep -d 'DICOM tags that should be kept intact. Space-separated tag names. Example: PatientSex PatientBirthDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s c -l config -d 'Anonymization configuration file' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s p -l keep-private-tags -d 'Keep private tags intact'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -d 'Patient ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s r -l replace -d 'DICOM tags that should be replaced with the values specified. Space-separted pairs TagName=TagValue. Example: PatientName=REMOVED AccessionNumber=42' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s m -l remove -d 'DICOM tags that should be removed. Space-separated tag names. Example: PatientSex PatientBirthDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s c -l config -d 'Modification configuration file' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from download" -d 'Patient ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from download" -s o -l output -d 'Output file path' -r
complete -c orthanc -n "__fish_seen_subcommand_from download" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from download" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -d 'Patient ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from study" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from study" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from study" -f -a "list" -d 'List all studies'
complete -c orthanc -n "__fish_seen_subcommand_from study" -f -a "show" -d 'Show study details'
complete -c orthanc -n "__fish_seen_subcommand_from study" -f -a "list-series" -d 'List all series of a study'
complete -c orthanc -n "__fish_seen_subcommand_from study" -f -a "search" -d 'Search for studies'
complete -c orthanc -n "__fish_seen_subcommand_from study" -f -a "anonymize" -d 'Anonymize study'
complete -c orthanc -n "__fish_seen_subcommand_from study" -f -a "modify" -d 'Modify study'
complete -c orthanc -n "__fish_seen_subcommand_from study" -f -a "download" -d 'Download study'
complete -c orthanc -n "__fish_seen_subcommand_from study" -f -a "delete" -d 'Delete study'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID AccessionNumber StudyDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from list" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -d 'Study ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from show" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from list-series" -d 'Study ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from list-series" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID Modality BodyPartExamined' -r
complete -c orthanc -n "__fish_seen_subcommand_from list-series" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from list-series" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from list-series" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s q -l query -d 'Search query terms. Space-separted pairs TagName=TagValue. Wildcards are allowed. Example: StudyDescription=*BRAIN* StudyDate=20200101' -r
complete -c orthanc -n "__fish_seen_subcommand_from search" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID AccessionNumber StudyDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from search" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -d 'Study ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s r -l replace -d 'DICOM tags that should be replaced with the values specified. Space-separted pairs TagName=TagValue. Example: PatientName=REMOVED AccessionNumber=42' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s k -l keep -d 'DICOM tags that should be kept intact. Space-separated tag names. Example: PatientSex PatientBirthDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s c -l config -d 'Anonymization configuration file' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s p -l keep-private-tags -d 'Keep private tags intact'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -d 'Study ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s r -l replace -d 'DICOM tags that should be replaced with the values specified. Space-separted pairs TagName=TagValue. Example: PatientName=REMOVED AccessionNumber=42' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s m -l remove -d 'DICOM tags that should be removed. Space-separated tag names. Example: PatientSex PatientBirthDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s c -l config -d 'Modification configuration file' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from download" -s o -l output -d 'Output file path' -r
complete -c orthanc -n "__fish_seen_subcommand_from download" -d 'Study ID'
complete -c orthanc -n "__fish_seen_subcommand_from download" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from download" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -d 'Study ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from series" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from series" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from series" -f -a "list" -d 'List all series'
complete -c orthanc -n "__fish_seen_subcommand_from series" -f -a "show" -d 'Show series details'
complete -c orthanc -n "__fish_seen_subcommand_from series" -f -a "list-instances" -d 'List all instances of a series'
complete -c orthanc -n "__fish_seen_subcommand_from series" -f -a "search" -d 'Search for series'
complete -c orthanc -n "__fish_seen_subcommand_from series" -f -a "anonymize" -d 'Anonymize series'
complete -c orthanc -n "__fish_seen_subcommand_from series" -f -a "modify" -d 'Modify series'
complete -c orthanc -n "__fish_seen_subcommand_from series" -f -a "download" -d 'Download series'
complete -c orthanc -n "__fish_seen_subcommand_from series" -f -a "delete" -d 'Delete series'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID Modality BodyPartExamined' -r
complete -c orthanc -n "__fish_seen_subcommand_from list" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -d 'Series ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from show" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from list-instances" -d 'Series ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from list-instances" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID InstanceCreationDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from list-instances" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from list-instances" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from list-instances" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s q -l query -d 'Search query terms. Space-separted pairs TagName=TagValue. Wildcards are allowed. Example: SeriesDescription=*BRAIN* SeriesDate=20200101' -r
complete -c orthanc -n "__fish_seen_subcommand_from search" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID Modality BodyPartExamined' -r
complete -c orthanc -n "__fish_seen_subcommand_from search" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -d 'Series ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s r -l replace -d 'DICOM tags that should be replaced with the values specified. Space-separted pairs TagName=TagValue. Example: PatientName=REMOVED AccessionNumber=42' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s k -l keep -d 'DICOM tags that should be kept intact. Space-separated tag names. Example: PatientSex PatientBirthDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s c -l config -d 'Anonymization configuration file' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s p -l keep-private-tags -d 'Keep private tags intact'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -d 'Series ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s r -l replace -d 'DICOM tags that should be replaced with the values specified. Space-separted pairs TagName=TagValue. Example: PatientName=REMOVED AccessionNumber=42' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s m -l remove -d 'DICOM tags that should be removed. Space-separated tag names. Example: PatientSex PatientBirthDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s c -l config -d 'Modification configuration file' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from download" -d 'Series ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from download" -s o -l output -d 'Output file path' -r
complete -c orthanc -n "__fish_seen_subcommand_from download" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from download" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -d 'Series ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -f -a "list" -d 'List all instances'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -f -a "show" -d 'Show instance details'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -f -a "tags" -d 'Show instance tags'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -f -a "search" -d 'Search for instances'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -f -a "anonymize" -d 'Anonymize instance'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -f -a "modify" -d 'Modify instance'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -f -a "download" -d 'Download instance'
complete -c orthanc -n "__fish_seen_subcommand_from instance" -f -a "delete" -d 'Delete instance'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID InstanceCreationDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from list" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -d 'Instance ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from show" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from tags" -d 'Instance ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from tags" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from tags" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s q -l query -d 'Search query terms. Space-separted pairs TagName=TagValue. Wildcards are allowed. Example: InstanceNumber=42 InstanceCreationTime=174242' -r
complete -c orthanc -n "__fish_seen_subcommand_from search" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: ID InstanceCreationDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from search" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from search" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -d 'Instance ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s r -l replace -d 'DICOM tags that should be replaced with the values specified. Space-separted pairs TagName=TagValue. Example: PatientName=REMOVED AccessionNumber=42' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s k -l keep -d 'DICOM tags that should be kept intact. Space-separated tag names. Example: PatientSex PatientBirthDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s c -l config -d 'Anonymization configuration file' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s o -l output -d 'Output file path' -r
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s p -l keep-private-tags -d 'Keep private tags intact'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from anonymize" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -d 'Instance ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s r -l replace -d 'DICOM tags that should be replaced with the values specified. Space-separted pairs TagName=TagValue. Example: PatientName=REMOVED AccessionNumber=42' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s m -l remove -d 'DICOM tags that should be removed. Space-separated tag names. Example: PatientSex PatientBirthDate' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s c -l config -d 'Modification configuration file' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s o -l output -d 'Output file path' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from download" -d 'Instance ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from download" -s o -l output -d 'Output file path' -r
complete -c orthanc -n "__fish_seen_subcommand_from download" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from download" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -d 'Instance ID' -r
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -f -a "list" -d 'List all modalities'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -f -a "show" -d 'Show modality details'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -f -a "create" -d 'Create a modality'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -f -a "modify" -d 'Modify a modality'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -f -a "echo" -d 'Send a C-ECHO request to a modality'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -f -a "store" -d 'Send a C-STORE request to a modality'
complete -c orthanc -n "__fish_seen_subcommand_from modality" -f -a "delete" -d 'Delete modality'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s c -l columns -d 'Display only the columns specified. Space-separated values. Example: Name Manufacturer' -r
complete -c orthanc -n "__fish_seen_subcommand_from list" -s n -l no-header -d 'Don\'t display table header'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from list" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -d 'Modality name' -r
complete -c orthanc -n "__fish_seen_subcommand_from show" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from show" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from create" -d 'Modality name' -r
complete -c orthanc -n "__fish_seen_subcommand_from create" -s a -l aet -d 'Modality AET' -r
complete -c orthanc -n "__fish_seen_subcommand_from create" -s h -l host -d 'Modality host' -r
complete -c orthanc -n "__fish_seen_subcommand_from create" -s p -l port -d 'Modality port' -r
complete -c orthanc -n "__fish_seen_subcommand_from create" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from create" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -d 'Modality name' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s a -l aet -d 'Modality AET' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s h -l host -d 'Modality host' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s p -l port -d 'Modality port' -r
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from modify" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from echo" -d 'Modality name' -r
complete -c orthanc -n "__fish_seen_subcommand_from echo" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from echo" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from store" -d 'Modality name' -r
complete -c orthanc -n "__fish_seen_subcommand_from store" -s e -l entity-ids -d 'Entity IDs' -r
complete -c orthanc -n "__fish_seen_subcommand_from store" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from store" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -d 'Modality name' -r
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from delete" -s V -l version -d 'Prints version information'
complete -c orthanc -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c orthanc -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'

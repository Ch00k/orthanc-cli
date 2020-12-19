[![crate](https://img.shields.io/crates/v/orthanc-cli.svg)](https://crates.io/crates/orthanc-cli)
[![doc](https://docs.rs/orthanc-cli/badge.svg)](https://docs.rs/orthanc-cli)
[![test](https://github.com/Ch00k/orthanc-cli/workflows/tests/badge.svg)](https://github.com/Ch00k/orthanc-cli/actions)
[![codecov](https://codecov.io/gh/Ch00k/orthanc-cli/branch/master/graphs/badge.svg)](https://codecov.io/github/Ch00k/orthanc-cli)
[![license](https://img.shields.io/crates/l/orthanc.svg)](./UNLICENSE)

# orthanc-cli

**orthanc-cli** is a command-line interface for
[Orthanc](https://book.orthanc-server.com/users/rest.html), an open-source, lightweight
DICOM server.

<!--toc-start-->
* [Compatibility](#compatibility)
* [Installation](#installation)
* [Configuration](#configuration)
  * [Orthanc server address](#orthanc-server-address)
  * [Orthanc server authentication](#orthanc-server-authentication)
* [Usage](#usage)
  * [Help](#help)
  * [Entities and identifiers](#entities-and-identifiers)
  * [Anonymizing and modifying entities](#anonymizing-and-modifying-entities)
    * [Anonymization](#anonymization)
    * [Modification](#modification)
<!--toc-end-->

## Compatibility

orthanc-cli usually supports the same Orthanc versions as its underlying
[orthanc-rs](https://crates.io/crates/orthanc) crate. See
[Compatibility](https://github.com/Ch00k/orthanc-rs#compatibility) for details.

## Installation

There are multuple different ways to install orthanc-cli.

* using [cargo](https://doc.rust-lang.org/cargo):

  ```
  $ cargo install orthanc-cli
  ```

* manually downloading a release package from Github Releases
  [page](https://github.com/Ch00k/orthanc-cli/releases)

## Configuration

orthanc-cli needs several settings configured in order to communicate with an Orthanc
server: Orthanc server address, and (optionally) username and password.

### Orthanc server address

Orthanc server address can be set with `-s/--server` command-line option. The value of the
option is a HTTP(S) URL, e.g. `http://127.0.0.1:8042`. Alternatively, if you prefer to not
type the option every time you call a command, you can set an environment variable
`ORC_ORTHANC_SERVER`

```
$ export ORC_ORTHANC_SERVER=http://127.0.0.1:8042
```

### Orthanc server authentication

If the Orthanc server you are working with requires authentication, you can provide it
with command-line options `-u/--username` and `-p/--password`. Similar to the server
address these can also be set as environment variables `ORC_ORTHANC_USERNAME` and
`ORC_ORTHANC_PASSWORD`:

```
$ export ORC_ORTHANC_USERNAME=orthanc
$ export ORC_ORTHANC_PASSWORD=orthanc
```

## Usage

### Help

To get a general idea of the usage run `orthanc help`. This will present the list of
options, flags, and subcommands:

```
$ orthanc help
orthanc 0.1.0
Andrii Yurchuk <ay@mntw.re>
Command-line interface for Orthanc, an open-source, lightweight DICOM server

USAGE:
    orthanc [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --server <SERVER>        Orthanc server address
    -u, --username <USERNAME>    Orthanc username
    -p, --password <PASSWORD>    Orthanc password

SUBCOMMANDS:
    patient     Patient-level commands
    study       Study-level commands
    series      Series-level commands
    instance    Instance-level commands
    modality    Modality-level commands
    help        Prints this message or the help of the given subcommand(s)
```

Each subommand in its turn has its own has its own help:

```
$ orthanc study --help
orthanc-study
Study-level commands

USAGE:
    orthanc study <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    list         List all studies
    show         Show study details
    anonymize    Anonymize study
    modify       Modify study
    download     Download study
    delete       Delete study
    help         Prints this message or the help of the given subcommand(s)
```

Subcommands might have nested subcommands, which also respond to `--help`, and so on. If
unsure, append `--help` to the command to see how to use it.

### Entities and identifiers

orthanc-cli makes a convention of calling patients, studies, series and instances
_entities_ (not to be confused with
[Application Entities](http://otpedia.com/entryDetails.cfm?id=137)). You might come across
this naming in documentation or names command-line options or flags.

Similarly to Orthanc web interface orthanc-cli operates mainly with unique identifiers
when it comes to referring to entities (patients, studies, series etc.). Each entity is
assigned a unique identifier by the Orthanc server. In the list of entities (e.g. studies)
the identifiers are in the first column (ID):

```
$ orthanc study list
 ID                                             PatientID   AccessionNumber   StudyInstanceUID              StudyDescription   StudyDate   StudyTime   Number of Series
------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 ab7a6e26-18072a37-5f2a2210-8a7f0823-f2fa9119   patient_2   REMOVED           1.3.46.670589.11.1.5.0.6560   Study 1            20110101    140606      2
 cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6   patient_1   REMOVED           1.3.46.670589.11.1.5.0.7116   Study 1            20120101    130431      2
 8c69229f-eba0eccb-2aa35808-e26bf10a-69375f79   patient_1   REMOVED           1.3.46.670589.11.3540642177   Study 2            20110101    084707      2
```

When you need to refer to an entity in any of the orthanc-cli commands use its Orthanc
unique identifier:

```
$ orthanc study show cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6
 ID                 cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6
 Patient ID         8be8a583-193f48d2-d9b8dd53-adc11459-e46c7c27
 PatientID          patient_1
 StudyID            402411870
 AccessionNumber    REMOVED
 StudyInstanceUID   1.3.46.670589.11.1.5.0.7116.2012100313043060185
 StudyDescription   Study 1
 StudyDate          20120101
 StudyTime          130431
 Number of Series   2
```

### Anonymizing and modifying entities

orthanc-cli allows modification and anonymization of entities. Both actions require
creating a configuration file describing how exactly the entity should be anonymized or
modified. The format of the configuration file is explained in the next sections.

Note that both anonymization and modification create a copy of the entity that is being
anonymized/modified instead of changing the entity in-place.

#### Anonymization

Anonymization of an entity can be done with or without configuration. If done without
configuration, anonymization deletes or erases DICOM tags according to [Application Level
Confidentiality Profile
Attributes](http://dicom.nema.org/medical/dicom/2017c/output/html/part15.html#table_E.1-1):

```
$ orthanc study anonymize cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6
 New study ID   bb8802bf-fa9621bd-e43406cf-707a3cfb-7786ec34
 Patient ID     6cf95a77-4112b9d3-905c17f0-d48ee8e1-b9e6d482
```

The configuration file must be in [YAML](https://yaml.org) format and may contain the
following fields:

* `replace` - the values of specified DICOM tags will be replaced with those specified
* `keep` - the values of the specified DICOM will be left intact even if it they are
  specified as to be removed in the table mentioned above
* `keep_private_tags` - whether or not to keep the values of private DICOM tags (defaults
  to `false` if omitted)

Example:

```yaml
replace:
  PatientName: Rick Sanchez
  PatientBirthDate: 19700101
keep:
  - StudyDate
  - StudyTime
keep_private_tags: true
```

The usage of such a configuration file is as follows:

```
$ orthanc study anonymize cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6 -c /tmp/anonymization_conf.yml
 New study ID   22fc5ba2-650a6ef5-76f78251-af82a47f-87ce33f4
 Patient ID     8d8454ca-3c70d505-3d4ddced-792feac4-7c992741
```

#### Modification

A configuration file is required for modification. The configuration file must be in
[YAML](https://yaml.org) format and may contain the following fields:

* `replace` - the values of specified DICOM tags will be replaced with those specified
* `remove` - the specified DICOM tags will be removed

Example:

```yaml
replace:
  PatientName: Rick Sanchez
  PatientBirthDate: 19700101
remove:
  - StudyDate
  - StudyTime
```

The usage of such a configuration file is as follows:

```
$ orthanc study modify cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6 -c /tmp/modification_conf.yml
 New study ID   db0a9bc8-7b0362ca-f361c32b-ba62bfd2-44ff849b
 Patient ID     8be8a583-193f48d2-d9b8dd53-adc11459-e46c7c27
```

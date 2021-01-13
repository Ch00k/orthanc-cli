[![crate](https://img.shields.io/crates/v/orthanc-cli.svg)](https://crates.io/crates/orthanc-cli)
[![test](https://github.com/Ch00k/orthanc-cli/workflows/tests/badge.svg)](https://github.com/Ch00k/orthanc-cli/actions)
[![codecov](https://codecov.io/gh/Ch00k/orthanc-cli/branch/master/graphs/badge.svg)](https://codecov.io/github/Ch00k/orthanc-cli)
[![license](https://img.shields.io/crates/l/orthanc.svg)](./UNLICENSE)

# orthanc-cli

**orthanc-cli** is a command-line interface for
[Orthanc](https://www.orthanc-server.com), an open-source, lightweight [DICOM](https://en.wikipedia.org/wiki/DICOM)
server.

<!--toc-start-->
* [Compatibility](#compatibility)
* [Installation](#installation)
  * [Completions](#completions)
* [Configuration](#configuration)
  * [Orthanc server address](#orthanc-server-address)
  * [Orthanc server authentication](#orthanc-server-authentication)
* [Usage](#usage)
  * [Help](#help)
  * [Entities and their IDs](#entities-and-their-ids)
  * [Search](#search)
  * [Anonymizing and modifying Entities](#anonymizing-and-modifying-entities)
    * [Anonymization](#anonymization)
    * [Modification](#modification)
<!--toc-end-->

## Compatibility

_orthanc-cli_ usually supports the same Orthanc versions as its underlying [orthanc-rs](https://crates.io/crates/orthanc)
crate. See [Compatibility](https://github.com/Ch00k/orthanc-rs#compatibility) for details.

## Installation

There are multuple different ways to install _orthanc-cli_.

* using [cargo](https://doc.rust-lang.org/cargo):

  ```
  $ cargo install orthanc-cli
  ```

* manually downloading a release package from Github Releases [page](https://github.com/Ch00k/orthanc-cli/releases)

### Completions

_orthanc-cli_ comes with pre-built completion files for Bash, fish and Zsh. See [here](./completion/README.md) for
details on how to use those files.

## Configuration

_orthanc-cli_ needs several settings configured in order to communicate with an Orthanc server: Orthanc server address,
and username and password (in case the server requires authentication).

### Orthanc server address

Orthanc server address can be set with `-s/--server` command-line option. The value of the option is an HTTP(S) URL,
e.g. `http://127.0.0.1:8042`. Alternatively, if you prefer to not type the option every time you call a command, you can
set an environment variable `ORC_ORTHANC_SERVER`

```
$ export ORC_ORTHANC_SERVER=http://127.0.0.1:8042
```

### Orthanc server authentication

If the Orthanc server you are working with requires authentication, you can provide it with command-line options
`-u/--username` and `-p/--password`. Similar to the server address these can also be set as environment variables
`ORC_ORTHANC_USERNAME` and `ORC_ORTHANC_PASSWORD`:

```
$ export ORC_ORTHANC_USERNAME=orthanc
$ export ORC_ORTHANC_PASSWORD=orthanc
```

## Usage

### Help

To get a general idea of the usage run `orthanc --help`. This will present the list of options, flags, and subcommands:

```
$ orthanc --help
orthanc-cli 0.3.0
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
    search       Search for studies
    anonymize    Anonymize study
    modify       Modify study
    download     Download study
    delete       Delete study
    help         Prints this message or the help of the given subcommand(s)
```

Subcommands might have nested subcommands, which also respond to `--help`. If unsure, append `--help` to the command to
see how to use it.

### Entities and their IDs

_orthanc-cli_ makes a convention of calling Patients, Studies, Series and Instances _Entities_ (not to be confused with
[Application Entities](http://otpedia.com/entryDetails.cfm?id=137)). You might come across this naming in documentation
or names of command-line options or flags.

Similarly to Orthanc web interface _orthanc-cli_ operates mainly with unique identifiers (IDs) when it comes to referring
to _Entities_ (Patients, Studies, Series etc.). Each Entity is assigned a unique identifier (ID) by the Orthanc server,
that looks similar to this:

```
22c54cb6-28302a69-3ff454a3-676b98f4-b84cd80a
```

In the list of Studies for example the identifiers are in the first column (ID):

```
$ orthanc study list
 ID                                             PatientID   AccessionNumber   StudyInstanceUID              StudyDescription   StudyDate   StudyTime   Number of Series
------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 ab7a6e26-18072a37-5f2a2210-8a7f0823-f2fa9119   patient_2   REMOVED           1.3.46.670589.11.1.5.0.6560   Study 1            20110101    140606      2
 cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6   patient_1   REMOVED           1.3.46.670589.11.1.5.0.7116   Study 1            20120101    130431      2
 8c69229f-eba0eccb-2aa35808-e26bf10a-69375f79   patient_1   REMOVED           1.3.46.670589.11.3540642177   Study 2            20110101    084707      2
```

When you need to refer to an Entity in any of the _orthanc-cli_ commands use its Orthanc ID:

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

### Search

_orthanc-cli_ allows searching for entities withing the Orthanc server. You can search for patients, studies, series and
instances with `orthanc <ENTITY> search --query <QUERY>`. Each of the commands will return a list of entities you search
for, e.g. `orthanc patient search` will return a list of patients, `orthanc study search` - a list of studies etc.

The value of the `--query` command-line option are space-separted pairs of DICOM tags: `TagName=TagValue`. For examle:

```
$ orthanc series search --query BodyPartExamined=PINKY
 ID                                             SeriesInstanceUID                SeriesDescription   Modality   BodyPartExamined   Number of Instances
-------------------------------------------------------------------------------------------------------------------------------------------------------
 33209de2-5b2e7753-9537bc4d-4bd166f6-fb48d303   1.2.276.0.7230010.3.1.3.816750   Series 1            MR         PINKY              1
 dab1ca97-70f554a9-c8e83dec-17216f2c-88148c44   1.2.276.0.7230010.3.1.3.816746   Series 1            MR         PINKY              1
```

Wildcards are allowed in values of some DICOM tags. More info on that
[here](http://dicom.nema.org/medical/dicom/2019e/output/chtml/part04/sect_C.2.2.2.4.html).

An example of wildcard usage in `StudyDescription`:

```
$ orthanc study search --query AccessionNumber=REMOVED StudyDescription=*1
 ID                                             PatientID   AccessionNumber   StudyInstanceUID                 StudyDescription   StudyDate   StudyTime   Number of Series
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 342f1834-e4658a76-2f7f8dd6-5f4034dd-eee91323   patient_1   REMOVED           1.2.276.0.7230010.3.1.2.816848   Study 1            20110101    140606      2
 92be942a-744ab613-d5ea8167-5b11a0c9-670f0b10   patient_1   REMOVED           1.2.276.0.7230010.3.1.2.816853   Study 1            20110101    140606      2
 ab7a6e26-18072a37-5f2a2210-8a7f0823-f2fa9119   patient_2   REMOVED           1.3.46.670589.11.1.5.0.6560.20   Study 1            20110101    140606      4
```

### Anonymizing and modifying Entities

_orthanc-cli_ allows modification and anonymization of entities.
Modification requires you to specify how exactly an entity should be modified, while anonymization does not. For both
anonymization and modification you can configure the process with either command-line options or a configuration file.

Note that both anonymization and modification create a copy of the entity that is being anonymized/modified instead of
changing the entity in-place.

#### Anonymization

Anonymization of an entity can be done with or without configuration. If done without configuration, anonymization
treats DICOM tags according to [Application Level Confidentiality Profile Attributes](http://dicom.nema.org/medical/dicom/2017c/output/html/part15.html#table_E.1-1):

```
$ orthanc study anonymize cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6
 New study ID   bb8802bf-fa9621bd-e43406cf-707a3cfb-7786ec34
 Patient ID     6cf95a77-4112b9d3-905c17f0-d48ee8e1-b9e6d482
```

To change the way particular DICOM tags are treated during anonymization you can use command-line options:

* `--replace`: the values of specified DICOM tags will be replaced with those specified
* `--keep`: the values of the specified DICOM tags will be left intact (even if they are specified as to be removed in
the table mentioned above)
* `--keep-private-tags`: whether or not to keep the values of private DICOM tags (if omitted private DICOM tags are
removed)

The above command-line options are used as follows:

```
$ orthanc study anonymize cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6 --replace PatientName="Rick Sanchez" PatientBirthDate=19700101 --keep StudyDate StudyTime --keep-private-tags
 New study ID   72b2983e-0196e005-7102f94f-4bf2161c-18d33b59
 Patient ID     1209a543-256b97d2-639bebf1-c3c076e7-0b4b8a3f
```

If you intend to give special treatment to more than a couple of DICOM tags, writing them all on the command line can
become inconvenient. For this purpose you can use an anonymization configuration file instead. The configuration file
must be in [YAML](https://yaml.org) format and may contain the following fields (see above for their meaning):

* `replace`
* `keep`
* `keep_private_tags`

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
$ orthanc study anonymize cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6 --config /tmp/anonymization_conf.yml
 New study ID   22fc5ba2-650a6ef5-76f78251-af82a47f-87ce33f4
 Patient ID     8d8454ca-3c70d505-3d4ddced-792feac4-7c992741
```

#### Modification

In order to modify an entity you are required to specify how exactly it should me modified. This can be done with the
following command-line options:

* `--replace`: the values of specified DICOM tags will be replaced with those specified
* `--remove`: the specified DICOM tags will be removed

The above command-line options are used as follows:

```
$ orthanc study modify cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6 --replace PatientName="Rick Sanchez" PatientBirthDate=19700101 --remove StudyDate StudyTime
 New study ID   24510c21-3b10e0ac-268f7570-b8c01c22-77e19a41
 Patient ID     b64615f0-5cac7527-68e751f7-c22d822c-e4ff1e1d
```

Similar to the process of anonymization you can use a configuration file for modification in case you need to modify
more than a hadful of DICOM tags. The configuration file must be in [YAML](https://yaml.org) format and may contain the
following fields:

* `replace`
* `remove`

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
$ orthanc study modify cbec5098-53cd29f5-86d01e4b-c6e76386-709f00a6 --config /tmp/modification_conf.yml
 New study ID   db0a9bc8-7b0362ca-f361c32b-ba62bfd2-44ff849b
 Patient ID     8be8a583-193f48d2-d9b8dd53-adc11459-e46c7c27
```

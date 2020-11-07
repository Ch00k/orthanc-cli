#!/usr/bin/env bash

set -e

# ftp://medical.nema.org/medical/dicom/DataSets/WG16/Philips/
curl https://minuteware.net/orc/test_data.tar.bz2 > /tmp/test_data.tar.bz2
mkdir -p $ORC_DATAFILES_PATH
tar xvjf /tmp/test_data.tar.bz2 -C $ORC_DATAFILES_PATH

curl_command="curl -i -X POST -H 'Expect:'"

if [ -n $ORC_ORTHANC_USERNAME ] && [ -n $ORC_ORTHANC_PASSWORD ]; then
    curl_command="$curl_command --user $ORC_ORTHANC_USERNAME:$ORC_ORTHANC_PASSWORD"
fi

curl_command="$curl_command $ORC_ORTHANC_ADDRESS/instances"

for f in $(find $ORC_DATAFILES_PATH/initial -type f); do
    cmd="$curl_command --data-binary @$f"
    $cmd
done

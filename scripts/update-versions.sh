#!/usr/bin/env bash

set -eo pipefail

if [ -z "$CRATE_ROOT" ] || [ -z "$NEW_VERSION" ]; then
    echo "Missing CRATE_ROOT and/or NEW_VERSION."
    echo "This script needs to run as a 'pre-release-hook' from cargo-release."
    exit 1
fi

FILES=( README.md tests/data/all_help.stdout )
for file in ${FILES[@]}; do
    sed -i -E \
        -e "s|orthanc-cli [0-9.]+|orthanc-cli ${NEW_VERSION}|g" \
        "${CRATE_ROOT}/$file"
done

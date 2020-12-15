SHELL := /usr/bin/env bash

export ORC_ORTHANC_SERVER ?= http://localhost:8028
export ORC_ORTHANC_USERNAME ?= orthanc
export ORC_ORTHANC_PASSWORD ?= orthanc
export ORC_DATAFILES_PATH ?= ./tests/data/dicom

export DINO_SCP_HOST ?= 0.0.0.0
export DINO_SCP_PORT ?= 5252
export DINO_SCP_AET ?= DINO


.PHONY: test clean unit_test integration_test unit_test_coverage integration_test_coverage install_tarpaulin cleanup_orthanc populate_orthanc reset_orthanc start_services stop_services release

test: unit_test integration_test

clean: cleanup_orthanc stop_services
	cargo clean

unit_test:
	cargo test --lib ${TEST}

unit_test_coverage: install_tarpaulin
	cargo tarpaulin --lib --verbose --ignore-tests --all-features --workspace --timeout 120 --out Xml

integration_test: reset_orthanc
	cargo test --test integration -- ${TEST} --test-threads=1 --show-output

integration_test_coverage: install_tarpaulin_HEAD reset_orthanc
	cargo tarpaulin --test integration --follow-exec --verbose --ignore-tests --all-features --workspace --timeout 120 --out Xml -- --test-threads=1

install_tarpaulin:
	cargo install --version 0.16.0 cargo-tarpaulin

install_tarpaulin_HEAD:
	cargo install --git https://github.com/xd009642/tarpaulin.git --branch develop cargo-tarpaulin

cleanup_orthanc:
	./scripts/cleanup_orthanc.sh

populate_orthanc:
	./scripts/populate_orthanc.sh

reset_orthanc: cleanup_orthanc populate_orthanc

start_services:
	docker-compose pull
	docker-compose up -d

stop_services:
	docker-compose down

check_completion:
	mkdir -p /tmp/orc_completion
	ORC_COMPLETION_OUTPUT_DIR=/tmp/orc_completion cargo build
	diff ./completion /tmp/orc_completion

release:
	cargo-release

# Fixture provenance needs stdlib tomllib. Keep an explicit PYTHON override.
ifndef PYTHON
PYTHON := $(shell for candidate in python3 python3.14 python3.13 python3.12 python3.11; do \
	if $$candidate -c 'import tomllib' >/dev/null 2>&1; then echo $$candidate; exit 0; fi; \
	done; echo python3)
endif

SCHEMA_EXPORT ?= target/public-schemas
CARGO_SHEAR_VERSION := 1.14.0
CARGO_SHEAR_ROOT := target/tools/cargo-shear

.PHONY: gate check-values

gate:
	$(PYTHON) scripts/check-reconciliation.py --merged
	$(PYTHON) scripts/check-fixture-provenance.py
	$(PYTHON) -m unittest discover -s scripts -p 'test_*.py'
	cargo fmt --all --check
	cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
	cargo test --workspace --all-features --locked
	$(PYTHON) scripts/check-macro-callers.py
	$(PYTHON) scripts/check-schema-compatibility.py --choices-only --output $(SCHEMA_EXPORT)
	$(MAKE) check-dependencies

.PHONY: check-dependencies
check-dependencies:
	cargo install cargo-shear --version =$(CARGO_SHEAR_VERSION) --locked --root $(CARGO_SHEAR_ROOT)
	$(CARGO_SHEAR_ROOT)/bin/cargo-shear shear --deny-warnings

check-values:
	cargo test --workspace --all-features --locked --test text_literals --test values_author_contracts

.PHONY: check-frozen-operations
check-frozen-operations:
	cargo test --workspace --all-features --locked --test frozen_operations
	$(PYTHON) scripts/check-macro-callers.py

.PHONY: check-object-inputs
check-object-inputs:
	$(PYTHON) scripts/check-object-inputs.py

.PHONY: check-transcript-schemas
check-transcript-schemas:
	$(PYTHON) -m unittest discover -s scripts -p 'test_schema_choices.py'
	cargo test -p biodata --features schema --locked --test schema_contract

.PHONY: check-subject-schema
check-subject-schema:
	$(PYTHON) -m unittest discover -s scripts -p 'test_schema_choices.py'
	cargo test -p biodata --features schema --locked --test subject_schema --test source_shapes

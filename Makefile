PROFILE ?= dev

.PHONY: fmt
fmt: ## run formatting on the project
	cargo fmt --all

.PHONY: test
test: ## run tests on project, usage: make test PROFILE=dev
	cargo test --profile $(PROFILE) --verbose

.PHONY: build
build: ## run build on project, usage: make test PROFILE=dev
	cargo build --profile $(PROFILE) --verbose

.PHONY: clippy
clippy: ## run clippy on project, usage: make clippy
	cargo clippy --all --all-targets -- -D warnings

RUST ?= latest
DOCKER = docker
DOCKER_UID = "$$(id -u)"
DOCKER_GID = "$$(id -g)"
DOCKER_PATH = /usr/src/myapp
RUST_VERSION ?= rust:latest
CARGO = cargo

.PHONY: build release

build:
	cargo build

release:
	$(DOCKER) run --rm --user $(DOCKER_UID):$(DOCKER_GID) \
		-v "$$PWD":$(DOCKER_PATH) \
		-w $(DOCKER_PATH) $(RUST_VERSION) \
		cargo build --release




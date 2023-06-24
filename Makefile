NAME=rpi-gpio-api

TARGET_DIR=target
TARGET_DOC_DIR=$(TARGET_DIR)/openapi
TARGET_LIC_DIR=$(TARGET_DIR)/license
TARGET_3RDPT_NOTICE=$(TARGET_LIC_DIR)/third-party-notices.txt

VERSION?=$(shell sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml)

RUST_CHANNEL?=$(shell sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml)
REDOC_CLI_VERSION?=v1.0.0-beta.128

#
# Build machines
#

define DOCKERFILE_ARMV7
FROM rust:$(RUST_CHANNEL)
RUN apt-get update \
 && apt-get install gcc-arm-linux-gnueabihf -y \
 && rustup target install armv7-unknown-linux-gnueabihf \
 && rustup component add clippy rustfmt \
 && cargo install cargo-deb cargo-license
endef
export DOCKERFILE_ARMV7

#
# Phony Targets
#

.PHONY: package doc license clean test

package: doc license
	@echo "$$DOCKERFILE_ARMV7" | docker build -t $(NAME)-builder:local -
	docker run -v $(CURDIR):/src -w /src --rm $(NAME)-builder:local cargo deb --target=armv7-unknown-linux-gnueabihf

doc: $(TARGET_DOC_DIR)/$(NAME).html

license: $(TARGET_3RDPT_NOTICE)

clean:
	@rm -rf $(TARGET_DIR)

test:
	@echo "$$DOCKERFILE_ARMV7" | docker build -t $(NAME)-builder:local -
	docker run -v $(CURDIR):/src -w /src --rm $(NAME)-builder:local /bin/sh -c "\
		rustc --version && \
		cargo --version && \
		cargo check --verbose && \
		cargo fmt --all -- --check && \
		cargo clippy --all-targets --all-features -- -D warnings"

#
# Actual Targets
#

$(TARGET_3RDPT_NOTICE):
	@mkdir -p $(TARGET_LIC_DIR)
	@echo "$$DOCKERFILE_ARMV7" | docker build -t $(NAME)-builder:local -
	docker run -v $(CURDIR):/src -w /src --rm $(NAME)-builder:local cargo license --avoid-build-deps --avoid-dev-deps --tsv >$(TARGET_3RDPT_NOTICE)

$(TARGET_DOC_DIR)/$(NAME).yaml $(TARGET_DOC_DIR)/$(NAME).html:
	@mkdir -p $(TARGET_DOC_DIR)
	@docker run -v $(CURDIR):/src -w /src --rm redocly/cli:$(REDOC_CLI_VERSION) \
		bundle api/$(NAME)/openapi.yaml \
		-o $(TARGET_DOC_DIR)/$(NAME).yaml
	@docker run -v $(CURDIR):/src -w /src --rm redocly/cli:$(REDOC_CLI_VERSION) \
		build-docs $(TARGET_DOC_DIR)/$(NAME).yaml \
		-o $(TARGET_DOC_DIR)/$(NAME).html \
		-t /usr/local/lib/node_modules/@redocly/cli/src/commands/build-docs/template.hbs

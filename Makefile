BINARY_NAME := riip
INSTALL_DIR := /usr/local/bin

.PHONY: all build install clean

all: build

build:
	cargo build --release

install: build
	@echo "Installing binary to $(INSTALL_DIR)..."
	sudo cp target/release/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Creating alias 'riip' for all users..."
	sudo sh -c "echo 'alias tomb=\"$(INSTALL_DIR)/$(BINARY_NAME)\"' >> /etc/profile"
	@echo "Installed: run 'riip' to use this." 

clean:
	cargo clean
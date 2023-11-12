# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

.PHONE: test
test:
	cargo test

.PHONY: all
all:
	cargo build --target x86_64-unknown-none --profile $(RUST_PROFILE)
	cp target/x86_64-unknown-none/$(RUST_PROFILE_SUBDIR)/turo kernel.elf

.PHONY: clean
clean:
	cargo clean
	rm -rf kernel.elf

.PHONY: distclean
distclean: clean
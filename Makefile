OUTPUT := ./target/x86_64-los/debug/kernel
SYSROOT_DIR := ../sysroot

all: build

build:
	@cargo build

install: all
	@cp $(OUTPUT) $(SYSROOT_DIR)/kernel.elf
	@echo "[ KERNEL ] Installed!"

clean:
	@cargo clean
	@echo "[ KERNEL ] Cleaned!"

.PHONY: all build install


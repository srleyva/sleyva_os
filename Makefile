setup:
	rustup override set nightly
	rustup component add rust-src
	rustup component add llvm-tools-preview
	cargo install bootimage

build:
	cargo build
	cargo bootimage

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-sleyva_os/debug/bootimage-sleyva_os.bin

test:
	cargo test
	cargo test --features recursive_mapped_memory --no-default-features
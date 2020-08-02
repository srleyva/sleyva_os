build:
	cargo build
	cargo bootimage

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-sleyva_os/debug/bootimage-sleyva_os.bin
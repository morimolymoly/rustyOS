build:
	cargo bootimage

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64_bare/debug/bootimage-rusty_os.bin

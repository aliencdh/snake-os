boot: src/boot.asm
	nasm -f bin src/boot.asm -o build/debug/boot.bin
run: boot
	qemu-system-x86_64 build/debug/boot.bin

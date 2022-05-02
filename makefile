all: boot kernel
	cat build/debug/boot.bin build/debug/full_kernel.bin > build/debug/snakeos.bin

boot: src/boot.asm
	nasm -f bin src/boot.asm -o build/debug/boot.bin

kernel: src/kernel_entry.asm src/main.c
	nasm -f elf src/kernel_entry.asm -o build/debug/kernel_entry.o
	i386-elf-gcc -ffreestanding -m32 -g -c src/main.c -o build/debug/kernel.o
	i386-elf-ld -Ttext 0x1000 build/debug/kernel_entry.o build/debug/kernel.o -o build/debug/full_kernel.bin --oformat binary

run: all
	qemu-system-x86_64 build/debug/snakeos.bin

debug: all
	qemu-system-x86_64 build/debug/snakeos.bin -S

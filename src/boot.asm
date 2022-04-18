[org 0x7c00]
KERNEL_LOCATION equ 0x1000

BOOT_DISK: db 0

mov [BOOT_DISK], dl

; clean the registers
xor ax, ax
mov es, ax
mov ds, ax
mov bp, 0x8000
mov sp, bp

mov bx, KERNEL_LOCATION

;;;;;;;;;;;;;;;
; LOAD SECTOR ;
;;;;;;;;;;;;;;;
mov ah, 2 ; memory read operation
mov al, 3 ; the number of sectors to be read

mov ch, 0 ; cylinder number
mov cl, 2 ; sector number
mov dh, 0 ; head number
mov dl, BOOT_DISK ; drive number
mov bx, 0x8000

int 0x13

; error handling
cmp al, 3
jne incorrect_num_sectors

; clear screen
mov ah, 0x0
mov al, 0x3 ; text mode
int 0x10

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; SWITCH TO PROTECTED MODE ;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;
cli ; disable interrupts
lgdt [GDT_Descriptor]
; change last bit of cr0 to 1
mov eax, cr0
or eax, 1
mov cr0, eax

jmp CODE_SEG:start_protected_mode

jmp $

;;;;;;;;;;;;;;;
; DEFINITIONS ;
;;;;;;;;;;;;;;;
incorrect_num_sectors:
    mov ah, 0x0e
    int 0x10
    mov bx, msg
    jmp print_str

msg: db "Incorrect number of sectors were loaded.", 0

print_str:
    mov al, [bx]
    cmp al, 0
    je exit
    int 0x10
    inc bx
    jmp print_str

exit: jmp $

;;;;;;;;;;;;;;;;;;
; GDT DESCRIPTOR ;
;;;;;;;;;;;;;;;;;;
GDT_Start:
    null_descriptor:
        dd 0
        dd 0
    code_descriptor:
        ; first 16 bits of limit
        dw 0xffff
        ; base
        dw 0 ; 16 bits
        db 0 ; + 8 bits = 24 bits
        ; pres, priv, type = 1001
        ; type flags = 1010
        db 0b10011010
        ; other + last 4 bits of limit
        db 0b11001111
        db 0
    data_descriptor:
        ; first 16 bits of limit
        dw 0xffff
        ; base
        dw 0
        db 0
        ; pres, priv, type + type flags
        db 0b10010010
        db 0b11001111
        db 0
GDT_End:

GDT_Descriptor:
    dw GDT_End - GDT_Start - 1 ; size
    dd GDT_Start

; offsets
CODE_SEG equ code_descriptor - GDT_Start
DATA_SEG equ data_descriptor - GDT_Start

;;;;;;;;;;;;;;;;;;
; 32-BIT SECTION ;
;;;;;;;;;;;;;;;;;;
[bits 32]
start_protected_mode:
    ; set up segment registers and stack
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ebp, 0x90000
    mov esp, ebp

    jmp KERNEL_LOCATION ; jump to kernel


; fill with 0s until 510 bytes
times 510-($-$$) db 0
; magic BIOS number
dw 0xaa55

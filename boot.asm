init:
	[org 0x7c00]
	mov [diskNum], dl

setScreenRes:
	mov ah, 0x00
	mov al, 0x03
	int 0x10

protect_init:
	code_seg equ code_desciptor - GDT_Start
	data_seg equ data_descriptor - GDT_Start
	cli
	lgdt [GDT_Descriptor]
	mov eax, cr0
	or eax, 1
	mov cr0, eax
	jmp code_seg:start_protected_mode

GDT_Start:
	null_descriptor:
		dd 0
		dd 0
	code_desciptor:
		dw 0xffff
		dw 0
		db 0
		db 0b10011010
		db 0b11001111
		db 0
	data_descriptor:
		dw 0xffff
		dw 0
		db 0
		db 0b10010010
		db 0b11001111
		db 0
GDT_End:

GDT_Descriptor:
	dw GDT_Start - GDT_End - 1
	dd GDT_Start

[bits 32]
start_protected_mode:
	mov al, 'H'
	mov ah, 0x0f
	mov [0xb8000], ax
	jmp $

diskNum: db 0

exit:
	times 510-($-$$) db 0
	dw 0xaa55

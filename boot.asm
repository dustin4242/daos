mov [diskNum], dl

setScreenRes:
	mov ah, 0x00
	mov al, 0x03
	int 0x10

clearDriveReg:
	xor ax, ax                          
	mov es, ax
	mov ds, ax
	mov bp, 0x8000
	mov sp, bp
	mov bx, diskAddr

loadDrive:
	mov ah, 2
	mov al, 1
	mov ch, 0
	mov dh, 0
	mov cl, 2
	mov dl, [diskNum];
	int 0x13


ttymode: 
	[org 0x7c00]
	mov ah, 0x0e

mov bx, diskAddr
printDrive:
	mov al, [bx]
	cmp al, 0
	je exit
	int 0x10
	inc bx
	jmp printDrive

diskNum: db 0
diskAddr: db 0x7e

exit:
	jmp $
	times 510-($-$$) db 0
	dw 0xaa55

db "Hello World", 0

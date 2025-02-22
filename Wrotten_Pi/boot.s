.section .text
.global _boot
_boot:
    MOV r0, #0x8000    // hardcoded kernel address
    BX r0

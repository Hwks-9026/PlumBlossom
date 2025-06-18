# Design Goals
1. Better understand computer architecture by developing an emulator for a simple 32 bit CPU with 32 bit memory addressing, and 32 bit words.
2. Write a custom assembly language and compiler to generate it, and link it into a rom that can be loaded into memory for the CPU to execute.

# Register Layout
em-x32 uses 16 general purpose registers: **r0** through **r15**, and a jump register **J**. These Registers are each 32 bits. The **PC** register tracks the number of instructions since power on, wrapping around. It is also 32 bits. A flag can be set in memory with an address in general RAM at which to increment upon wrap around. The stack pointer, **SP** is a register that points to a 32 bit memory address in the stack RAM. A non-writable register **IP** keeps track of the instruction pointer. 
## Register Hex Words
0x00000000: r0
0x00000001: r1
...
0x0000000F: r15
0x00000010: J
0x00000020: PC
0x00000030: SP
0x00000040: IP

# Instruction Set
- em-x32 has a small instruction set. Instructions are two bytes, followed by up to 15 bytes of additional data. This additional data could be a register label or a 32 bit signed integer, written in decimal or hexidecimal. The first byte of the instruction demarcates how much additional data the CPU should fetch in memory, from zero words to three words. The second byte denotes which instruction the CPU should execute with the additional data.

*some notation: %rX signifies any of the general purpose registers. ex: JZ %r0. If multiple registers are needed for an instruction they will be notated %rA, %rB, etc. Anytime a general purpose register can be used, %J may be used to write to or read from the* **J** *register*

## 0 Extra Words
0x00: NOP           - No operation.
0x01: JMP           - The jump instruction takes program execution to the address that is the **J** register.
0x02: CAL           - Store address of next byte on the stack and execute JMP.
0x03: RET           - Return from subroutine by popping stack and then jumping there.

## 1 Extra Word
0x10: JNZ %rX           - JMP if %rX is not zero.
0x11: JIZ %rX           - JMP if %rX is zero.
0x12: PSH %rX           - Push %rX to the stack.
0x13: POP %rX           - Pop stack and store to %rX.
0x14: NOT %rX           - Invert bits of %rX.
0x15: SHL %rX           - Shift %rX left.
0x16: SHR %rX           - Shift %rX right (logical).

## 2 Extra Words 
0x20: MOV %rA %rB       - writes the value of %rB to %rA.
0x21: MVI %rX imm32     - writes the next word to %rX.
0x22: LDA %rA %rB       - load the word at address %rB into %rA.
0x23: STA %rA %rB       - write %rA to the address stored at %rB.

## 3 Extra Words
0x30: ADD %rA %rX %rY   - Add         %rX and  %rY and store the result in %rA.
0x30: SUB %rA %rX %rY   - Subtract    %rX from %rY and store the result in %rA.
0x30: MUL %rA %rX %rY   - Multiply    %rX and  %rY and store the result in %rA.
0x30: DIV %rA %rX %rY   - Divide      %rX by   %rY and store the result in %rA.
0x30: AND %rA %rX %rY   - Bitwise and %rX and  %rY and store the result in %rA.
0x30: XOR %rA %rX %rY   - Bitwise xor %rX and  %rY and store the result in %rA.

# 32 Bit Memory Map
Memory on the PlumBlossom system is mapped as such:
0x00000000 - 0x0000001F || The first 32 bits contain the reset vector the system will change the instruction pointer to.
0x00000020 - 0x00000027 || The next 8 bits contain important flags. See appendix 1.
0x00000028 - 0x0000003F || Checksum for loaded image file that is calculated by the emulator at start up
0x00000040 - 0x0009603F || Custom black and white display for system. (Two buffers)
0x00096040 - 0x8009603F || 2 Gigabytes of general purpose ram for user programs
0x80096040 - 0xFFFFFFFF || Unspecified readable/writable. Depending on the configuration could be used for a different external device.
             

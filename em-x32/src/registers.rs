#[derive(Default, Debug)]
pub(crate) struct Registers {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    r13: u32,
    r14: u32,
    r15: u32,
    j: u32,
    pc: u32,
    sp: u32,
    ip: u32,
}

#[repr(u32)]
pub(crate) enum Register {
R0 = 0,
R1 = 1,
R2 = 2,
R3 = 3,
R4 = 4,
R5 = 5,
R6 = 6,
R7 = 7,
R8 = 8,
R9 = 9,
R10 = 10,
R11 = 11,
R12 = 12,
R13 = 13,
R14 = 14,
R15 = 15,
J = 16, 
PC = 32,
SP = 48,
IP = 64,
}

impl Registers {
    pub fn new() -> Registers {
        let mut r = Registers::default();
        r.sp = 0x80096040;
        return r;
    }

    pub fn write(&mut self, register: u32, value: u32) {
        match register {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            8 => self.r8 = value,
            9 => self.r9 = value,
            10 => self.r10 = value,
            11 => self.r11 = value,
            12 => self.r12 = value,
            13 => self.r13 = value,
            14 => self.r14 = value,
            15 => self.r15 = value,
            16 => self.j = value,
            32 => self.pc = value,
            48 => self.sp = value,
            64 => self.ip = value,
            _ => {}
        }
    }
    pub fn read(&self, register: u32) -> u32 {
        match register {
            0 => return self.r0,
            1 => return self.r1,
            2 => return self.r2,
            3 => return self.r3,
            4 => return self.r4,
            5 => return self.r5,
            6 => return self.r6,
            7 => return self.r7,
            8 => return self.r8,
            9 => return self.r9,
            10 => return self.r10,
            11 => return self.r11,
            12 => return self.r12,
            13 => return self.r13,
            14 => return self.r14,
            15 => return self.r15,
            16 => return self.j,
            32 => return self.pc,
            48 => return self.sp,
            64 => return self.ip,
            _ => return 0,
        }
    }
}

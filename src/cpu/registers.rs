pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagRegister, // flag register
    pub h: u8,
    pub l: u8,
}
impl Registers {
    // implement af, bc, de, and hl registers for 16 bit manip
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (u8::from(self.f) as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xff00) >> 8) as u8;
        let f_val = (value & 0xf0) as u8; // keep lower bits as 0
        self.f = FlagRegister::from(f_val);
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xff00) >> 8) as u8;
        self.c = (value & 0xff) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xff00) >> 8) as u8;
        self.e = (value & 0xff) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xff00) >> 8) as u8;
        self.l = (value & 0xff) as u8;
    }
}

#[derive(Clone, Copy)]
pub struct FlagRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

pub const ZERO_FLAG_BYTE_POS: u8 = 7;
pub const SUBTRACT_FLAG_BYTE_POS: u8 = 6;
pub const HALF_CARRY_FLAG_BYTE_POS: u8 = 5;
pub const CARRY_FLAG_BYTEPOS: u8 = 4;

impl std::convert::From<FlagRegister> for u8 {
    fn from(flag: FlagRegister) -> u8 {
        ((if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POS) |
            ((if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POS) |
            ((if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POS) |
            ((if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTEPOS)
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POS) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POS) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POS) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTEPOS) & 0b1) != 0;

        FlagRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

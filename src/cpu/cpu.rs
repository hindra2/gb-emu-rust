use super::registers::{ Registers };
use super::instructions::{
    Instruction,
    ArithmaticTarget,
    Jump,
    LoadByteSource,
    LoadByteTarget,
    LoadType,
};
use super::membus::MemoryBus;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
}

// if result is zero, zero flag is true
// if opeartion was a subtraction, subtract flag is true
// if there was carry, carry flag is true
// half carry is only set to true if there is overflow from the lower bits to the upper bits

impl CPU {
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xcb;

        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed {
                    "cb"
                } else {
                    ""
                },
                instruction_byte
            );
            panic!("Unkown instruction found for: {}", description)
        };

        self.pc = next_pc;
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::F => {
                        let value = u8::from(self.registers.f);
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::H => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                }
                self.pc.wrapping_add(1)
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::F => {
                        let value = u8::from(self.registers.f);
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                }
                self.pc.wrapping_add(1)
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    Jump::NotZero => !self.registers.f.zero,
                    Jump::NotCarry => !self.registers.f.carry,
                    Jump::Zero => self.registers.f.zero,
                    Jump::Carry => self.registers.f.carry,
                    Jump::Always => true,
                };
                self.jump(jump_condition)
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                            _ => { panic!("implement others") }
                        };
                        match target {
                            LoadByteTarget::A => {
                                self.registers.a = source_value;
                            }
                            LoadByteTarget::B => {
                                self.registers.b = source_value;
                            }
                            LoadByteTarget::C => {
                                self.registers.c = source_value;
                            }
                            LoadByteTarget::D => {
                                self.registers.d = source_value;
                            }
                            LoadByteTarget::E => {
                                self.registers.e = source_value;
                            }
                            LoadByteTarget::H => {
                                self.registers.h = source_value;
                            }
                            LoadByteTarget::L => {
                                self.registers.l = source_value;
                            }
                            LoadByteTarget::HLI =>
                                self.bus.write_byte(self.registers.get_hl(), source_value),
                        }
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                }
            }
            Instruction::NOP() => { self.pc.wrapping_add(1) }
        }
    }

    pub fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    pub fn read_next_word(&self) -> u16 {
        let lsb = self.bus.read_byte(self.pc + 1) as u16;
        let msb = self.bus.read_byte(self.pc + 2) as u16;
        (msb << 8) | lsb
    }

    pub fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xf) + (value & 0xf) > 0xf;

        new_value
    }

    pub fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = self.registers.a & 0xf < value & 0xf;

        new_value
    }

    pub fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let lsb = self.bus.read_byte(self.pc + 1) as u16;
            let msb = self.bus.read_byte(self.pc + 2) as u16;
            (msb << 8) | lsb
        } else {
            self.pc.wrapping_add(3)
        }
    }

    // stack pointer functions
    pub fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xff00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xff00) as u8);
    }

    pub fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    pub fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    pub fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump { self.pop() } else { self.pc.wrapping_add(1) }
    }
}

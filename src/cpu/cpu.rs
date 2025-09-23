use crate::cpu::instructions;

use super::registers::{ Registers };
use super::instructions::{ Instruction, ArithmaticTarget };
use super::membus::MemoryBus;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
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

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction);
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

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::F => {
                        let value = u8::from(self.registers.f);
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::H => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::F => {
                        let value = u8::from(self.registers.f);
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                    ArithmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1);
                    }
                }
            }
        }
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
}

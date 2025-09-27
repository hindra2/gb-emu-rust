pub enum Instruction {
    ADD(ArithmaticTarget), // A = A + target
    ADDHL(ArithmaticTarget), // A = A + target, hl <- target register
    ADC(ArithmaticTarget), // A = A + target + carry
    SUB(ArithmaticTarget), // A = A - target
    SBC(ArithmaticTarget), // A = A - target - carry
    AND(ArithmaticTarget), // A = A & target
    OR(ArithmaticTarget), // A = A | target
    XOR(ArithmaticTarget), // A = A XOR target
    CP(ArithmaticTarget), // compare (A - target)
    INC(ArithmaticTarget), // target = target + 1 (increment)
    DEC(ArithmaticTarget), // target = target - 1 (decrement)
    CCF(ArithmaticTarget), // toggle carry flag value
    SCF(ArithmaticTarget), // set carry flag to true
    RRA(ArithmaticTarget), // bit rotate A register right through carry flag
    RLA(ArithmaticTarget), // bit rotate A reigsiter left through carry flag
    RRCA(ArithmaticTarget), // bit rotate A regsiter right (not through carry flag)
    RLCA(ArithmaticTarget), // bit rotate A register left (not through carry flag)
    CPL(ArithmaticTarget), // A = A'
    BIT(ArithmaticTarget), // test to see if a specific bit of a specific register is set
    RESET(ArithmaticTarget), // set a specific bit of a specific register to 0
    SET(ArithmaticTarget), // set a specific bit of a specific register to 1
    SRL(ArithmaticTarget), // bit shift a specific register right by 1
    RR(ArithmaticTarget), // bit rotate a specific register right by 1 through the carry flag
    RL(ArithmaticTarget), // bit rotate a specific register left by 1 through the carry flag
    RRC(ArithmaticTarget), // bit rotate a specific register right by 1 (not through the carry flag)
    RLC(ArithmaticTarget), // bit rotate a specific register left by 1 (not through the carry flag)
    SRA(ArithmaticTarget), // arithmatic shift a specific register right by 1
    SLA(ArithmaticTarget), // arithmatic shift a specific register left by 1
    SWAP(ArithmaticTarget), // switch upper and lower nibbles of a specific register
    JP(Jump), // Jump instruction
    LD(LoadType), // Load
    CALL(Jump),
    RET(Jump), // return
    NOP(), // no operation
    HALT(),
}
impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(ArithmaticTarget::B)),
            // other opcodes
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::ADD(ArithmaticTarget::A)),
            // other opcodes
        }
    }
}

pub enum ArithmaticTarget {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum ArithmaticTarget16 {
    AF,
    BC,
    DE,
    HL,
}

pub enum Jump {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

# CPU Implementation Todo

## Basic Arithmetic

-   [x] **ADD(ArithmeticTarget)** - A = A + target
-   [ ] **ADDHL(ArithmeticTarget)** - HL = HL + target register
-   [ ] **ADC(ArithmeticTarget)** - A = A + target + carry
-   [x] **SUB(ArithmeticTarget)** - A = A - target
-   [ ] **SBC(ArithmeticTarget)** - A = A - target - carry

## Logic

-   [ ] **AND(ArithmeticTarget)** - A = A & target
-   [ ] **OR(ArithmeticTarget)** - A = A | target
-   [ ] **XOR(ArithmeticTarget)** - A = A XOR target
-   [ ] **CPL(ArithmeticTarget)** - A = A' (complement A)

## Flag Operations

-   [ ] **CCF(ArithmeticTarget)** - Toggle carry flag value
-   [ ] **SCF(ArithmeticTarget)** - Set carry flag to true

## Rotate (A register)

-   [ ] **RRA(ArithmeticTarget)** - Rotate A register right through carry flag
-   [ ] **RLA(ArithmeticTarget)** - Rotate A register left through carry flag
-   [ ] **RRCA(ArithmeticTarget)** - Rotate A register right (not through carry)
-   [ ] **RLCA(ArithmeticTarget)** - Rotate A register left (not through carry)

## Rotate

-   [ ] **RR(ArithmeticTarget)** - Rotate register right through carry flag
-   [ ] **RL(ArithmeticTarget)** - Rotate register left through carry flag
-   [ ] **RRC(ArithmeticTarget)** - Rotate register right (not through carry)
-   [ ] **RLC(ArithmeticTarget)** - Rotate register left (not through carry)

## Shift

-   [ ] **SRL(ArithmeticTarget)** - Logical shift register right by 1
-   [ ] **SRA(ArithmeticTarget)** - Arithmetic shift register right by 1
-   [ ] **SLA(ArithmeticTarget)** - Arithmetic shift register left by 1

## Bit Manipulation

-   [ ] **BIT(ArithmeticTarget)** - Test if specific bit of register is set
-   [ ] **RESET(ArithmeticTarget)** - Set specific bit of register to 0
-   [ ] **SET(ArithmeticTarget)** - Set specific bit of register to 1
-   [ ] **SWAP(ArithmeticTarget)** - Switch upper and lower nibbles of register

## Others

-   [ ] **CP(ArithmeticTarget)** - Compare (A - target)
-   [ ] **INC(ArithmeticTarget)** - target = target + 1 (increment)
-   [ ] **DEC(ArithmeticTarget)** - target = target - 1 (decrement)

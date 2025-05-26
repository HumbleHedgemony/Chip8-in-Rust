use super::address::Address;
use super::cpuDependencies;
use super::registers::Register;
use super::registers::Registers;
use super::memory::Memory;
use super::stack::Stack;
use super::display::Display;
use super::keyboard::Keyboard;
use super::timers::Timers;
use super::configuration;
use rand::prelude::*;
use std::marker::PhantomData;



pub trait CpuVariant {}



pub trait CpuInstructions {

    fn get_word_from_pc(&mut self) -> u16;
    fn get_byte_from_pc(&mut self) -> u8;
    fn carry_flag(&mut self, isOverflowed:bool);
    fn skip_word_instruction(&mut self);
    fn reverse_word_instruction(&mut self);
    fn get_x_register(&mut self,instruction:u16) -> &mut Register;
    fn get_y_register(&mut self,instruction:u16) -> &mut Register;
    fn get_n(&mut self, instruction:u16) -> u16;
    fn get_nn(&mut self, instruction:u16) -> u16;
    fn get_nnn(&mut self, instruction:u16) -> u16;
    

    fn opcode_0NNN(&self);
    fn opcode_00E0(&mut self);
    fn opcode_00EE(&mut self);
    fn opcode_1NNN(&mut self, instruction:u16);
    fn opcode_2NNN(&mut self, instruction:u16);
    fn opcode_3XNN(&mut self, instruction:u16);
    fn opcode_4XNN(&mut self, instruction:u16);
    fn opcode_5XY0(&mut self, instruction:u16);
    fn opcode_9XY0(&mut self, instruction:u16);
    fn opcode_6XNN(&mut self, instruction:u16);
    fn opcode_7XNN(&mut self, instruction:u16);
    fn opcode_8XY0(&mut self, instruction:u16);
    fn opcode_8XY1(&mut self, instruction:u16);
    fn opcode_8XY2(&mut self, instruction:u16);
    fn opcode_8XY3(&mut self, instruction:u16);
    fn opcode_8XY4(&mut self, instruction:u16);
    fn opcode_8XY5(&mut self, instruction:u16);
    fn opcode_8XY7(&mut self, instruction:u16);
    fn opcode_ANNN(&mut self, instruction:u16);
    fn opcode_CXNN(&mut self, instruction:u16);
    fn opcode_DXYN(&mut self, instruction:u16);
    fn opcode_EX9E(&mut self, instruction:u16);
    fn opcode_EXA1(&mut self, instruction:u16);
    fn opcode_FX07(&mut self, instruction:u16);
    fn opcode_FX15(&mut self, instruction:u16);
    fn opcode_FX18(&mut self, instruction:u16);
    fn opcode_FX1E(&mut self, instruction:u16);
    fn opcode_FX0A(&mut self, instruction:u16);
    fn opcode_FX29(&mut self, instruction:u16);
    fn opcode_FX33(&mut self, instruction:u16);
}

pub trait CpuVariantInstructions: CpuInstructions {
    fn get_cputype(&self) -> String;
    fn opcode_8XY6(&mut self, instruction:u16);
    fn opcode_8XYE(&mut self, instruction:u16);
    fn opcode_BXNN(&mut self, instruction:u16);
    fn opcode_FX55(&mut self, instruction:u16);
    fn opcode_FX65(&mut self, instruction:u16);

}
pub struct Cosmac;
impl CpuVariant for Cosmac {}

pub struct Super;
impl CpuVariant for Super{}

pub enum CpuType {
    Super(crate::core::cpu::Super),
    Cosmac(crate::core::cpu::Cosmac),
}


    pub struct Cpu< V:CpuVariant> {


        pub index:Address,
        pub pc:Address,
        pub stack:Stack,
        pub registers:Registers,
        pub memory:Memory,
        pub display:Display,
        pub keyboard:Keyboard,
        pub timers:Timers,
        cpuVariant:std::marker::PhantomData<V>,

    }


    impl< V: CpuVariant> Cpu< V> {

        pub fn init() -> Cpu< V> {


            Cpu {
                index: Address::init(0),
                pc: Address::init(configuration::MEMORY_STARTING_ADDRESS),
                stack: Stack::init(),
                registers: Registers::init(),
                memory: Memory::init(),
                display: Display::init(),
                keyboard: Keyboard::init(),
                timers: Timers::init(),
                cpuVariant:PhantomData,
            }
        }

        /* 
        #[inline]
        fn tick(&mut self) {
            //fetch
            let instruction:u16 = self.get_word_from_pc();

            // decode and execute
            self.execute(instruction);

        }

        fn execute(&mut self, instruction_raw:u16) {

            let mut instruction:Address = Address{address:instruction_raw};

            match true {
                true if instruction.is_equal(0x00E0) => self.opcode_00E0(),
                true if instruction.is_equal(0x00EE) => self.opcode_00EE(),
                true if instruction.in_range(0x1000,0x1FFF) => self.opcode_1NNN(instruction_raw),
                true if instruction.in_range(0x2000, 0x2FFF) => self.opcode_2NNN(instruction_raw),
                true if instruction.in_range(0x3000,0x3FFF) => self.opcode_3XNN(instruction_raw),
                true if instruction.in_range(0x4000,0x4FFF) => self.opcode_4XNN(instruction_raw),
                true if instruction.in_range(0x5000,0x5FFF) => self.opcode_5XY0(instruction_raw),
                true if instruction.in_range(0x6000,0x6FFF) => self.opcode_6XNN(instruction_raw),
                true if instruction.in_range(0x7000,0x7FFF) => self.opcode_7XNN(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8000 as u16) => self.opcode_8XY0(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8001 as u16) => self.opcode_8XY1(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8002 as u16) => self.opcode_8XY2(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8004 as u16) => self.opcode_8XY4(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8005 as u16) => self.opcode_8XY5(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8006 as u16) => self.opcode_8XY6(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8007 as u16) => self.opcode_8XY7(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x800E as u16) => self.opcode_8XYE(instruction_raw),
                true if instruction.in_range(0x9000, 0x9FFF) => self.opcode_9XY0(instruction_raw),
                true if instruction.in_range(0xA000, 0xAFFF) => self.opcode_ANNN(instruction_raw),
                true if instruction.in_range(0xB000, 0xBFFF) => self.opcode_BXNN(instruction_raw),
                true if instruction.in_range(0xC000, 0xCFFF) => self.opcode_CXNN(instruction_raw),
                true if instruction.in_range(0xD000, 0xDFFF) => self.opcode_DXYN(instruction_raw),
                true if instruction.match_nibbles_HNML(0xE09E as u16) => self.opcode_EX9E(instruction_raw),
                true if instruction.match_nibbles_HNML(0xE0A1 as u16) => self.opcode_EXA1(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF007 as u16) => self.opcode_FX07(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF00A as u16) => self.opcode_FX0A(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF015 as u16) => self.opcode_FX15(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF018 as u16) => self.opcode_FX18(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF01E as u16) => self.opcode_FX1E(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF029 as u16) => self.opcode_FX29(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF033 as u16) => self.opcode_FX33(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF055 as u16) => self.opcode_FX55(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF065 as u16) => self.opcode_FX65(instruction_raw)
            }
        }
        */

        /* 
        fn get_word_from_pc(&mut self) -> u16 {
            let mut word:u16 = (self.get_byte_from_pc() as u16) << 8;
            word = word | (self.get_byte_from_pc() as u16);
            return word;
        }

        fn get_byte_from_pc(&mut self) -> u8 {
            let pc_val:u16 = self.pc.get();
            self.pc.increment();
            let byte = self.memory.read(pc_val);
            return byte;
        }

        pub fn carry_flag(&mut self, isOverflowed:bool) {
            if isOverflowed {
                let mut overflowRegister:&mut Register = self.registers.get(0xf);
                overflowRegister.set(1 as u8);
            }
        }

        fn skip_word_instruction(&mut self) {
            self.pc.increment();
            self.pc.increment();
        }

        fn reverse_word_instruction(&mut self) {
            self.pc.decrement();
            self.pc.decrement();
        }

        fn get_x_register(&mut self,instruction:u16) -> &mut Register {
            let registerIndexX:u8 = ((instruction & 0x0f00) >> 8) as u8;
            return &mut self.registers.get(registerIndexX);
        }

        fn get_y_register(&mut self,instruction:u16) -> &mut Register {
            let registerIndexX:u8 = ((instruction & 0x00f0) >> 4) as u8;
            return &mut self.registers.get(registerIndexX);
        }

        fn get_n(&mut self, instruction:u16) -> u16 {
            let n:u16 = instruction & 0x000f;
            return n;
        }


        fn get_nn(&mut self, instruction:u16) -> u16 {
            let nn:u16 = instruction & 0x00ff;
            return nn;
        }

        fn get_nnn(&mut self, instruction:u16) -> u16 {
            let nnn:u16 = instruction & 0x0fff;
            return nnn;
        }

        */

    }
/*
    pub trait CpuInstructions {


        fn get_word_from_pc(&mut self) -> u16;
        fn get_byte_from_pc(&mut self) -> u8;
        fn carry_flag(&mut self, isOverflowed:bool);
        fn skip_word_instruction(&mut self);
        fn reverse_word_instruction(&mut self);
        fn get_x_register(&mut self,instruction:u16) -> &mut Register;
        fn get_y_register(&mut self,instruction:u16) -> &mut Register;
        fn get_n(&mut self, instruction:u16) -> u16;
        fn get_nn(&mut self, instruction:u16) -> u16;
        fn get_nnn(&mut self, instruction:u16) -> u16;

        fn opcode_0NNN();
        fn opcode_00E0(&mut self);
        fn opcode_00EE(&mut self);
        fn opcode_1NNN(&mut self, instruction:u16);
        fn opcode_2NNN(&mut self, instruction:u16);
        fn opcode_3XNN(&mut self, instruction:u16);
        fn opcode_4XNN(&mut self, instruction:u16);
        fn opcode_5XY0(&mut self, instruction:u16);
        fn opcode_9XY0(&mut self, instruction:u16);
        fn opcode_6XNN(&mut self, instruction:u16);
        fn opcode_7XNN(&mut self, instruction:u16);
        fn opcode_8XY0(&mut self, instruction:u16);
        fn opcode_8XY1(&mut self, instruction:u16);
        fn opcode_8XY2(&mut self, instruction:u16);
        fn opcode_8XY3(&mut self, instruction:u16);
        fn opcode_8XY4(&mut self, instruction:u16);
        fn opcode_8XY5(&mut self, instruction:u16);
        fn opcode_8XY7(&mut self, instruction:u16);
        fn opcode_ANNN(&mut self, instruction:u16);
        fn opcode_CXNN(&mut self, instruction:u16);
        fn opcode_DXYN(&mut self, instruction:u16);
        fn opcode_EX9E(&mut self, instruction:u16);
        fn opcode_EXA1(&mut self, instruction:u16);
        fn opcode_FX07(&mut self, instruction:u16);
        fn opcode_FX15(&mut self, instruction:u16);
        fn opcode_FX18(&mut self, instruction:u16);
        fn opcode_FX1E(&mut self, instruction:u16);
        fn opcode_FX0A(&mut self, instruction:u16);
        fn opcode_FX29(&mut self, instruction:u16);
        fn opcode_FX33(&mut self, instruction:u16);
    }
    */


    impl< V: CpuVariant> CpuInstructions for  Cpu< V> {

        fn get_word_from_pc(&mut self) -> u16 {
            let mut word:u16 = (self.get_byte_from_pc() as u16) << 8;
            word = word | (self.get_byte_from_pc() as u16);
            return word;
        }

        fn get_byte_from_pc(&mut self) -> u8 {
            let pc_val:u16 = self.pc.get();
            self.pc.increment();
            let byte = self.memory.read(pc_val);
            return byte;
        }

        fn carry_flag(&mut self, isOverflowed:bool) {
            let mut overflowRegister:&mut Register = self.registers.get(0xf);
            if isOverflowed {
                let mut overflowRegister:&mut Register = self.registers.get(0xf);
                overflowRegister.set(1 as u8);
            } else {
                overflowRegister.set(0 as u8);
            }
        }

        fn skip_word_instruction(&mut self) {
            self.pc.increment();
            self.pc.increment();
        }

        fn reverse_word_instruction(&mut self) {
            self.pc.decrement();
            self.pc.decrement();
        }

         fn get_x_register(&mut self,instruction:u16) -> &mut Register {
            let registerIndexX:u8 = ((instruction & 0x0f00) >> 8) as u8;
            return self.registers.get(registerIndexX);
        }

         fn get_y_register(&mut self,instruction:u16) -> &mut Register {
            let registerIndexX:u8 = ((instruction & 0x00f0) >> 4) as u8;
            return self.registers.get(registerIndexX);
        }

         fn get_n(&mut self, instruction:u16) -> u16 {
            let n:u16 = instruction & 0x000f;
            return n;
        }


         fn get_nn(&mut self, instruction:u16) -> u16 {
            let nn:u16 = instruction & 0x00ff;
            return nn;
        }

         fn get_nnn(&mut self, instruction:u16) -> u16 {
            let nnn:u16 = instruction & 0x0fff;
            return nnn;
        }
        
        /*
        *
        * @description 
        *  We’ll start out with an instruction that you actually don’t want to implement! 
        *  In the original CHIP-8 interpreters, this would pause execution of the CHIP-8 program 
        *  and call a subroutine written in machine language at address NNN instead.
        *  This routine would be written in the machine language of the computer’s CPU; 
        *  on the original COSMAC VIP and the ETI-660, this was 1802 machine code, 
        *  and on the DREAM 6800, M6800 code. Unless you’re making an emulator for 
        *  either of those computers, skip this one.
        * 
        * @note
        *      i wrote an implementation for this instruction but commented it out
        *      provided it as an example in case programmer wants to see how its
        *      implemented
        *      
        * 
        */
         fn opcode_0NNN(&self) {

        }
        /*
        * Clear Screen
        * 
        * @description
        *  This is pretty simple: It should clear the display, 
        *                         turning all pixels off to 0.
        *  */
         fn opcode_00E0(&mut self) {
            self.display.clear_screen();
        }

        /*
        * subroutine
        * 
        * @description
        * Returning from a subroutine is done with 00EE
        * removing (“popping”) the last address from the stack and setting the PC to it
        */
         fn opcode_00EE(&mut self) {
            let address:u16 = self.stack.pop();
            self.pc.set(address);
        }

        /*
        * jump
        *  
        * @description
        *  This instruction should simply set PC to NNN, 
        *  causing the program to jump to that memory location. 
        *  Do not increment the PC afterwards, it jumps directly there.
        *
        * */
         fn opcode_1NNN(&mut self, instruction:u16) {
            let address:u16 = self.get_nnn(instruction);
            self.pc.set(address);
        }

        /*
        * 
        * 
        * @description
        *  2NNN calls the subroutine at memory location NNN. 
        *  In other words, just like 1NNN, you should set PC to NNN.
        *  However, the difference between a jump and a call is that this
        *  instruction should first push the current PC to the stack, so the
        *  subroutine can return later.
        * 
        *  */
         fn opcode_2NNN(&mut self, instruction:u16) {
            let pc_val:u16 = self.pc.get();
            self.stack.push(pc_val);
            self.opcode_1NNN(instruction);
        }

        /**
         * 
         * Skip conditionally
         * 
         * @description
         *  These instructions do the same thing: 
         *  They either do nothing, or they skip one two-byte instruction 
         *  (increment PC by 2) if some condition is true. 
         *  (If you didn’t increment PC in the “fetch” stage above, they will obviously 
         *  increment PC by either 4 or 2.)
         * 
         *  3XNN will skip one instruction if the value in VX is equal to NN
         * 
         */
         fn opcode_3XNN(&mut self, instruction:u16) {
            let nn:u16 = self.get_nn(instruction);
            let vx:&mut Register = self.get_x_register(instruction);
            
            if vx.is_equal(nn as u8) {
                self.skip_word_instruction();
            }
        }
        /*
        * 
        * Skip conditionally
        * 
        * @description
        *  These instructions do the same thing: 
        *  They either do nothing, or they skip one two-byte instruction 
        *  (increment PC by 2) if some condition is true. 
        *  (If you didn’t increment PC in the “fetch” stage above, they will obviously 
        *  increment PC by either 4 or 2.)
        * 
        *  4XNN will skip if they are not equal.
        * 
        */
         fn opcode_4XNN(&mut self, instruction:u16) {
            let nn:u16 = self.get_nn(instruction);
            let vx:&mut Register = self.get_x_register(instruction);

            if !vx.is_equal(nn as u8) {
                self.skip_word_instruction();
            }
        }

        /*
        * 
        * Skip conditionally
        * 
        * @description
        *  These instructions do the same thing: 
        *  They either do nothing, or they skip one two-byte instruction 
        *  (increment PC by 2) if some condition is true. 
        *  (If you didn’t increment PC in the “fetch” stage above, they will obviously 
        *  increment PC by either 4 or 2.)
        * 
        *  5XY0 skips if the values in VX and VY are equal
        * 
        */
         fn opcode_5XY0(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);
            if vx.is_equal(vy_val) {
                self.skip_word_instruction();
            }
        }

        /*
        * 
        * Skip conditionally
        * 
        * @description
        *  These instructions do the same thing: 
        *  They either do nothing, or they skip one two-byte instruction 
        *  (increment PC by 2) if some condition is true. 
        *  (If you didn’t increment PC in the “fetch” stage above, they will obviously 
        *  increment PC by either 4 or 2.)
        * 
        *  9XY0 skips if the values in VX and VY are not equal
        * 
        */
         fn opcode_9XY0(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);

            if !vx.is_equal(vy_val) {
                self.skip_word_instruction();
            }
        }

        /*
        * set
        * 
        * @description
        * 
        * Simply set the register VX to the value NN.
        * 
        */
         fn opcode_6XNN(&mut self, instruction:u16) {
            let nn:u16 = self.get_nn(instruction);
            let vx:&mut Register = self.get_x_register(instruction);
            
            vx.set(nn as u8);
        }
        /*
        * add
        * 
        * @description
        * 
        * Add the value NN to VX.
        * 
        * Note that on most other systems, and even in some of the
        * other CHIP-8 instructions, this would set the carry flag
        * if the result overflowed 8 bits; in other words, if the result
        * of the addition is over 255.
        * For this instruction, this is not the case. 
        * If V0 contains FF and you execute 7001, the CHIP-8’s 
        * flag register VF is not affected.
        * 
        */
         fn opcode_7XNN(&mut self,instruction:u16) {
            let nn:u16 = self.get_nn(instruction) as u16;
            let vx:&mut Register = self.get_x_register(instruction);

            let value:u16 = vx.get() as u16 + nn;

            vx.set(value as u8);

            self.carry_flag((value > 0xff));
            
        }


        /***********************************/
        /* Logical Arithmetic Instructions */
        /***********************************/

        /*
        * Set
        * 
        * @description
        * VX is set to the value of VY.
        * 
        * 
        * 
        * 
        */
         fn opcode_8XY0(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);

            vx.set(vy_val);
        }

        /*
        *
        * Binary OR
        * VX is set to the bitwise/binary logical disjunction (OR) 
        * of VX and VY. VY is not affected.
        * 
        *  */

         fn opcode_8XY1(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);

            let binaryOr:u8 = vx.get() | vy_val;
            
            vx.set(binaryOr);
        }
        /*
        * Binary AND
        * VX is set to the bitwise/binary logical disjunction (AND) 
        * of VX and VY. VY is not affected.
        * 
        *  */
         fn opcode_8XY2(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);

            let binary_and:u8 = vx.get() & vy_val;
            
            vx.set(binary_and);
        }


        /*
        * Logical XOR
        * VX is set to the bitwise/binary exclusive or (XOR) 
        * of VX and VY. VY is not affected.
        * 
        *  */
         fn opcode_8XY3(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);

            let binary_xor:u8 = vx.get() ^ vy_val;
            
            vx.set(binary_xor);
        }

        /*
        * 8XY4: Add
        * 
        * VX is set to the value of VX plus the value of VY. VY is not affected.
        * Unlike 7XNN, this addition will affect the carry flag. 
        * If the result is larger than 255 (and thus overflows the 8-bit 
        * register VX), the flag register VF is set to 1. If it doesn’t overflow, 
        * VF is set to 0.
        * 
        * 
        *  */
         fn opcode_8XY4(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);

            let result:u16 = (vx.get() as u16) + (vy_val as u16);
            // by converting the value to u8 it truncates the most significant bit.
            // e.g.
            // 11111111 + 00000001 = 100000000
            // (100000000  as u8) = 0
            // e.g. 2
            // 11111111 + 00000010 = 100000001
            // (100000001  as u8) = 00000001
            vx.set(result as u8); 

            self.carry_flag(result > 0xff);
        }

        /*
        * Subtract
        * 
        * These both subtract the value in one register from the other,
        * and put the result in VX. In both cases, VY is not affected.
        * 8XY5 sets VX to the result of VX - VY.
        * 8XY7 sets VX to the result of VY - VX.
        * This subtraction will also affect the carry flag, but note that
        * it’s opposite from what you might think. If the minuend (the first operand)
        *  is larger than the subtrahend (second operand),
        *  VF will be set to 1. If the subtrahend is larger, and we “underflow” the result, VF is set to 0. 
        * 
        * Another way of thinking of it is that VF is set to 1 before the subtraction, 
        * and then the subtraction either borrows from VF (setting it to 0) or not. 
        * */
         fn opcode_8XY5(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);
         

            let vx_val:u8 = vx.get();
            let overflow:bool = vy_val > vx_val;



            let result:u8 = if (overflow) {
                0
            } else {
                vx_val - vy_val
            };
            vx.set(result);

            self.carry_flag(overflow);
        }

         fn opcode_8XY7(&mut self, instruction:u16) {
            let vy_val:u8 = self.get_y_register(instruction).get();
            let vx:&mut Register = self.get_x_register(instruction);

            let vx_val:u8 = vx.get(); // temp placeholder
            let overflow:bool = vx_val > vy_val;

            let result:u8 = if (overflow) {
                0
            } else {
                vy_val - vx_val
            };

            vx.set(result);

            self.carry_flag(overflow);
        }

        /*
        *
        * Set Index
        * This sets the index register I to the value NNN.
        *
        *
        */
         fn opcode_ANNN(&mut self, instruction:u16) {
            let immediate:u16 = self.get_nnn(instruction);
            
            self.index.set(immediate);

        }

            /**
         * 
         * jump with offset
         * 
         * @description
         * Starting with CHIP-48 and SUPER-CHIP, BNNN was (probably unintentionally) 
         * changed to work as BXNN: It will jump to the address XNN, plus the value in 
         * the register VX. 
         * e.g.
         * So the instruction B220 will jump to address 220 plus the value in the register V2.
         */
        /* 
        fn opcode_BXNN(&mut self, instruction:u16) {
            if (self.COSMAC) {
                self.opcode_BXNN_original(instruction);
            } else {
                self.opcode_BXNN_super(instruction);
            }
        }
        */
        
        /*
        * random number generator
        * 
        * @description
        *      generates a random unsigned 8 bit number, 
        *      bitwise AND the number with immediate nn
        *      then places it in register vx
        *      e.g 
        *          random_number_u8 & nn
        */
         fn opcode_CXNN(&mut self, instruction:u16) {
            let nn:u16 = self.get_nn(instruction);
            let vx:&mut Register = self.get_x_register(instruction);

            // create random number
            let mut rng:ThreadRng = rand::thread_rng();
            let randomNumber:u8 = rng.gen();

            vx.set(randomNumber & (nn as u8));

        }

        /*
        *
        * display
        * 
        * @description
        * 
        *  this opcode seems confusing, thats correct it seems
        *  but what it actually does is print a sprite upon
        *  the screen with max dimensions 8 wide by 16 pixel high
        * 
        *  it first prints from left to right, then top to bottom
        *  the sprite below is a 8 wide by 9 pixel high sprite
        * 
        *  *       *
        *   *     * 
        *    *   *   
        *     * *    
        *      *        
        *     * *    
        *    *   *   
        *   *     * 
        *  *       *  
        * 
        * 
        * each pixel is a max of eight pixels wide
        * note the screen is measured from the top left corner, using x and y coordinates
        * 
        * 1. it grabs the first location x from vx register and y from vy.
        * then the N(maximum of 16) value is used to determine how high the sprite is(how many rows). 
        * 
        * 2. create two variables x,y and set them to the values in vx and vy respectively.
        *    then modulo them with their respective axis size (x = 64, y = 32)
        *    note: (x & 63) and (y & 31) is equilavent to modulo
        *    this creates wrap around on the screen. 
        *     wrap-around: where a sprite that is outside the dimensions of the screen
        *                  gets printed on the opposite side.
        *      e.g if a sprite begins at x = 68 y = 2
        *          then the sprite would instead be printed at x = 4, y = 2(68 - 64 = 4)
        *          then do the same for y if the sprite is outside the screen
        * 
        * 3. use n to determine the height of the sprite then 
        *     then print each row.
        *    if the y row is outside the screens dimensions then cancel the loop
        *    printing pixels
        *      1. cancel loop if the row is outside the screens dimensions
        *      2. get an 8 bit memory(spriterow) using the address in the index register( add
        *          the row number) to get the corresponding eight bits for each row
        *      3. each pixel is represented by a bit in the spriterow, from most significant to the
        *         least significant bit.
        *          loop through the each column grabbing each bit where 1 = turn on pixel and 
        *          0 = turn off,
        *          if the bit value is 1 and the pixel on the screen is already turned on(true = 1),
        *          set the vf register(carry register) to 1
        *      
        *      
        */         

         fn opcode_DXYN(&mut self, instruction:u16) {
            let n:u8 = self.get_n(instruction) as u8;
            let index_val:u16 = self.index.get();

            // sometimes the sprites wrap around
            let mut x:u8 = self.get_x_register(instruction).get() & 63; // vx % 64
            let mut y:u8 = self.get_y_register(instruction).get() & 31; // vy % 32

            let mut result:bool = false;
            let mut spriteRow:u8 = 0;
            let mut bit:u8 = 0;

            for row in 0..n {
                if (row > 31) {
                    break;
                }
                spriteRow = self.memory.read(index_val + (row  as u16));
                for column in 0..8 {
                    // cancel loop if outside screens dimensions
                    if (column > 63) {
                        break;
                    }

                    bit = (spriteRow >> (7 - column)) & 0x1;
                    result = self.display.set_pixel((bit == 1),x+column,y+row);
                    self.carry_flag(result);
                }
            }
        }
        /*
        *
        * skips if key
        * 
        * Like the earlier skip instructions, these two also skip 
        * the following instruction based on a condition. These skip 
        * based on whether the player is currently pressing a key or not.
        * 
        * These instructions (unlike the later FX0A) don’t wait for input,
        * they just check if the key is currently being held down.
        * 
        * EX9E will skip one instruction (increment PC by 2) 
        * if the key corresponding to the value in VX is pressed.
        * 
        *  */
         fn opcode_EX9E(&mut self, instruction:u16) {
            let vx_val:i8 = self.get_x_register(instruction).get() as i8;
            if self.keyboard.key_is_null() {
                return;
            } else if self.keyboard.get_key() == vx_val {
                self.skip_word_instruction();
            }

        }
            /*
        *
        * skips if key
        * 
        * Like the earlier skip instructions, these two also skip 
        * the following instruction based on a condition. These skip 
        * based on whether the player is currently pressing a key or not.
        * 
        * These instructions (unlike the later FX0A) don’t wait for input,
        * they just check if the key is currently being held down.
        * 
        * EXA1 skips if the key corresponding to the value in VX is not pressed.
        * 
        *  */
         fn opcode_EXA1(&mut self, instruction:u16) {
            let vx_val:i8 = self.get_x_register(instruction).get() as i8;
            if self.keyboard.key_is_null() {
                return;
            } else if self.keyboard.get_key() != vx_val {
                self.skip_word_instruction();
            }
        }
        /**
         * timers
         * 
         * FX07 sets VX to the current value of the delay timer
         */
         fn opcode_FX07(&mut self, instruction:u16) {
            let delay:u8 = self.timers.get_delay();
            let vx:&mut Register = self.get_x_register(instruction);

            vx.set(delay);

        }

        /*
        * timer
        * FX15 sets the delay timer to the value in VX
        * 
        *  */
         fn opcode_FX15(&mut self,instruction:u16) {
            let vx_val:u8 = self.get_x_register(instruction).get();

            self.timers.set_delay(vx_val);
        }
        /*
        * timer
        * FX18 sets the sound timer to the value in VX
        * 
        * Note: there’s no instruction to read the sound timer; 
        *       the sound timer will simply make a beeping sound 
        *       as long as it’s above 0.
        *  */
         fn opcode_FX18(&mut self,instruction:u16) {
            let vx_val:u8 = self.get_x_register(instruction).get();

            self.timers.set_sound(vx_val);
        }

        /*
        *
        * Add to index
        * 
        * The index register I will get the value in VX added to it.
        * 
        * note: 
        * Unlike other arithmetic instructions, this did not affect VF
        * on overflow on the original COSMAC VIP. However, it seems that
        * some interpreters set VF to 1 if I “overflows” from 0FFF to above
        * 1000 (outside the normal addressing range). This wasn’t the case on
        * the original COSMAC VIP, at least, but apparently the CHIP-8 interpreter
        * for Amiga behaved this way. At least one known game, Spacefight 2091!, 
        * relies on this behavior. I don’t know of any games that rely on this 
        * not happening, so perhaps it’s safe to do it like the Amiga interpreter did.
        * 
        * remember addresses or index register only uses 12 bits, not the full 16
        */
         fn opcode_FX1E(&mut self, instruction:u16) {
            let mut value:u16 = self.get_x_register(instruction).get() as u16;

            value += self.index.get();

            self.index.set(value & 0xfff);

            self.carry_flag(value > 0xfff);
        }
        /* 
        * get key
        * 
        * This instruction “blocks”; it stops executing instructions and waits for key input (or loops forever, unless a key is pressed).
        * In other words, if you followed my advice earlier and increment PC after fetching each instruction, then it should be decremented again here unless a key is pressed. Otherwise, PC should simply not be incremented.
        *
        * Although this instruction stops the program from executing further instructions, the timers (delay timer and sound timer) should still be decreased while it’s waiting.
        * If a key is pressed while this instruction is waiting for input, its hexadecimal value will be put in VX and execution continues.
        * 
        */
         fn opcode_FX0A(&mut self, instruction:u16) {
  
            if self.keyboard.key_is_null() {
                self.reverse_word_instruction();
            } else {
                let key:u8 = self.keyboard.get_key() as u8;
                let vx:&mut Register = self.get_x_register(instruction);
                vx.set(key);
            }
        }

        /*
        * Font Character
        * 
        * @description
        * The index register I is set to the address of the hexadecimal
        * character in VX. You probably stored that font somewhere in the
        * first 512 bytes of memory, so now you just need to point I to
        * the right character.
        *  */
         fn opcode_FX29(&mut self, instruction:u16) {
            let vx_val:u8 = self.get_x_register(instruction).get();
            
            self.index.set(vx_val as u16);
        }
        /**
         * 
         * Binary-coded decimal conversion
         * 
         * This instruction is a little involved. It takes the number in VX
         * (which is one byte, so it can be any number from 0 to 255) and converts
         * it to three decimal digits, storing these digits in memory at the
         * address in the index register I. For example, if VX contains 156
         * (or 9C in hexadecimal), it would put the number 1 at the address in 
         * I, 5 in address I + 1, and 6 in address I + 2.
         * 
         */
         fn opcode_FX33(&mut self, instruction:u16) {
            let index_val:u16 = self.index.get();
            let mut vx_val:u8 = self.get_x_register(instruction).get();
            let mut divider:u8 = 100;
            let mut res:u8 = 0;
            let mut count:u16 = 0;
            for i in 0..=2 {
                res = vx_val / divider;
                if res > 0 {
                    self.memory.write((index_val + count), res);
                    vx_val %= divider;
                    count += 1;
                }
                divider /= 10;
            }
        }
    }



    impl<'a> CpuVariantInstructions for  Cpu< Cosmac> where Cpu< Cosmac> :CpuInstructions{
        
        fn get_cputype(&self) -> String {
            return String::from("Cosmac");
        }
  
  /*
        *
        * 
        * 8XY6 and 8XYE: Shift
        * 
        * copies the value in vy register, places it into the vx register
        * bit shifts the value to the right by one
        * 
        * if the truncated bit is 1, then set the carry flag to 1
        * otherwise set to 0
        * 
        * @note compatible with the COSMAC VIP chip8 (original chip8) only
        * 
        */
        fn opcode_8XY6(&mut self, instruction:u16) {


            let vy: &mut Register = self.get_y_register(instruction);
            let y: u8 = vy.get();

            let vx: &mut Register = self.get_x_register(instruction);
            let result:u8 = y >> 1;

            vx.set(result);

            self.carry_flag((y & 0x1) == 1);
        }
    /*
        *
        * 
        * 8XYE: Shift
        * 
        * copies the value in vy register, places it into the vx register
        * bit shifts the value to the left by one
        * 
        * if the truncated bit is 1, then set the carry flag to 1
        * otherwise set to 0
        * 
        * @note compatible with the COSMAC VIP chip8 (original chip8) only
        * 
        */
         fn opcode_8XYE(&mut self, instruction:u16) {
            let vy: &mut Register = self.get_y_register(instruction);
            let y:u8  = vy.get();
            let result:u8 = vy.get() << 1;

            let vx:&mut Register = self.get_x_register(instruction);
            vx.set(result);

            

            self.carry_flag(y & 0x80 > 0);
        }

        /*
        *
        * jump with offset
        * 
        * @description
        *  jumps to address nnn plus the value in register v0
        * e.g.
        *      pc = nnn + v0
        * 
        * e.g 
        *      if NNN = 0x200
        *      and v0 = 0x01
        *      then the new address would be 0x201
        * 
        * @note only compatible in the original COSMAC VIP interpreter
        * 
        *  */
         fn opcode_BXNN(&mut self, instruction:u16) {
            let v0:&mut Register = self.registers.get(0);
            let value:u8 = v0.get();
            let nnn:u16 = self.get_nnn(instruction);
        
            self.pc.set((value as u16 ) + nnn);

        }
        /**
         * Store and load memory
         * 
         * @description
         * The original CHIP-8 interpreter for the COSMAC VIP actually
         * incremented the I register while it worked. Each time it stored
         * or loaded one register, it incremented I. After the instruction
         * was finished, I would end up being set to the new value I + X + 1.
         */
         fn opcode_FX55(&mut self, instruction:u16) {
            let vx:&mut Register = self.get_x_register(instruction);
            let mut val:u8 = vx.get();

            for i in 0..val {
                self.memory.write(self.index.get(), self.registers.get(i).get());
                self.index.increment();
            }
        }
    /**
         * Store and load memory
         * 
         * @description
         * The original CHIP-8 interpreter for the COSMAC VIP actually
         * incremented the I register while it worked. Each time it stored
         * or loaded one register, it incremented I. After the instruction
         * was finished, I would end up being set to the new value I + X + 1.
         */
         fn opcode_FX65(&mut self, instruction:u16) {
            let vx:&mut Register = self.get_x_register(instruction);
            let mut val:u8 = vx.get();

            for i in 0..val {
                let mem:u8 = self.memory.read(self.index.get());
                self.registers.get(i).set(mem);
                self.index.increment();
            }
        }


    }

    impl<'a> CpuVariantInstructions for Cpu< Super> where  Cpu< Super>:CpuInstructions {

        fn get_cputype(&self) -> String{
            return String::from("Super");
        }

        /*
        *
        * 
        * 8XY6 and 8XYE: Shift
        * 
        * shifts the vx register only completely forgetting the vy register
        * 
        * if the truncated bit is 1, then set the carry flag to 1
        * otherwise set to 0
        * 
        * @note starting with CHIP-48 and SUPER-CHIP in the early 1990s, 
        * these instructions were changed so that they shifted VX in place, 
        * and ignored the Y completely.
        * 
        */
        fn opcode_8XY6(&mut self, instruction:u16) {
            let mut vx:&mut Register = self.get_x_register(instruction);

            let vx_val:u8 = vx.get();
            let result:u8 = vx_val >> 1;
            vx.set(result);

            self.carry_flag((vx_val & 0x1) == 1);
        }
        /*
        *
        * 
        * 8XYE: Shift
        * 
        * shifts the vx register only completely forgetting the vy register
        * 
        * if the truncated bit is 1, then set the carry flag to 1
        * otherwise set to 0
        * 
        * @note starting with CHIP-48 and SUPER-CHIP in the early 1990s, 
        * these instructions were changed so that they shifted VX in place, 
        * and ignored the Y completely.
        * 
        */
         fn opcode_8XYE(&mut self, instruction:u16) {
            let mut vx:&mut Register = self.get_x_register(instruction);
            
            let vx_val:u8 = vx.get();
            let result:u8 = vx_val << 1;
            vx.set(result);

            self.carry_flag((vx_val & 0x80) > 0);
        }

        /**
         * 
         * jump with offset
         * 
         * @description
         * Starting with CHIP-48 and SUPER-CHIP, BNNN was (probably unintentionally) 
         * changed to work as BXNN: It will jump to the address XNN, plus the value in 
         * the register VX. 
         * e.g.
         * So the instruction B220 will jump to address 220 plus the value in the register V2.
         */
         fn opcode_BXNN(&mut self, instruction:u16) {
            let mut vx:&mut Register = self.get_x_register(instruction);
            let value:u8 = vx.get();

            let nnn:u16 = self.get_nnn(instruction);
            
            self.pc.set((value as u16) + nnn);

        }
        /*
        *
        * Store and load memory
        * 
        * @description
        * For FX55, the value of each variable register from V0 to VX inclusive
        * (if X is 0, then only V0) will be stored in successive memory addresses,
        * starting with the one that’s stored in I. V0 will be stored at the address
        * in I, V1 will be stored in I + 1, and so on, until VX is stored in I + X.
        * 
        */
         fn opcode_FX55(&mut self, instruction:u16) {
            let mut vx:&mut Register  = self.get_x_register(instruction);
            let vx_val:u16 = vx.get() as u16;
            let index_val:u16 = self.index.get();

            for i in 0..=vx_val {
                let register_val:u8 = self.registers.get(i as u8).get();
                self.memory.write(index_val + i as u16, register_val);
            }
        }
        /*
        * Store and load memory
        * 
        * @description
        * FX65 does the opposite; it takes the value stored at the memory
        * addresses and loads them into the variable registers instead.
        * 
        
        */
         fn opcode_FX65(&mut self, instruction:u16) {
            let mut vx:&mut Register  = self.get_x_register(instruction);
            let mut vx_val:u16 = vx.get() as u16;
            let index_val:u16 = self.index.get();

            for i in 0..=vx_val {
                let mem:u8 = self.memory.read(index_val + i);
                self.registers.get(i as u8).set(mem);
            }
        }


/* 
        #[inline]
        fn tick(&mut self) {
            //fetch
            let instruction:u16 = self.get_word_from_pc();

            // decode and execute
            self.execute(instruction);

        }

        fn execute(&mut self, instruction_raw:u16) {

            let mut instruction:Address = Address{address:instruction_raw};

            match true {
                true if instruction.is_equal(0x00E0) => self.opcode_00E0(),
                true if instruction.is_equal(0x00EE) => self.opcode_00EE(),
                true if instruction.in_range(0x1000,0x1FFF) => self.opcode_1NNN(instruction_raw),
                true if instruction.in_range(0x2000, 0x2FFF) => self.opcode_2NNN(instruction_raw),
                true if instruction.in_range(0x3000,0x3FFF) => self.opcode_3XNN(instruction_raw),
                true if instruction.in_range(0x4000,0x4FFF) => self.opcode_4XNN(instruction_raw),
                true if instruction.in_range(0x5000,0x5FFF) => self.opcode_5XY0(instruction_raw),
                true if instruction.in_range(0x6000,0x6FFF) => self.opcode_6XNN(instruction_raw),
                true if instruction.in_range(0x7000,0x7FFF) => self.opcode_7XNN(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8000 as u16) => self.opcode_8XY0(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8001 as u16) => self.opcode_8XY1(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8002 as u16) => self.opcode_8XY2(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8004 as u16) => self.opcode_8XY4(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8005 as u16) => self.opcode_8XY5(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8006 as u16) => self.opcode_8XY6(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8007 as u16) => self.opcode_8XY7(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x800E as u16) => self.opcode_8XYE(instruction_raw),
                true if instruction.in_range(0x9000, 0x9FFF) => self.opcode_9XY0(instruction_raw),
                true if instruction.in_range(0xA000, 0xAFFF) => self.opcode_ANNN(instruction_raw),
                true if instruction.in_range(0xB000, 0xBFFF) => self.opcode_BXNN(instruction_raw),
                true if instruction.in_range(0xC000, 0xCFFF) => self.opcode_CXNN(instruction_raw),
                true if instruction.in_range(0xD000, 0xDFFF) => self.opcode_DXYN(instruction_raw),
                true if instruction.match_nibbles_HNML(0xE09E as u16) => self.opcode_EX9E(instruction_raw),
                true if instruction.match_nibbles_HNML(0xE0A1 as u16) => self.opcode_EXA1(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF007 as u16) => self.opcode_FX07(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF00A as u16) => self.opcode_FX0A(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF015 as u16) => self.opcode_FX15(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF018 as u16) => self.opcode_FX18(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF01E as u16) => self.opcode_FX1E(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF029 as u16) => self.opcode_FX29(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF033 as u16) => self.opcode_FX33(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF055 as u16) => self.opcode_FX55(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF065 as u16) => self.opcode_FX65(instruction_raw)
            }
        }
        */
    }

    
    pub trait CpuExtendedInstructions: CpuVariantInstructions {
        fn tick(&mut self);
        fn execute(&mut self, instruction:u16);
        fn as_cpu(&mut self) -> &mut dyn CpuInstructions;
        fn as_cpuvariant(&mut self) -> &mut dyn CpuVariantInstructions;
            
        fn get_memory(&mut self) -> &mut Memory;
        fn get_display(&mut self) -> &mut Display;
        fn get_timers(&mut self) -> &mut Timers;
        fn get_keyboard(&mut self) -> &mut Keyboard;

        fn get_cpu_type(&self) -> String;


    }

    impl< V: CpuVariant> CpuExtendedInstructions for  Cpu< V> where Cpu< V>: CpuVariantInstructions  {

        fn get_cpu_type(&self) -> String {
            return self.get_cputype();
        }
        fn get_memory(&mut self) -> &mut Memory {
            return &mut self.memory;
        }

        fn get_timers(&mut self) -> &mut Timers {
            return &mut self.timers;
        }

        fn get_keyboard(&mut self) -> &mut Keyboard {
            return &mut self.keyboard;
        }

        fn get_display(&mut self) -> &mut Display {
            return &mut self.display;
        }

        fn as_cpu(&mut self) -> &mut dyn CpuInstructions {
            self
        }
        fn as_cpuvariant(&mut self) -> &mut dyn CpuVariantInstructions {
            self
        }


        #[inline]
        fn tick(&mut self) {
            //fetch
            let instruction:u16 = self.get_word_from_pc();

            // decode and execute
            self.execute(instruction);

        }

        fn execute(&mut self, instruction_raw:u16) {

            let mut instruction:Address = Address::init(instruction_raw);

            match true {
                true if instruction.is_equal(0x00E0) => self.opcode_00E0(),
                true if instruction.is_equal(0x00EE) => self.opcode_00EE(),
                true if instruction.in_range(0x1000,0x1FFF) => self.opcode_1NNN(instruction_raw),
                true if instruction.in_range(0x2000, 0x2FFF) => self.opcode_2NNN(instruction_raw),
                true if instruction.in_range(0x3000,0x3FFF) => self.opcode_3XNN(instruction_raw),
                true if instruction.in_range(0x4000,0x4FFF) => self.opcode_4XNN(instruction_raw),
                true if instruction.in_range(0x5000,0x5FFF) => self.opcode_5XY0(instruction_raw),
                true if instruction.in_range(0x6000,0x6FFF) => self.opcode_6XNN(instruction_raw),
                true if instruction.in_range(0x7000,0x7FFF) => self.opcode_7XNN(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8000 as u16) => self.opcode_8XY0(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8001 as u16) => self.opcode_8XY1(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8002 as u16) => self.opcode_8XY2(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8004 as u16) => self.opcode_8XY4(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8005 as u16) => self.opcode_8XY5(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8006 as u16) => self.opcode_8XY6(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8007 as u16) => self.opcode_8XY7(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x800E as u16) => self.opcode_8XYE(instruction_raw),
                true if instruction.in_range(0x9000, 0x9FFF) => self.opcode_9XY0(instruction_raw),
                true if instruction.in_range(0xA000, 0xAFFF) => self.opcode_ANNN(instruction_raw),
                true if instruction.in_range(0xB000, 0xBFFF) => self.opcode_BXNN(instruction_raw),
                true if instruction.in_range(0xC000, 0xCFFF) => self.opcode_CXNN(instruction_raw),
                true if instruction.in_range(0xD000, 0xDFFF) => self.opcode_DXYN(instruction_raw),
                true if instruction.match_nibbles_HNML(0xE09E as u16) => self.opcode_EX9E(instruction_raw),
                true if instruction.match_nibbles_HNML(0xE0A1 as u16) => self.opcode_EXA1(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF007 as u16) => self.opcode_FX07(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF00A as u16) => self.opcode_FX0A(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF015 as u16) => self.opcode_FX15(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF018 as u16) => self.opcode_FX18(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF01E as u16) => self.opcode_FX1E(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF029 as u16) => self.opcode_FX29(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF033 as u16) => self.opcode_FX33(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF055 as u16) => self.opcode_FX55(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF065 as u16) => self.opcode_FX65(instruction_raw),
                _ => panic!("unknown opcode {instruction_raw}"),
            }
        }
    
    }
/*

        fn execute<T: CpuExtendedInstructions>(cpu:&mut T, instruction_raw:u16) {

            let mut instruction:Address = Address{address:instruction_raw};

            match true {
                true if instruction.is_equal(0x00E0) => cpu.opcode_00E0(),
                true if instruction.is_equal(0x00EE) => cpu.opcode_00EE(),
                true if instruction.in_range(0x1000,0x1FFF) => cpu.opcode_1NNN(instruction_raw),
                true if instruction.in_range(0x2000, 0x2FFF) => cpu.opcode_2NNN(instruction_raw),
                true if instruction.in_range(0x3000,0x3FFF) => cpu.opcode_3XNN(instruction_raw),
                true if instruction.in_range(0x4000,0x4FFF) => cpu.opcode_4XNN(instruction_raw),
                true if instruction.in_range(0x5000,0x5FFF) => cpu.opcode_5XY0(instruction_raw),
                true if instruction.in_range(0x6000,0x6FFF) => cpu.opcode_6XNN(instruction_raw),
                true if instruction.in_range(0x7000,0x7FFF) => cpu.opcode_7XNN(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8000 as u16) => cpu.opcode_8XY0(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8001 as u16) => cpu.opcode_8XY1(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8002 as u16) => cpu.opcode_8XY2(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8004 as u16) => cpu.opcode_8XY4(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8005 as u16) => cpu.opcode_8XY5(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8006 as u16) => cpu.opcode_8XY6(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x8007 as u16) => cpu.opcode_8XY7(instruction_raw),
                true if instruction.match_nibbles_HNNL(0x800E as u16) => cpu.opcode_8XYE(instruction_raw),
                true if instruction.in_range(0x9000, 0x9FFF) => cpu.opcode_9XY0(instruction_raw),
                true if instruction.in_range(0xA000, 0xAFFF) => cpu.opcode_ANNN(instruction_raw),
                true if instruction.in_range(0xB000, 0xBFFF) => cpu.opcode_BXNN(instruction_raw),
                true if instruction.in_range(0xC000, 0xCFFF) => cpu.opcode_CXNN(instruction_raw),
                true if instruction.in_range(0xD000, 0xDFFF) => cpu.opcode_DXYN(instruction_raw),
                true if instruction.match_nibbles_HNML(0xE09E as u16) => cpu.opcode_EX9E(instruction_raw),
                true if instruction.match_nibbles_HNML(0xE0A1 as u16) => cpu.opcode_EXA1(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF007 as u16) => cpu.opcode_FX07(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF00A as u16) => cpu.opcode_FX0A(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF015 as u16) => cpu.opcode_FX15(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF018 as u16) => cpu.opcode_FX18(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF01E as u16) => cpu.opcode_FX1E(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF029 as u16) => cpu.opcode_FX29(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF033 as u16) => cpu.opcode_FX33(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF055 as u16) => cpu.opcode_FX55(instruction_raw),
                true if instruction.match_nibbles_HNML(0xF065 as u16) => cpu.opcode_FX65(instruction_raw)
            }
        }
*/
   




use super::address::Address;
use super::cpuDependencies;
use super::registers::Register;
use super::registers::Registers;
use super::memory::Memory;
use super::stack::Stack;
use super::display::Display;
use super::keyboard::Keyboard;
use super::timers::Timers;
use super::cpuDependencies::CpuDependencies;


enum CpuVariant {
    SUPER,
    COSMAC,
}


    pub trait CPUVariantInstructionsTrait {

        fn opcode_8XY6(&mut self, instruction:u16);
        fn opcode_8XYE(&mut self, instruction:u16);
        fn opcode_BXNN(&mut self, instruction:u16);
        fn opcode_FX55(&mut self, instruction:u16);
        fn opcode_FX65(&mut self, instruction:u16);
        fn get_x_register(&mut self,instruction:u16) -> &mut Register;
        fn get_y_register(&mut self,instruction:u16) -> &mut Register;
        fn get_nnn(&mut self, instruction:u16) -> u16;

    }


    pub struct CosmacCPU<'a> {
        index:&'a mut Address,
        pc:&'a mut Address,
        stack:&'a mut Stack,
        registers:&'a mut Registers,
        memory:&'a mut Memory,
        display:&'a mut Display,
        keyboard:&'a mut Keyboard,
        timers:&'a mut Timers,

    }

    impl<'a> CosmacCPU<'a> {
        pub fn init<'b>( cpuDependencies:&'b mut CpuDependencies) -> CosmacCPU<'b> {


            CosmacCPU{                 
                index:&mut (cpuDependencies.index),
                pc:&mut (cpuDependencies.pc),
                stack:&mut (cpuDependencies.stack), 
                registers:&mut (cpuDependencies.registers), 
                memory:&mut (cpuDependencies.memory),  
                display:&mut (cpuDependencies.display),
                keyboard:&mut (cpuDependencies.keyboard),
                timers:&mut (cpuDependencies.timers)
            } 
            /* 
            CosmacCPU{                 
                index:index,
                pc:pc,
                stack:stack,
                registers:registers,
                memory:memory,
                display:display,
                keyboard:keyboard,
                timers:timers,
                cd:cpuDependencies,
            }*/
        }

        pub fn carry_flag(&mut self, isOverflowed:bool) {
            if (isOverflowed) {
                let mut overflowRegister:&mut Register = self.registers.get(0xf);
                overflowRegister.set(1 as u8);
            }
        }

   
        
    }
    impl<'a> CPUVariantInstructionsTrait for CosmacCPU<'a> {

        fn get_x_register(&mut self,instruction:u16) -> &mut Register{
            let registerIndexX:u8 = ((instruction & 0x0f00) >> 8) as u8;
            return self.registers.get(registerIndexX);
        }

         fn get_y_register(&mut self, instruction:u16) -> &mut Register {
            let registerIndexX:u8 = ((instruction & 0x00f0) >> 4) as u8;
            return self.registers.get(registerIndexX);
        }
        /*

        can't use this since it double borrows
         fn get_xy_registers(&mut self, instruction:u16) -> (&mut Register,&mut Register) {

            let mut registerX:&mut Register = self.get_x_register(instruction);
            let mut registerY:&mut Register = self.get_y_register(instruction);

            return (registerX,registerY);
        }
        */

        fn get_nnn(&mut self, instruction:u16) -> u16 {
            let nnn:u16 = instruction & 0x0fff;
            return nnn;
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

    pub struct SuperCPU<'a> {
        index:&'a mut Address,
        pc:&'a mut Address,
        stack:&'a mut Stack,
        registers:&'a mut Registers,
        memory:&'a mut Memory,
        display:&'a mut Display,
        keyboard:&'a mut Keyboard,
        timers:&'a mut Timers,
        cd:&'a mut CpuDependencies,

    }

    impl<'a> SuperCPU<'a> {
        pub fn init<'b>( index:&'b mut Address, pc:&'b mut Address,stack:&'b mut Stack, registers:&'b mut Registers, memory:&'b mut Memory,  display:&'b mut Display, keyboard:&'b mut Keyboard,timers:&'b mut Timers, cpuDependencies:&'b mut CpuDependencies) -> SuperCPU<'b> {

            SuperCPU{                 
                index:index,
                pc:pc,
                stack:stack,
                registers:registers,
                memory:memory,
                display:display,
                keyboard:keyboard,
                timers:timers,
                cd:cpuDependencies,
            }
        }


        pub fn carry_flag(&mut self, isOverflowed:bool) {
            if (isOverflowed) {
                let mut overflowRegister:&mut Register = self.registers.get(0xf);
                overflowRegister.set(1 as u8);
            }
        }



    }
    impl<'a> CPUVariantInstructionsTrait for SuperCPU<'a> {

        fn get_x_register(&mut self,instruction:u16) -> &mut Register{
            let registerIndexX:u8 = ((instruction & 0x0f00) >> 8) as u8;
            return self.registers.get(registerIndexX);
        }

         fn get_y_register(&mut self, instruction:u16) -> &mut Register {
            let registerIndexX:u8 = ((instruction & 0x00f0) >> 4) as u8;
            return self.cd.registers.get(registerIndexX);
        }
        /*

        can't use this since it double borrows
         fn get_xy_registers(&mut self, instruction:u16) -> (&mut Register,&mut Register) {

            let mut registerX:&mut Register = self.get_x_register(instruction);
            let mut registerY:&mut Register = self.get_y_register(instruction);

            return (registerX,registerY);
        }
        */

        fn get_nnn(&mut self, instruction:u16) -> u16 {
            let nnn:u16 = instruction & 0x0fff;
            return nnn;
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
            let mut registerX:&mut Register = self.get_x_register(instruction);

            let vx:u8 = registerX.get();
            let result:u8 = vx >> 1;
            registerX.set(result);

            self.carry_flag((vx & 0x1) == 1);
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
            let mut registerX:&mut Register = self.get_x_register(instruction);
            
            let vx:u8 = registerX.get();
            let result:u8 = vx << 1;
            registerX.set(result);

            self.carry_flag((vx & 0x80) > 0);
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
            let mut val:u8 = vx.get();

            for i in 0..=val {
                self.memory.write((self.index.get() + i as u16) as u16, self.registers.get(i).get());
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
            let mut val:u8 = vx.get();

            for i in 0..=val {
                let mem:u8 = self.memory.read(self.index.get() + i as u16);
                self.registers.get(i).set(mem);
            }
        }

    }

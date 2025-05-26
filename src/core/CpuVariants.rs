use crate::core::cpu::Cpu;


use crate::core::registers::Register;



    pub trait CpuVariant {

        fn opcode_8XY6(&mut self, instruction:u16);
        fn opcode_8XYE(&mut self, instruction:u16);
        fn opcode_BXNN(&mut self, instruction:u16);
        fn opcode_FX55(&mut self, instruction:u16);
        fn opcode_FX65(&mut self, instruction:u16);

    }

    pub struct Cosmac;
    pub struct Super;

    impl CpuVariant for Cpu<Cosmac> {
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

    impl CpuVariant for Cpu<Super> {


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
            let result:u8 = vx << 1;
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
                let register_val:u8 = self.registers.get(i).get();
                self.memory.write(index_val + i, register_val);
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
    }


   
    


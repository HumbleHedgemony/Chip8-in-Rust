#[allow(arithmetic_overflow)]

use crate::core::cpu::Cpu;
use crate::core::address;
use crate::core::registers::Register;
use crate::core::cpu::CpuVariant;
use crate::core::cpu::CpuVariantInstructions;
use crate::core::cpu::Cosmac;
use crate::core::cpu::Super;
extern crate rand;

#[cfg(test)]
mod cputest {


use rand::Rng;

pub struct Empty;
impl CpuVariant for Empty {}

use crate::core::cpu::Cpu;
use crate::core::address;
use crate::core::registers::Register;
use crate::core::cpu::CpuVariant;
use crate::core::cpu::CpuVariantInstructions;

use crate::core::display::Display;
use crate::core::keyboard::Keyboard;
use crate::core::timers::Timers;
extern crate rand;

        #[test]
        fn opcode_0NNN() {

        }
        /*
        * Clear Screen
        * 
        * @description
        *  This is pretty simple: It should clear the display, 
        *                         turning all pixels off to 0.
        *  */
        #[test]
        fn opcode_00E0() {
            let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);

            cpu.opcode_00E0();

        }

        /*
        * subroutine
        * 
        * @description
        * Returning from a subroutine is done with 00EE
        * removing (“popping”) the last address from the stack and setting the PC to it
        */
        #[test]
        fn opcode_00EE() {
            let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);

            let val:u16 = 0xAfff;
            cpu.stack.push(val);

            cpu.opcode_00EE();

            let cpu_val:u16 = cpu.pc.get();

            assert_eq!(val,cpu_val);
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
        #[test]
        fn opcode_1NNN() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x0FEC;
            let address:u16 = cpu.get_nnn(instruction);
            
            cpu.opcode_1NNN(instruction);

            assert_eq!(cpu.pc.get(),address);
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

        #[test]
        fn opcode_2NNN() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x2BFF;
            let pc_val:u16 = cpu.pc.get();

            cpu.opcode_2NNN(instruction);

            let val:u16 = cpu.stack.pop();

            assert_eq!(pc_val, val);
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
        #[test]
        fn opcode_3XNN() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x03ff;

            let pc_val:u16 = cpu.pc.get();
            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(0xff);
            
            cpu.opcode_3XNN(instruction);


            assert_eq!(cpu.pc.get(), pc_val+2);
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

        #[test]
        fn opcode_4XNN() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x4599;

            let pc_val:u16 = cpu.pc.get();
            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(0x99);
            
            cpu.opcode_4XNN(instruction);
            assert_eq!(cpu.pc.get(), pc_val+2);

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
        #[test]
        fn opcode_5XY0() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x5340;

            let pc_val:u16 = cpu.pc.get();
            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(0x99);
            cpu.registers.get(4).set(0x99);
            
            cpu.opcode_5XY0(instruction);
            
            assert_eq!(cpu.pc.get(), pc_val+2);
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
        #[test]
        fn opcode_9XY0() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x9340;

            let pc_val:u16 = cpu.pc.get();
            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(0x99);
            cpu.registers.get(4).set(0x98);
            
            cpu.opcode_9XY0(instruction);
            
            assert_eq!(cpu.pc.get(), pc_val+2);
        }

        /*
        * set
        * 
        * @description
        * 
        * Simply set the register VX to the value NN.
        * 
        */
        fn opcode_6XNN() {

                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x6340;
            let nn:u8 = cpu.get_nn(instruction) as u8;

            cpu.opcode_6XNN(instruction);
            
            assert_eq!(cpu.registers.get(3).get(), nn);

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
        #[test]
        fn opcode_7XNN() {

                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x9340;
            let value:u16 = 0x09;

            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(value as u8);

            
            cpu.opcode_7XNN(instruction);
            
            assert_eq!(cpu.registers.get(3).get() as u16, value + nn);
            
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
        #[test]
        fn opcode_8XY0() {

                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x8340;
            let value:u16 = 0x09;

            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(value as u8);
            cpu.registers.get(4).set((value+5) as u8);
            
            cpu.opcode_8XY0(instruction);
            
            assert_eq!(cpu.registers.get(3).get() as u16, value+5);
        }

        /*
        *
        * Binary OR
        * VX is set to the bitwise/binary logical disjunction (OR) 
        * of VX and VY. VY is not affected.
        * 
        *  */
        #[test]
        fn opcode_8XY1() {


                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x8341;
            let x_val:u16 = 0x09;
            let y_val:u16 = 0x5;

            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(x_val as u8);
            cpu.registers.get(4).set(y_val as u8);
            
            cpu.opcode_8XY1(instruction);
            
            assert_eq!(cpu.registers.get(3).get() as u16, x_val | y_val);

        }
        /*
        * Binary AND
        * VX is set to the bitwise/binary logical disjunction (AND) 
        * of VX and VY. VY is not affected.
        * 
        *  */
        #[test]
        fn opcode_8XY2() {


                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x8341;
            let x_val:u8 = 0x09;
            let y_val:u8 = 0x5;

            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(x_val);
            cpu.registers.get(4).set(y_val);
            
            cpu.opcode_8XY2(instruction);
            
            assert_eq!(cpu.registers.get(3).get(), x_val & y_val);
        }


        /*
        * Logical XOR
        * VX is set to the bitwise/binary exclusive or (XOR) 
        * of VX and VY. VY is not affected.
        * 
        *  */
        #[test]
        fn opcode_8XY3() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x8341;
            let x_val:u16 = 0x09;
            let y_val:u16 = 0x5;

            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(x_val as u8);
            cpu.registers.get(4).set(y_val as u8);
            
            cpu.opcode_8XY3(instruction);
            
            assert_eq!(cpu.registers.get(3).get() as u16, x_val ^ y_val);
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
        #[test]
        fn opcode_8XY4() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x8344;
            let value:u16 = 0x09;
            let vy_val:u8 = 0x19;

            let nn:u16 = cpu.get_nn(instruction);
            cpu.registers.get(3).set(value as u8);
            cpu.registers.get(4).set(vy_val);

            cpu.opcode_8XY4(instruction);
            
            assert_eq!(cpu.registers.get(3).get() as u16, value + vy_val as u16);
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
        #[test]
        fn opcode_8XY5() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x8345u16;
            let value:u16 = 0x09;
            let vy_val:u8 = 0x19;

            cpu.registers.get(3).set(value as u8);
            cpu.registers.get(4).set(vy_val);

            cpu.opcode_8XY5(instruction);
            
            assert_eq!(cpu.registers.get(3).get() as u16, 0);
            assert_eq!(cpu.registers.get(15).get(), 1);

            let instruction:u16 = 0x8345u16;
            let value:u16 = 0x30;
            let vy_val:u8 = 0x19;

            cpu.registers.get(3).set(value as u8);
            cpu.registers.get(4).set(vy_val);

            cpu.opcode_8XY5(instruction);
            
            assert_eq!(cpu.registers.get(3).get(), value as u8 - vy_val);
            assert_eq!(cpu.registers.get(15).get(), 0);

        }
        #[test]
        fn opcode_8XY7() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x8347u16;
            let value:u16 = 0x09;
            let vy_val:u8 = 0x19;

            cpu.registers.get(3).set(value as u8);
            cpu.registers.get(4).set(vy_val);

            cpu.opcode_8XY7(instruction);
            
            assert_eq!(cpu.registers.get(3).get() as u16, vy_val as u16 - value);
            assert_eq!(cpu.registers.get(15).get(), 0);
        }

        /*
        *
        * Set Index
        * This sets the index register I to the value NNN.
        *
        *
        */
        #[test]
        fn opcode_ANNN() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0xA345u16;
            let nnn:u16 = cpu.get_nnn(instruction);

            cpu.opcode_ANNN(instruction);

            assert_eq!(cpu.index.get(),nnn);

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
        fn opcode_BXNN() {
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
        #[test]
        fn opcode_CXNN() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);

            let instruction:u16 = 0xC352;
            let nn:u8 = 52;
            cpu.registers.get(3).set(nn);


            cpu.opcode_CXNN(instruction);
            // create random number
            let mut rng = rand::thread_rng();
            let randomNumber:u8 = rng.gen();


            assert_ne!(randomNumber & (nn as u8), cpu.registers.get(3).get());

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

        fn opcode_DXYN() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);

            let instruction:u16 = 0xD358;
            let rows:u8 = 9;
            // sometimes the sprites wrap around
            let mut x:u8 = 65 & 63; // vx % 64
            let mut y:u8 = 31 & 31; // vy % 32
            cpu.registers.get(3).set(x);
            cpu.registers.get(5).set(y);
            let n:u8 = cpu.get_n(instruction) as u8;
            let index_val:u16 = 0x200;
            cpu.index.set(index_val);

            let rows: [u8; 8] = [
                215,
                226,
                72,
                41,
                49,
                99,
                21,
                153
                ];
            for row in 0..rows.len() {
                cpu.memory.write(index_val + row as u16,rows[row] );
            }

            cpu.opcode_DXYN(instruction);
            let mut bit:u8  =  0;

            for row in 0..rows.len() {
                for column in 0..8 as u8 {
                    bit = (rows[row] >> (7 - column)) & 0x1;
                    assert_eq!((bit == 1), cpu.display.get_pixel(x+column, y+row as u8));
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
        #[test]
        fn opcode_EX9E() {

                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x939Eu16;
            let pc_val:u16 = cpu.pc.get();
            cpu.registers.get(3).set(9);
            cpu.keyboard.nine();

            cpu.opcode_EX9E(instruction);

            assert_eq!(cpu.pc.get(),pc_val+2);

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
        #[test]
        fn opcode_EXA1() {
   
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0x939Eu16;
            let pc_val:u16 = cpu.pc.get();
            cpu.registers.get(3).set(7);
            cpu.keyboard.nine();

            cpu.opcode_EXA1(instruction);

            assert_eq!(cpu.pc.get(),pc_val+2);

        }
        /**
         * timers
         * 
         * FX07 sets VX to the current value of the delay timer
         */
        #[test]
        fn opcode_FX07() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0xF307u16;
            let delay:u8 = 8;
            cpu.timers.set_delay(delay);

            cpu.opcode_FX07(instruction);

            assert_eq!(cpu.registers.get(3).get(),delay);

        }

        /*
        * timer
        * FX15 sets the delay timer to the value in VX
        * 
        *  */
        #[test]
        fn opcode_FX15() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0xF315u16;
            let delay:u8 = 8;
            cpu.registers.get(3).set(delay);

            cpu.opcode_FX15(instruction);

            assert_eq!(cpu.timers.get_delay(),delay);
        }
        /*
        * timer
        * FX18 sets the sound timer to the value in VX
        * 
        * Note: there’s no instruction to read the sound timer; 
        *       the sound timer will simply make a beeping sound 
        *       as long as it’s above 0.
        *  */
        #[test]
        fn opcode_FX18() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0xF318u16;
            let delay:u8 = 8;
            cpu.registers.get(3).set(delay);

            cpu.opcode_FX18(instruction);

            assert_eq!(cpu.timers.get_sound(),delay);
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
        #[test]
        fn opcode_FX1E() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0xF31Eu16;
            let vx_val:u8 = 8;
            let index_val:u16 = 12;
            cpu.registers.get(3).set(vx_val);
            cpu.index.set(index_val);

            cpu.opcode_FX1E(instruction);

            assert_eq!(cpu.index.get(),(vx_val as u16 + index_val) & 0x0fff);
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
        #[test]
        fn opcode_FX0A() {
               
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0xF30Au16;
            let pc_val:u16 = cpu.pc.get();

            cpu.opcode_FX0A(instruction);

            assert_eq!(pc_val-2,cpu.pc.get());

            cpu.keyboard.nine();

            cpu.opcode_FX0A(instruction);

            assert_eq!(cpu.registers.get(3).get(),9);
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
        #[test]
        fn opcode_FX29() {
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0xF329u16;
            cpu.registers.get(3).set(8);
            
            cpu.opcode_FX29(instruction);
            
            assert_eq!(cpu.index.get(),8);
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
        #[test]
        fn opcode_FX33() {
            let address:u16 = 0x200;
            let mut value:u8 = 244;
            let instruction:u16 = 0xF333;
                        let mut display:Display = Display::init();
            let mut keyboard:Keyboard = Keyboard::init();
            let mut timers:Timers = Timers::init();
            let mut cpu:Cpu<Empty> = Cpu::<Empty>::init(&mut display, &mut keyboard, &mut timers);
            let instruction:u16 = 0xF329u16;
            cpu.registers.get(3).set(value);
            cpu.index.set(address);

            cpu.opcode_FX33(instruction);
            let mut divider:u8 = 100;
            let mut res:u8 = 0;
            let mut count:u16 = 0;
            for i in 0..=2 {
                res = value / divider;
                if res > 0 {
                    assert_eq!(res, cpu.memory.read(address + count));
                    value %= divider;
                    count += 1;
                } 
                divider /= 10;
            }
        }
    }


    #[cfg(test)]
    mod CpuVariantInstructionsForCosmac {

        use crate::core::cpu::Cpu;
        use crate::core::address;
        use crate::core::registers::Register;
        use crate::core::cpu::CpuVariant;
        use crate::core::cpu::CpuVariantInstructions;
        use crate::core::cpu::Cosmac;
        use crate::core::display::Display;
        use crate::core::keyboard::Keyboard;
        use crate::core::timers::Timers;

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
              #[test]
              fn opcode_8XY6() {
                let mut value:u8 = 255;
                let instruction:u16 = 0x8346;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                                let mut cpu:Cpu<Cosmac> = Cpu::<Cosmac>::init(&mut display, &mut keyboard, &mut timers);             


                  cpu.registers.get(4).set(value);

                  cpu.opcode_8XY6(instruction);

                 let vx_val = cpu.registers.get(3).get();
                 
                assert_eq!(vx_val, value >> 1);
                assert_eq!(cpu.registers.get(0xf).get(), 1);

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
              #[test]
               fn opcode_8XYE() {
                let mut value:u8 = 255;
                let instruction:u16 = 0x8346;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                                let mut cpu:Cpu<Cosmac> = Cpu::<Cosmac>::init(&mut display, &mut keyboard, &mut timers);             


                  cpu.registers.get(4).set(value);

                  cpu.opcode_8XYE(instruction);

                 let vx_val = cpu.registers.get(3).get();
                 
                assert_eq!(vx_val, value << 1);
                assert_eq!(cpu.registers.get(0xf).get(), 1);
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
              #[test]
               fn opcode_BXNN() {

                let address:u16 = 0x200;
                let mut value:u8 = 5;
                let instruction:u16 = 0x8346;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                let mut cpu:Cpu<Cosmac> = Cpu::<Cosmac>::init(&mut display, &mut keyboard, &mut timers);             
                cpu.pc.set(address);
                cpu.registers.get(0).set(value);

                cpu.opcode_BXNN(instruction);


                let nnn:u16 = cpu.get_nnn(instruction);

                assert_eq!(nnn + value as u16, cpu.pc.get());

      
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
              #[test]
               fn opcode_FX55() {
                let address:u16 = 0x200;
                let mut val:u8 = 10;
                let instruction:u16 = 0xF355;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                let mut cpu:Cpu<Cosmac> = Cpu::<Cosmac>::init(&mut display, &mut keyboard, &mut timers);             
               cpu.index.set(address);
                cpu.registers.get(3).set(val);
                
                cpu.opcode_FX55(instruction);
      
                  for i in 0..val as u16 {

                    let mem:u8 = cpu.memory.read(address+i);
                    assert_eq!(mem,cpu.registers.get(i as u8).get());
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
              #[test]
               fn opcode_FX65() {

                let address:u16 = 0x200;
                let mut val:u8 = 10;
                let instruction:u16 = 0xF355;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                                let mut cpu:Cpu<Cosmac> = Cpu::<Cosmac>::init(&mut display, &mut keyboard, &mut timers);             
       
               cpu.index.set(address);
                cpu.registers.get(3).set(val);
                
                  for i in 0..val as u16 {
                    cpu.memory.write(address + i, val + i as u8 );
                  }

                  cpu.opcode_FX65(instruction);

                  for i in 0..val as u16 {
                    let mem:u8 = cpu.memory.read(address+i);
                    assert_eq!(mem,cpu.registers.get(i as u8).get());
                  }
              }
      
      
          }
          #[cfg(test)]
          mod CpuVariantInstructionsForSuper {

            use crate::core::cpu::Cpu;
            use crate::core::address;
            use crate::core::registers::Register;
            use crate::core::cpu::CpuVariant;
            use crate::core::cpu::CpuVariantInstructions;
            use crate::core::cpu::Cosmac;
            use crate::core::cpu::Super;
            use crate::core::display::Display;
            use crate::core::keyboard::Keyboard;
            use crate::core::timers::Timers;
      
      
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
              #[test]
              fn opcode_8XY6() {

                let address:u16 = 0x200;
                let mut value:u8 = 255;
                let instruction:u16 = 0x8346;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                let mut cpu:Cpu<Super> = Cpu::<Super>::init(&mut display, &mut keyboard, &mut timers);

                  cpu.registers.get(3).set(value);

                  cpu.opcode_8XY6(instruction);

                 let vx_val = cpu.registers.get(3).get();
                 
                assert_eq!(vx_val, value >> 1);
                assert_eq!(cpu.registers.get(0xf).get(), 1);

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
              #[test]
               fn opcode_8XYE() {

                let mut value:u8 = 255;
                let instruction:u16 = 0x8346;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                let mut cpu:Cpu<Super> = Cpu::<Super>::init(&mut display, &mut keyboard, &mut timers);

                  cpu.registers.get(3).set(value);

                  cpu.opcode_8XYE(instruction);

                 let vx_val = cpu.registers.get(3).get();
                 
                assert_eq!(vx_val, value << 1);
                assert_eq!(cpu.registers.get(0xf).get(), 1);
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
              #[test]
               fn opcode_BXNN() {

                let address:u16 = 0x200;
                let mut value:u8 = 5;
                let instruction:u16 = 0x8346;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                let mut cpu:Cpu<Super> = Cpu::<Super>::init(&mut display, &mut keyboard, &mut timers);                cpu.pc.set(address);
                cpu.registers.get(3).set(value);

                cpu.opcode_BXNN(instruction);


                let nnn:u16 = cpu.get_nnn(instruction);

                assert_eq!(nnn + value as u16, cpu.pc.get());
      
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
              #[test]
               fn opcode_FX55() {
                let address:u16 = 0x200;
                let mut val:u8 = 10;
                let instruction:u16 = 0xF355;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                let mut cpu:Cpu<Super> = Cpu::<Super>::init(&mut display, &mut keyboard, &mut timers);                
                cpu.index.set(address);
                cpu.registers.get(3).set(val);
                
                cpu.opcode_FX55(instruction);
      
                  for i in 0..val as u16 {

                    let mem:u8 = cpu.memory.read(address+i);
                    assert_eq!(mem,cpu.registers.get(i as u8).get());
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
              #[test]
               fn opcode_FX65() {
                let address:u16 = 0x200;
                let mut val:u8 = 10;
                let instruction:u16 = 0xF355;
                let mut display:Display = Display::init();
                let mut keyboard:Keyboard = Keyboard::init();
                let mut timers:Timers = Timers::init();
                let mut cpu:Cpu<Super> = Cpu::<Super>::init(&mut display, &mut keyboard, &mut timers);         
               cpu.index.set(address);
                cpu.registers.get(3).set(val);
                
                  for i in 0..val as u16 {
                    cpu.memory.write(address + i, val + i as u8 );
                  }

                  cpu.opcode_FX65(instruction);

                  for i in 0..val as u16 {
                    let mem:u8 = cpu.memory.read(address+i);
                    assert_eq!(mem,cpu.registers.get(i as u8).get());
                  }
              }
      
            }      
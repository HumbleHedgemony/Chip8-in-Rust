

   use std::cell::RefCell;
   use std::cell::RefMut;

   #[derive(Copy, Clone)]
   pub  struct Register {
        value:u8,
    }

    impl Register {

        pub fn init(value:u8) -> Register {
            return Register{value:value}
        }

        pub fn get(&self) -> u8 {
            return self.value;
        }

        pub fn set(&mut self, register:u8) {
            self.value = register;
        }

        pub fn increment(&mut self) {
            self.value += 1;
        }

        pub fn decrement(&mut self) {
            self.value -= 1;
        }

        pub fn least_significant_bit(&self) -> u8 {
            return (self.value & 0x1) as u8;
        }
        /*
        *  an address only uses 12 bits of the 16
        * 
        */
        pub fn most_significant_bit(&self) -> u8 {
            return ((self.value & 0x80) >> 7) as u8;
        }

        pub fn lower_nibble(&self) -> u8 {
            return (self.value & 0xf) as u8;
        }


        pub fn lower_mid_nibble(&self) -> u8 {
            return ((self.value & 0xF0) >> 4) as u8
        }


        pub fn is_equal(&self, value:u8) -> bool {
            return if self.value == value {
                true
            } else {
                false
            }
        }

        pub fn in_range(&self,min:u8, max:u8) -> bool {
            return if self.value >= min && self.value <= max {
                true
            } else {
                false
            }
        }
    }

   pub struct Registers {

        pub registers:[Register;16],

    }

    impl Registers {

        pub fn init() -> Registers {
            /* 
            let arr:[RefCell<Register>;16] = [
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0)),
            RefCell::new(Register::init(0))
            ]; */
            Registers{registers:[Register::init(0);16]}
        }

        pub fn get(&mut self, index:u8) -> & mut Register {
            return &mut self.registers[index as usize];
        }

        pub fn set(&mut self, index:u8, value:u8) {
            self.registers[index as usize].set(value);
        }

    }

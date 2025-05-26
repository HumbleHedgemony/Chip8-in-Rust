
    pub struct Address {
        address:u16,
    }

    impl Address {

        pub fn init(address:u16) -> Address {
            Address{address:address}
        }

        pub fn get(&self) -> u16 {
            return self.address;
        }
    
        pub fn get_12_bits(&self) -> u16 {
            return self.address & 0xfff;
        }

        pub fn set(&mut self, address:u16) {
            self.address = address;
        }

        pub fn increment(&mut self) {
            self.address += 1;
        }

        pub fn decrement(&mut self) {
            self.address -= 1;
        }

        pub fn least_significant_bit(&self) -> u8 {
            return (self.address & 0x1) as u8;
        }
        /*
        *  an address only uses 12 bits of the 16
        * 
        */
        pub fn most_significant_bit(&self) -> u8 {
            return ((self.address & 0x8000) >> 15) as u8;
        }

        pub fn upper_nibble(&self) -> u16 {
            return self.address & 0xf000;
        }
        
        pub fn lower_mid_nibble(&self) -> u16 {
            return self.address & 0x00f0;
        }

        pub fn lower_nibble(&self) -> u16 {
            return self.address & 0xf;
        }

        pub fn match_nibbles_HNNL(&self, instruction:u16) -> bool {
            let mut result:bool = false;
            result = ( instruction & 0xf000 ) == self.upper_nibble();
            return result && (self.address & 0xf) == self.lower_nibble();
        }

        pub fn match_nibbles_HNML(&self, instruction:u16) -> bool {
            let mut result:bool = false;
            result = ( instruction & 0xf000 ) == self.upper_nibble();
            result = result && (( instruction & 0x00F0 ) == self.lower_mid_nibble());
            return result && (self.address & 0xf) == self.lower_nibble();
        }

        pub fn is_equal(&self,value:u16) -> bool {
            return if self.address == value {
                true
            } else {
                false
            }
        }

        pub fn in_range(&self,min:u16, max:u16) -> bool {
            return if self.address >= min && self.address <= max {
                true
            } else {
                false
            }
        }
    }

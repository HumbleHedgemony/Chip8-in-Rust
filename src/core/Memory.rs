
use super::configuration::MEMORY_STARTING_ADDRESS;
use super::font::Font;
use super::address::Address;

    pub struct Memory {

       
        memory:[u8; Memory::MEMORY_SIZE], 
        fontAddress:(u16,u16),
        font:Font,
    }

    impl Memory {
        const MEMORY_SIZE:usize = 3584;   // 4096 - 0x200(512)
        const BEGINNING_ADDRESS:u16 = MEMORY_STARTING_ADDRESS;
        pub fn init() -> Memory {
            Memory {
                memory: [0; Memory::MEMORY_SIZE ],
                font:Font::init(),
                fontAddress:(0,0x1ff),
            }

        }
        #[inline]
        pub fn read(&self,address:u16) -> u8 {
            if address >= self.fontAddress.0 && address <= self.fontAddress.1 {
                return self.font.get(address);
            } else {
                return self.read_from_raw(address);
            }
        }
        #[inline]
        pub fn write(&mut self, address:u16, value:u8) {
            self.memory[(address - Memory::BEGINNING_ADDRESS) as usize] = value;
        }

        pub fn read_from_raw(&self, address:u16) -> u8 {
            return self.memory[(address - Memory::BEGINNING_ADDRESS) as usize];
        }

        pub fn read_from_address(&self, address:Address) -> u8 {
            return self.memory[address.get_12_bits() as usize];
        }

        pub fn write_from_address(&mut self, address:Address, value:u8) {
            self.memory[address.get_12_bits() as usize] = value;
        }

        pub fn reset(&mut self) {
            self.memory = [0; Memory::MEMORY_SIZE ];
        }

        pub fn write_rom(&mut self,rom:&[u8]) {
            if rom.len() <= self.memory.len() {
                self.reset();
                self.memory[0..rom.len()].copy_from_slice(rom);
            }
        }



    }
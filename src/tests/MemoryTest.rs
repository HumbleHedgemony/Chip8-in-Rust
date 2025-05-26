

#[cfg(test)]
mod memory_test {

    use crate::core::memory::Memory;


    #[test]
    fn read() {
        let mem:Memory = Memory::init();

        let address:u16 = 0x0;
        
        let res:u8 = mem.read(address);

        assert_eq!(res, 0xF0);

    }

    
    #[test]
    fn write() {
        let mut mem:Memory = Memory::init();
        let address:u16 = 0x200;
        let value:u8 = 0x0E;
        mem.write(address,value);

        let res:u8 = mem.read(address);

        assert_eq!(res, value);
    }



    #[test]
    fn write_rom() {
        let mut mem:Memory = Memory::init();
        let start_address:u16 = 0x200;
        let mut rom:[u8; 20] = [0; 20];

        for i in 0..rom.len() {
            rom[i] = i as u8;
        }

        mem.write_rom(&rom);

        assert_eq!(mem.read(start_address+3),3);
        assert_eq!(mem.read(start_address+10),10);
    }


    #[test]
    fn reset() {
        let mut mem:Memory = Memory::init();
        let start_address = 0x200;
        let mut rom:[u8; 20] = [0; 20];

        for i in 0..rom.len() {
            rom[i] = i as u8;
        }

        mem.write_rom(&rom);

        assert_eq!(mem.read(start_address+3),3);
        assert_eq!(mem.read(start_address+10),10);

        mem.reset();

        assert_eq!(mem.read(start_address+3),0);
        assert_eq!(mem.read(start_address+10),0);
    }



}
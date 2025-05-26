

#[cfg(test)]
mod Chip8Test {
    use crate::core::cpu::Cpu;
    use crate::core::cpu::CpuType;
    use crate::core::chip8::Chip8;
    use std::thread::current;
    use std::time::Duration;
    use std::time::Instant;
    use more_asserts as ma;
    use std::thread::sleep;

    #[test]
    pub fn super_cpu() {
        let chip8:Chip8 = Chip8::init_super();

        assert_eq!(chip8.cpu.get_cpu_type(), String::from("Super") );
    }

    #[test]
    pub fn cosmac_cpu() {
        let chip8:Chip8 = Chip8::init_cosmac();

        
        assert_eq!(chip8.cpu.get_cpu_type(), String::from("Cosmac") );
    }

    #[test]
    pub fn update_time() {
        let mut chip8:Chip8 = Chip8::init_super();

            sleep(Duration::from_millis(1000));
            let currentTime:Instant = Instant::now();
            chip8.currentTime = currentTime;
            chip8.update_time();
            let duration:u128 = chip8.currentTime.duration_since(chip8.previousTime).as_micros();
            ma::assert_gt!(duration,1000000 as u128);
            /* 
            let temp:Duration = self.currentTime.duration_since(self.previousTime) - Duration::from_micros(Chip8::TIME_PER_CYCLE_IN_MICROSECONDS as u64);
            self.previousTime = self.currentTime - temp;
            */

    }

    #[test]
    pub fn write_rom() {
        let mut chip8:Chip8 = Chip8::init_super();
        let mut value:[u8;3584] = [0;3584];
        for i in 0..3584 {
            value[i as usize] = (i % 8) as u8;
        }

        chip8.write_rom(&mut value);

        assert_eq!(value[0],0);
        assert_eq!(value[8],0);
        assert_eq!(value[17],1);

        assert_eq!(chip8.cpu.get_memory().read(0x200),0);
        assert_eq!(chip8.cpu.get_memory().read(0x208),0);
        assert_eq!(chip8.cpu.get_memory().read(0x209),1);
        assert_eq!(chip8.cpu.get_memory().read(0x20A),2);
        assert_eq!(chip8.cpu.get_memory().read(0x211),1);
    }




}
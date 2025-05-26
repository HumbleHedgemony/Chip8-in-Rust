#[cfg(test)]
mod cpu_dependencies_test {
    use crate::core::cpuDependencies::CpuDependencies;
    use crate::core::registers::Register;
    use crate::core::registers::Registers;
    use crate::core::address::Address;
    use crate::core::stack::Stack;
    use crate::core::display::Display;
    use crate::core::memory::Memory;
    use crate::core::keyboard::Keyboard;
    use crate::core::timers::Timers;
    


    #[test]
    pub fn get_x_register() {
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();

        let register:&mut Register = cpuDependencies.get_x_register(0x0f00);

        assert_eq!(register.is_equal(0), true);
    
    }

    #[test]
    pub fn carry_flag() {

        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        cpuDependencies.carry_flag(true);
        let register:&mut Register = cpuDependencies.get_x_register(0x0f00);
        assert_eq!(register.is_equal(1), true);
    }

    #[test]
    pub fn get_index(){
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();

        let register:&mut Address = cpuDependencies.get_index();
        assert_eq!(register.is_equal(0), true);
    }

    #[test]
    pub fn get_pc() {
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();

        let register:&mut Address = cpuDependencies.get_pc();
        assert_eq!(register.is_equal(0x200), true);
    }

    #[test]
    pub fn get_stack(){
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();

        let stack:&mut Stack = cpuDependencies.get_stack();

        stack.push(15);

        assert_eq!(stack.pop(),15);
    }

    #[test]
    pub fn get_registers() {
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        cpuDependencies.carry_flag(true);

        let registers:&mut Registers = cpuDependencies.get_registers();
        let register:&mut Register = registers.get(15);

        assert_eq!(register.is_equal(1), true);
    }

    #[test]
    pub fn get_memory(){
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        let memory:&mut Memory = cpuDependencies.get_memory();
        
        let val:u8 = memory.read(0);

        assert_eq!(val,0xF0);
        
    }

    #[test]
    pub fn get_display() {
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        let display: &mut Display = cpuDependencies.get_display();

        display.turn_on(0,0);

        assert_eq!(display.get_pixel(0,0),true);
    }

    #[test]
    pub fn get_keyboard() {
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        let keyboard:&mut Keyboard = cpuDependencies.get_keyboard();

        keyboard.f();

        assert_eq!(keyboard.get_key(),15);
    }
    
    #[test]
    pub fn get_timers(){
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        let timers:&mut Timers = cpuDependencies.get_timers();

        timers.set_sound(8);

        assert_eq!(timers.get_sound(),8);
    }
}
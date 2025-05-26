use crate::core::address::Address;
use super::registers;
use super::registers::Register;
use super::registers::Registers;
use super::memory::Memory;
use super::stack::Stack;
use super::display::Display;
use super::keyboard::Keyboard;
use super::timers::Timers;
use super::configuration;



pub struct CpuDependencies {
    pub index:Address,
    pub pc:Address,
    pub stack:Stack,
    pub registers:Registers,
    pub memory:Memory,
    pub display:Display,
    pub keyboard:Keyboard,
    pub timers:Timers,

}

impl CpuDependencies {

    pub fn init() -> CpuDependencies {

        return CpuDependencies {
            index: Address::init(0),
            pc: Address::init(configuration::MEMORY_STARTING_ADDRESS),
            stack: Stack::init(),
            registers: Registers::init(),
            memory: Memory::init(),
            display: Display::init(),
            keyboard: Keyboard::init(),
            timers: Timers::init(),
        };
    }

    pub fn reset() -> CpuDependencies {
        return Self::init();
    }

    pub fn carry_flag(&mut self, isOverflowed:bool) {
        if (isOverflowed) {
            let mut overflowRegister:&mut Register = self.registers.get(15);
            overflowRegister.set((1 as u8));
        }
    }

    pub fn get_x_register(&mut self,instruction:u16) -> & mut Register {
        let registerIndexX:u8 = ((instruction & 0xf00) >> 8) as u8;
        return self.registers.get(registerIndexX);
       
    }

    pub fn get_index(&mut self) -> & mut Address {
        return &mut self.index;
    }

    pub fn get_pc(&mut self) -> & mut Address {
        return &mut self.pc;
    }

    pub fn get_stack(&mut self) ->  & mut Stack {
        return &mut self.stack;
    }

    pub fn get_registers(&mut self) -> & mut Registers {
        return &mut self.registers;
    }

    pub fn get_memory(&mut self) -> & mut Memory {
        return &mut self.memory;
    }

    pub fn get_display(&mut self) -> & mut Display {
        return  &mut self.display;
    }

    pub fn get_keyboard(&mut self) -> & mut Keyboard {
        return &mut self.keyboard;
    }

    pub fn get_timers(&mut self) -> & mut Timers {
        return &mut self.timers;
    }

}



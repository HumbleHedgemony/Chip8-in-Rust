#![feature(get_mut_unchecked)]

use std::time::{Instant, Duration};

use super::registers::Register;
use super::registers::Registers;
use super::memory::Memory;
use super::stack::Stack;
use super::display::Display;
use super::keyboard::Keyboard;
use super::timers::Timers;
use super::cpu::CpuType;
use super::cpu::Cpu;
use super::cpu::CpuVariant;
use super::cpu::CpuVariantInstructions;
use super::cpu::Cosmac;
use super::cpu::Super;
use super::cpu::CpuExtendedInstructions;
use std::marker::PhantomData;
use super::cpu::CpuInstructions;

/* 
impl Into<Option<&dyn CpuExtendedInstructions>> for Option<Cpu<'_>> {

    fn into(&mut self) -> Option<&dyn CpuExtendedInstructions> {
        self.as_ref().map(|x| x as &dyn CpuExtendedInstructions);
    }

}
*/


pub struct Chip8 {

    pub cpu:Box<dyn CpuExtendedInstructions>,



   // 700 cycles per second 1000000us per second / 700 cycles per second

    pub previousTime:Instant,
    pub currentTime:Instant,

    pub exit:bool,

}


impl Chip8 {

    const TIME_PER_CYCLE_IN_MICROSECONDS:u128 = 1429;

    pub fn init_super() -> Chip8 {
        
        Chip8 {
            cpu:Box::new(Cpu::<Super>::init()),
            previousTime:Instant::now(),
            currentTime:Instant::now(),
            exit:false,

        }
    }   


    pub fn init_cosmac() -> Chip8 {
        
        Chip8 {
            cpu:Box::new(Cpu::<Cosmac>::init()),
            previousTime:Instant::now(),
            currentTime:Instant::now(),
            exit:false,
        }
    }   

    pub fn super_cpu(&mut self) {
        self.cpu = Box::new(Cpu::<Super>::init());
    }
 
    pub fn cosmac_cpu(&mut self) {
        self.cpu = Box::new(Cpu::<Cosmac>::init());

    }

    pub fn run(&mut self) {
        while self.exit == false {
            self.currentTime = Instant::now();
            let currentTime = self.currentTime;
            if currentTime.duration_since(self.previousTime).as_micros() >= Chip8::TIME_PER_CYCLE_IN_MICROSECONDS {
                self.cpu.tick();
                self.cpu.get_timers().update(currentTime);
                self.update_time();

            }
        }
    }

    pub fn update_time(&mut self) {
        let temp:Duration = self.currentTime.duration_since(self.previousTime) - Duration::from_micros(Chip8::TIME_PER_CYCLE_IN_MICROSECONDS as u64);
        self.previousTime = self.currentTime - temp;
    }

    pub fn get_keyboard(&mut self) -> &mut Keyboard {
        return self.cpu.get_keyboard();
    }

    pub fn get_display(&mut self) -> &mut Display {
        return self.cpu.get_display();
    }

    pub fn get_timers(&mut self) -> &mut Timers {
        return self.cpu.get_timers();
    }

    pub fn write_rom(&mut self,data:&[u8]) {
  
        self.cpu.get_memory().write_rom(&data);
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

}

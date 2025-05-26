//Lib.rs
#![crate_name = "chip8"]
#![crate_type = "lib" ]
#![allow(warnings)] 

mod core {
    // tested no
    // pub mod Chip8;

    pub mod configuration;


    // tested - yes - 13-11-2024
    pub mod registers;

    // tested - yes - 13-11-2024
    pub mod display;

    // tested - yes - 13-11-2024
    pub mod timers;

    // tested - yes - 13-11-2024
    pub mod stack;

    // tested - yes - 14-11-2024
    pub mod keyboard;

    // tested - yes - 14-11-2024 */
    pub mod font;

    // tested - yes - 14-11-2024*/
    pub mod address;

    // tested yes - 18-11-2024
    pub mod memory;
    
    // tested yes - 18-11-2024
    pub mod cpuDependencies;

    //pub mod cpuvariants;

    pub mod cpu;

    pub mod chip8;



}

#[cfg(test)]
pub mod tests;


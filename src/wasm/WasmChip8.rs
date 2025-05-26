use super::Chip8::Chip8;

#[wasm_bindgen]
struct WasmChip8 {
    chip8:Chip8,
}

#[wasm_bindgen]
impl WasmChip8 {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmChip8 {
        WasmChip8 {chip8: Chip8::init_super()}
    }

    pub fn set_super(&mut self) {
        self.chip8 = Chip8::init_super();
    }

    pub fn set_cosmac(&mut self) {
        self.chip8 = Chip8::init_cosmac();
    }
    /*
     *
     * 
     * timestamp: i64(double)
     *   DOMHighResTimeStamp passed to rust
     */
    pub fn tick(&mut self, timestamp:i64) {
        self.chip8.cpu.tick();

        self.currentTime = Instant::now();
        let currentTime = self.currentTime;

        self.cpu.get_timers().update(currentTime);
        self.update_time();
    }

    pub fn get_sound(&self) -> u8 {
        self.chip8.get_timers().get_sound()
    }

    pub fn get_delay(&self) -> u8 {
        self.chip8.get_timers().get_delay()
    }

    pub fn get_display(&self) -> [[bool;64];32] {
        self.chip8.get_display().get_screen()
    }

    pub fn set_key(&mut self, key:u8) {
        self.chip8.get_keyboard().press(key);
    }

    pub fn write_rom(&mut self, game:[u8]) {
        self.chip8.write_rom(&game);
    }

}
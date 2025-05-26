

    pub struct Display {
        /**
         * 64 pixels wide
         * 32 pixels tall
         * 
         * array[32][64] [[false,false,false,.....],
         *                [false,false,false,.....],
         *                 ......                    
         *               ]
         * 
         */
        pixels:[[bool;64];32],

  

    }
    impl Display {
        // const X_DIMENSION:u8 = 64;
        // const Y_DIMENSION:u8 = 32;

        pub fn init() -> Display {
            return Display{pixels:[[false;64];32]};
        }
        pub fn set(&mut self, value:bool, x:u8, y:u8) {
            self.pixels[y as usize][x as usize] = value;
        }

        pub fn turn_off(&mut self, x:u8, y:u8) {
            self.set(false,x,y);
        }

        pub fn turn_on(&mut self, x:u8, y:u8) {
            self.set(true,x,y);
        }

        #[inline]
        pub fn set_pixel(&mut self, bit:bool, x:u8, y:u8) -> bool {
            let value:bool = self.get_pixel(x,y);
            return if bit && value { // if incoming value and current pixel is true, then turn it off and notify cpu
                self.turn_off(x,y);
                false
            } else {
                self.set(bit,x,y);
                true
            }
        }
        pub fn toggle(&mut self, x:u8, y:u8) -> bool {
            let value:bool = !self.get_pixel(x,y);
            self.set(value, x , y);
            return value;
        }

        pub fn get_row(&self, y:u8) -> [bool;64]  {
            return self.pixels[y as usize];
        }

        pub fn get_pixel(&self, x:u8, y:u8) -> bool {
            return self.pixels[y as usize][x as usize];
        }

        pub fn get_screen(&self) -> [[bool;64];32] {
            return self.pixels;
        }

        pub fn clear_screen(&mut self) {
            for row in 0..32 {
                for column in 0..64 {
                    self.set(false,column,row);
                }
            }
        }

    }

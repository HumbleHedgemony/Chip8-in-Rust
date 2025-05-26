
    pub struct Stack {
        size:u8,
        max:u8,
        stack:[u16;16],
    }

    impl Stack {

        pub fn init() -> Stack {
                Stack{size:0,max:16,stack:[0;16]}
        }

        pub fn push(&mut self, value:u16) {
            if self.size != self.max {
                self.stack[self.size as usize] = value;
                self.size += 1;
            }
        }

        pub fn pop(&mut self) -> u16 {
            let mut address:u16 = 0;
            if self.size > 0 {
                self.size -= 1;
                address = self.stack[self.size as usize];
                self.stack[self.size as usize] = 0;
            } 
            address
        }

        pub fn length(&self) -> u8 {
            return self.size;
        }
    }

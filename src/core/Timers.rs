
    use std::time::{Instant, Duration};
    use std::ops::Add;
   pub  struct Timers {

        /*
        * timers are 1 byte (4 bit) in size
        * 8 bit unsigned int is the smallest type possible
        * 
        * @note
        *      i could use a single u8 to run both delay and sound timer.
        *      but that would requirebitmasking, and little bit of math
        *      that isn't worth it for 8 bits of memory. 
        */
        delay:u8,
        sound:u8,

        /*
        * timers update 60 times per second(60hz)
        * 1000000 microseconds in a second
        * 60 updates per second
        * 
        * 1000000 / 60 = 16667 microseconds per update
        */
        


        /*
        * the last time timer was updated.
        * 
        * */
        soundPreviousUpdate:Instant,
        delayPreviousUpdate:Instant,




    }


    impl Timers {
    
        const MICROSECONDS_PER_TIMER_UPDATE:u128 = 16667;

        pub fn init() -> Timers {
            Timers{delay:0,sound:0,soundPreviousUpdate:Instant::now(),delayPreviousUpdate:Instant::now()}
        }
        pub fn set_sound(&mut self, value:u8) {
            self.sound = value;
            self.soundPreviousUpdate = Instant::now();
        }

       pub fn get_sound(&self) -> u8 {
            return self.sound;
        }

        pub fn set_delay(&mut self, value:u8 ) {
            self.delay = value;
            self.delayPreviousUpdate = Instant::now();
        }

        pub fn get_delay(&self) -> u8 {
            return self.delay;
        }

        pub fn update(&mut self, time:Instant) {
            self.update_delay(time);
            self.update_sound(time);
        }

        pub fn update_delay(&mut self, time:Instant ) {
            if self.delay == 0 {
                return;
            }

            let microsecondsDifference:Duration = time.duration_since(self.delayPreviousUpdate);
            let res:u128 = microsecondsDifference.as_micros()/Timers::MICROSECONDS_PER_TIMER_UPDATE;
            let leftOver:u128 = microsecondsDifference.as_micros() % Timers::MICROSECONDS_PER_TIMER_UPDATE;

            if res > 0 {
                self.delay -= 1;
            }
            self.delayPreviousUpdate = time.add(
                    Duration::from_micros(
                        (microsecondsDifference.as_micros() - leftOver) as u64
                                        ) );

        }

        pub fn update_sound(&mut self, time:Instant) {
            if self.sound == 0 {
                return;
            }

            let microsecondsDifference:Duration = time.duration_since(self.soundPreviousUpdate);
            let res:u128 = microsecondsDifference.as_micros()/Timers::MICROSECONDS_PER_TIMER_UPDATE;
            let leftOver:u128 = microsecondsDifference.as_micros() % Timers::MICROSECONDS_PER_TIMER_UPDATE;

            if res > 0 {
                self.sound -= res as u8;
            }
            self.delayPreviousUpdate = time.add(
                Duration::from_micros(
                (microsecondsDifference.as_micros() - leftOver) as u64
                                    )
            );

        }
    }
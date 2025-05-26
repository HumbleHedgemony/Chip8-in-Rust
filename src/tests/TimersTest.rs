
#[cfg(test)]
mod TimersTest {
    use std::time::{Instant, Duration};
    use crate::core::timers::Timers;

    #[test]
    fn set_sound() {
        let mut timers:Timers = Timers::init();

        timers.set_sound(8);

        assert_eq!(timers.get_sound(),8);
    }


    #[test]
    fn set_delay() {
        let mut timers:Timers = Timers::init();

        timers.set_delay(8);

        assert_eq!(timers.get_delay(),8);
    }


    #[test]
    fn update_delay() {
        let mut timers:Timers = Timers::init();
        timers.set_delay(8);
        let mut time:Instant = Instant::now();
        let addDuration:Duration = Duration::from_micros(16667);
        let newTime:Option<Instant> = time.checked_add(addDuration);

        if let Some(t) = newTime {
            timers.update_delay(t);
            assert_eq!(timers.get_delay(),7);
        } else {
            println!("assert");
            assert!(true, "failed to get updated time for delay.");
        }
    }
    #[test]
    fn update_sound() {
        let mut timers:Timers = Timers::init();
        timers.set_sound(8);
        let mut time:Instant = Instant::now();
        let addDuration:Duration = Duration::from_micros(16667*2);
        let newTime:Option<Instant> = time.checked_add(addDuration);

        if let Some(t) = newTime {
            timers.update_sound(t);
            assert_eq!(timers.get_sound(),6);
        } else {
            assert!(true, "failed to get updated time for sound.");
        }
    }

    #[test]
    fn update_sound2() {
        let mut timers:Timers = Timers::init();
        timers.set_sound(8);
        let mut time:Instant = Instant::now();
        let addDuration:Duration = Duration::from_micros(16666*2);
        let newTime:Option<Instant> = time.checked_add(addDuration);

        if let Some(t) = newTime {
            timers.update_sound(t);
            assert_eq!(timers.get_sound(),7);
        } else {
            assert!(true, "failed to get updated time for sound.");
        }
    }

}
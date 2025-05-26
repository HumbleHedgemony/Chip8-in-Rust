

mod display_test {
    use crate::core::display::Display;

    #[test]
    pub fn set() {
        let mut display:Display = Display::init();

        display.set(true,1,1);

        assert_eq!(display.get_pixel(1,1),true);
    }
    #[test]
    pub fn turn_on() {
        let mut display:Display = Display::init();

        display.turn_on(2,1);

        assert_eq!(display.get_pixel(2,1),true);
    }
    #[test]
    pub fn turn_off() {
        let mut display:Display = Display::init();

        display.turn_on(2,1);

        assert_eq!(display.get_pixel(2,1),true);

        display.turn_off(2,1);

        assert_eq!(display.get_pixel(2,1),false);
    }
    #[test]
    pub fn set_pixel() {
        let mut display:Display = Display::init();

        assert_eq!(display.set_pixel(true,2,1), true);
        assert_eq!(display.get_pixel(2,1), true);

        assert_eq!(display.set_pixel(true,2,1),false);
        assert_eq!(display.get_pixel(2,1), false);

    }
    #[test]
    pub fn toggle() {
        let mut display:Display = Display::init();

        assert_eq!(display.toggle(2,1),true);
        assert_eq!(display.get_pixel(2,1),true);
        assert_eq!(display.toggle(2,1),false);
        assert_eq!(display.get_pixel(2,1),false);
    }
    #[test]
    pub fn get_row() {

        let mut display:Display = Display::init();
        display.turn_on(0,0);
        display.turn_on(1,0);
        display.turn_on(2,0);
        display.turn_on(3,0);
        display.turn_on(4,0);
        let row:[bool;64] = display.get_row(0);

        for i in 0..=4 {
            assert_eq!(row[i as usize],true);
        }

    }
    #[test]
    pub fn get_screen() {
        let mut display:Display = Display::init();
        display.turn_on(0,0);
        display.turn_on(1,0);
        display.turn_on(2,0);
        display.turn_on(3,0);
        display.turn_on(4,0);

       let screen:[[bool;64];32] = display.get_screen();

       for i in 0..=4 {
            assert_eq!(screen[0][i],true);
        }

    }
    #[test]
    pub fn clear_screan() {
        let mut display:Display = Display::init();
        display.turn_on(0,0);
        display.turn_on(1,0);
        display.turn_on(2,0);
        display.turn_on(3,0);
        display.turn_on(4,0);
        let mut row:[bool;64] = display.get_row(0);

        for i in 0..=4 {
            assert_eq!(row[i],true);
        }

        display.clear_screen();

        row = display.get_row(0);

        for i in 0..=4 {
            assert_eq!(row[i as usize],false);
        }
    }
}
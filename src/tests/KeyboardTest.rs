
// Keyboard test

#[cfg(test)]
mod KeyboardTest {
    use crate::core::keyboard::Keyboard;

    #[test]
    fn null() {
        
        let mut keyboard:Keyboard = Keyboard::init();
        
        assert_eq!(keyboard.key_is_null(), true);

    }


    #[test]
    fn one() {
        let mut keyboard:Keyboard = Keyboard::init();
        
        keyboard.one();
        assert_eq!(keyboard.get_key(), 1);

    }
    #[test]
    fn zero() {
        let mut keyboard: Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.zero();
        assert_eq!(keyboard.get_key(), 0);

    }


    #[test]
    fn two() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.two();
        assert_eq!(keyboard.get_key(), 2);

    }

    #[test]
    fn three() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.three();
        assert_eq!(keyboard.get_key(), 3);

    }

    #[test]
    fn four() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.four();
        assert_eq!(keyboard.get_key(), 4);

    }

    #[test]
    fn five() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.five();
        assert_eq!(keyboard.get_key(), 5);

    }

    #[test]
    fn six() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.six();
        assert_eq!(keyboard.get_key(), 6);

    }

    #[test]
    fn seven() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.seven();
        assert_eq!(keyboard.get_key(), 7);

    }

    #[test]
    fn eight() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.eight();
        assert_eq!(keyboard.get_key(), 8);

    }

    #[test]
    fn nine() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.nine();
        assert_eq!(keyboard.get_key(), 9);

    }

    #[test]
    fn a() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.a();
        assert_eq!(keyboard.get_key(), 10);

    }

    #[test]
    fn b() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.b();
        assert_eq!(keyboard.get_key(), 11);

    }

    #[test]
    fn c() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.c();
        assert_eq!(keyboard.get_key(), 12);

    }


    #[test]
    fn d() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.d();
        assert_eq!(keyboard.get_key(), 13);

    }


    #[test]
    fn e() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.e();
        assert_eq!(keyboard.get_key(), 14);

    }


    #[test]
    fn f() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        keyboard.f();
        assert_eq!(keyboard.get_key(), 15);

    }

    #[test]
    fn key_up() {
        let mut keyboard:Keyboard = Keyboard::init();

        keyboard.one();
        assert_eq!(keyboard.get_key(), 1);

        keyboard.key_up();

        assert_eq!(keyboard.key_is_null(),true);

    }

}
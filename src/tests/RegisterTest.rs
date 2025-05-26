
#[cfg(test)]
mod RegisterTest {
    use crate::core::registers::Register;

    #[test]
    fn get() {
        let value1:u8 = 0xf;
        let mut register:Register = Register::init(value1);

        assert_eq!(register.get(), value1);
        assert_ne!(register.get(),0xA);

    }

    #[test]
    fn set() {
        let value1:u8 = 0xf;
        let mut register:Register = Register::init(value1);
        let value2:u8 = 0xDC;

        register.set(value2);

        assert_eq!(register.get(), value2);
        assert_ne!(register.get(),value1);

    }

    #[test]
    fn is_equal() {
        let value1:u8 = 0xf;
        let mut register:Register = Register::init(value1);

        assert_eq!(register.is_equal(0xf),true);
        assert_eq!(register.is_equal(0xE),false);
    }

    #[test]
    fn in_range() {
        let value1:u8 = 0xf;
        let mut register:Register = Register::init(value1);

        assert_eq!(register.in_range(0xC, 0x10),true);
        assert_eq!(register.in_range(0x0,0xE),false);
    }

    #[test]
    fn increment() {
        let value1:u8 = 0xCF;
        let mut register:Register = Register::init(value1);
        register.increment();

        assert_eq!(register.get(),0xD0);
        assert_ne!(register.get(),0x0);

        register.increment();

        assert_eq!(register.get(),0xD1);
        assert_ne!(register.get(),0x0);
    }

    #[test]
    fn decrement() {
        let value1:u8 = 0xCF;
        let mut register:Register = Register::init(value1);
        register.decrement();

        assert_eq!(register.get(),0xCE);
        assert_ne!(register.get(),0x0);

        register.decrement();

        assert_eq!(register.get(),0xCD);
        assert_ne!(register.get(),0x0);
    }

    #[test]
    fn lower_mid_nibble() {
        let value1:u8 = 0xCF;
        let mut register:Register = Register::init(value1);

        assert_eq!(register.lower_mid_nibble(),0xC);
        assert_ne!(register.lower_mid_nibble(),0xA);
    }


    #[test]
    fn most_significant_bit() {
        let value1:u8 = 0xAF;
        let mut register:Register = Register::init(value1);

        assert_eq!(register.most_significant_bit(),0x1);
        assert_ne!(register.most_significant_bit(),0x2);
    }

    #[test]
    fn least_significant_bit() {
        let value1:u8 = 0xAF;
        let mut register:Register = Register::init(value1);

        assert_eq!(register.least_significant_bit(),0x1);
        assert_ne!(register.least_significant_bit(),0x0);
    }


}
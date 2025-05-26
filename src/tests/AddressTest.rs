#[cfg(test)]
mod AddressTest {
    use crate::core::address::Address;

    #[test]
    fn get() {
        let value1:u16 = 0xf;
        let mut address:Address = Address::init(value1);

        assert_eq!(address.get(), value1);
        assert_ne!(address.get(),0xA);

    }

    #[test]
    fn set() {
        let value1:u16 = 0xf;
        let mut address:Address = Address::init(value1);
        let value2:u16 = 0xEDC;

        address.set(value2);

        assert_eq!(address.get(), value2);
        assert_ne!(address.get(),value1);

    }

    #[test]
    fn is_equal() {
        let value1:u16 = 0xf;
        let mut address:Address = Address::init(value1);

        assert_eq!(address.is_equal(0xf),true);
        assert_eq!(address.is_equal(0xE),false);
    }

    #[test]
    fn in_range() {
        let value1:u16 = 0xf;
        let mut address:Address = Address::init(value1);

        assert_eq!(address.in_range(0xC, 0x10),true);
        assert_eq!(address.in_range(0x0,0xE),false);
    }

    #[test]
    fn lower_nibble() {
        let value1:u16 = 0xCF;
        let mut address:Address = Address::init(value1);

        assert_eq!(address.lower_nibble(),0xF);
        assert_ne!(address.lower_nibble(),0xC);
    }

    #[test]
    fn upper_nibble() {
        let value1:u16 = 0xA0CF;
        let mut address:Address = Address::init(value1);

        assert_eq!(address.upper_nibble(),0xA000);
        assert_ne!(address.upper_nibble(),0xC000);
    }

    #[test]
    fn lower_mid_nibble() {
        let value1:u16 = 0xA0CF;
        let mut address:Address = Address::init(value1);

        assert_eq!(address.lower_mid_nibble(),0xC0);
        assert_ne!(address.lower_mid_nibble(),0xA);
    }


    #[test]
    fn most_significant_bit() {
        let value1:u16 = 0xA0CF;
        let mut address:Address = Address::init(value1);

        assert_eq!(address.most_significant_bit(),0x1);
        assert_ne!(address.most_significant_bit(),0x2);
    }

    #[test]
    fn least_significant_bit() {
        let value1:u16 = 0xA0CF;
        let mut address:Address = Address::init(value1);

        assert_eq!(address.least_significant_bit(),0x1);
        assert_ne!(address.least_significant_bit(),0x0);
    }

    #[test]
    fn increment() {
        let value1:u16 = 0xA0CF;
        let mut address:Address = Address::init(value1);
        address.increment();

        assert_eq!(address.get(),0xA0D0);
        assert_ne!(address.get(),0x0);

        address.increment();

        assert_eq!(address.get(),0xA0D1);
        assert_ne!(address.get(),0x0);
    }

    #[test]
    fn decrement() {
        let value1:u16 = 0xA0CF;
        let mut address:Address = Address::init(value1);
        address.decrement();

        assert_eq!(address.get(),0xA0CE);
        assert_ne!(address.get(),0x0);

        address.decrement();

        assert_eq!(address.get(),0xA0CD);
        assert_ne!(address.get(),0x0);
    }

    #[test]
    fn match_nibbles_HNML() {
        let value1:u16 = 0xA0CF;
        let mut address:Address = Address::init(value1);

        assert_eq!( address.match_nibbles_HNML(0xA1CF),true);
        assert_ne!(address.match_nibbles_HNML(0xA1CE),false);

    }

    #[test]
    fn match_nibbles_HNNL() {
        let value1:u16 = 0xA0CF;
        let mut address:Address = Address::init(value1);

        assert_eq!( address.match_nibbles_HNNL(0xA1EF),true);
        assert_ne!(address.match_nibbles_HNNL(0xA1EE),false);

    }


}

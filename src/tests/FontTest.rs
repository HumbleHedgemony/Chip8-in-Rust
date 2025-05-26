
// Font test

#[cfg(test)]
mod FontTest {
    use crate::core::font::Font;

    #[test]
    fn get_check() {
        let mut font:Font = Font::init();

        let value:u8 = font.get(3);
        assert_eq!(value, 0x90);

        let value2:u8 = font.get(10);
        assert_eq!(value2,0xF0);
    }


}
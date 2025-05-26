#[cfg(test)]
mod stack_test {
    use crate::core::stack::Stack;
    #[test]
    fn push_check() {
        let mut stack:Stack = Stack::init();
        
        let value1:u16 = 0xf;
        let value2:u16 = 0xA;

        stack.push(value1);
        assert_eq!(stack.length(), 1);
        
        stack.push(value2);
        assert_eq!(stack.length(),2);


    }

    #[test]
    fn pop_check() {
        let mut stack:Stack = Stack::init();
        
        let value1:u16 = 0xf;
        let value2:u16 = 0xA;

        stack.push(value1);
        assert_eq!(stack.length(), 1);
        
        stack.push(value2);
        assert_eq!(stack.length(),2);

        assert_eq!(value2, stack.pop());
        assert_eq!(value1, stack.pop());
    }
}
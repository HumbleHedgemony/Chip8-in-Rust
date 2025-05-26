#[derive( PartialEq, Eq)] 
enum Key {
    Null = -1,
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15
}
trait ToValue<T> {
    fn value(&self) -> T;
}

impl ToValue<i8> for Key {
    fn value (&self) -> i8 {
        let res:i8 = match self {
            Key::Null => -1,
            Key::Zero => 0,
            Key::One => 1,
            Key::Two => 2,
            Key::Three => 3,
            Key::Four => 4,
            Key::Five => 5,
            Key::Six => 6,
            Key::Seven => 7,
            Key::Eight => 8,
            Key::Nine => 9,
            Key::A => 10,
            Key::B => 11,
            Key::C => 12,
            Key::D => 13,
            Key::E => 14,
            Key::F => 15,
            _ => -20
        };
        return res;
    }
}

enum State {
    Down,
    Up,
    Null
}

    pub struct Keyboard {

        currentKey:Key,
        keyState:State,
    }


    impl Keyboard {


        pub fn init() -> Keyboard {
            Keyboard{currentKey:Key::Null,keyState:State::Null}
        }

        pub fn press(&mut self, key:u8) {
            match key {

                0 => self.zero(),
                1 => self.one(),
                2 => self.two(),
                3 => self.three(),
                4 => self.four(),
                5 => self.five(),
                6 => self.six(),
                7 => self.seven(),
                8 => self.eight(),
                9 => self.nine(),
                0xA => self.a(),
                0xb => self.b(),
                0xc => self.c(),
                0xd => self.d(),
                0xe => self.e(),
                0xf => self.f(),
                _ => {}
            }
        }

        pub fn zero(&mut self) {
            self.currentKey = Key::Zero;
            self.keyState = State::Down;
        }

        pub fn one(&mut self) {
            self.currentKey = Key::One;
            self.keyState = State::Down;
        }


        pub fn two(&mut self) {
            self.currentKey = Key::Two;
            self.keyState = State::Down;
        }



        pub fn three(&mut self) {
            self.currentKey = Key::Three;
            self.keyState = State::Down;
        }

        pub fn four(&mut self) {
            self.currentKey = Key::Four;
            self.keyState = State::Down;
        }


        pub fn five(&mut self) {
            self.currentKey = Key::Five;
            self.keyState = State::Down;
        }


        pub fn six(&mut self) {
            self.currentKey = Key::Six;
            self.keyState = State::Down;
        }

        pub fn seven(&mut self) {
            self.currentKey = Key::Seven;
            self.keyState = State::Down;
        }


        pub fn eight(&mut self) {
            self.currentKey = Key::Eight;
            self.keyState = State::Down;
        }

        pub fn nine(&mut self) {
            self.currentKey = Key::Nine;
            self.keyState = State::Down;
        }



        pub fn a(&mut self) {
            self.currentKey = Key::A;
            self.keyState = State::Down;
        }

        pub fn b(&mut self) {
            self.currentKey = Key::B;
            self.keyState = State::Down;
        }


        pub fn c(&mut self) {
            self.currentKey = Key::C;
            self.keyState = State::Down;
        }


        pub fn d(&mut self) {
            self.currentKey = Key::D;
            self.keyState = State::Down;
        }

        pub fn e(&mut self) {
            self.currentKey = Key::E;
            self.keyState = State::Down;
        }


        pub fn f(&mut self) {
            self.currentKey = Key::F;
            self.keyState = State::Down;
        }

        pub fn key_up(&mut self) {
            self.currentKey = Key::Null;
            self.keyState = State::Null;
        }

        pub fn get_key(&mut self) -> i8 {
            return self.currentKey.value();
        }

        pub fn key_is_null(&self) -> bool {
            return self.currentKey.eq(&Key::Null);
        }

    }

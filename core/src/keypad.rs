const ROW0_FLAG: u8 = 0x10;
const ROW1_FLAG: u8 = 0x20;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

#[derive(PartialEq)]
pub enum KeyEvent {
    Press(Key),
    Release(Key),
}

pub struct Keypad {
    data: u8,
    row0: u8,
    row1: u8,
    pub interrupt: u8,
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            data: 0xFF,
            row0: 0x0F,
            row1: 0x0F,
            interrupt: 0x00,
        }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, value: u8) {
        let mask = ROW0_FLAG | ROW1_FLAG;
        self.data = (self.data & !mask) | (value & mask);
        self.update();
    }

    fn update(&mut self) {
        let old = self.data & 0xF;
        let mut new = 0xF;

        // let new = 0xF & ((!self.data & ROW0_FLAG) >> ROW0_FLAG.trailing_zeros() & self.row0)
        //      & ((!self.data & ROW1_FLAG) >> ROW1_FLAG.trailing_zeros() & self.row1);

        if self.data & ROW0_FLAG == 0 {
            new &= self.row0;
        }
        if self.data & ROW1_FLAG == 0 {
            new &= self.row1;
        }

        if old == 0xF && new != 0xF {
            self.interrupt |= 0x10;
        }

        self.data = (self.data & 0xF0) | new;
    }

    /**
     * is_pressed
     */
    pub fn is_pressed(&self, key: Key) -> bool {
        match key {
            Key::Right => self.row0 & 0b0001 == 0,
            Key::Left => self.row0 & 0b0010 == 0,
            Key::Up => self.row0 & 0b0100 == 0,
            Key::Down => self.row0 & 0b1000 == 0,
            Key::A => self.row1 & 0b0001 == 0,
            Key::B => self.row1 & 0b0010 == 0,
            Key::Select => self.row1 & 0b0100 == 0,
            Key::Start => self.row1 & 0b1000 == 0,
        }
    }

    pub fn press(&mut self, key: Key) {
        match key {
            Key::Right => self.row0 &= 0b1110,
            Key::Left => self.row0 &= 0b1101,
            Key::Up => self.row0 &= 0b1011,
            Key::Down => self.row0 &= 0b0111,
            Key::A => self.row1 &= 0b1110,
            Key::B => self.row1 &= 0b1101,
            Key::Select => self.row1 &= 0b1011,
            Key::Start => self.row1 &= 0b0111,
        }
        self.update();
    }

    pub fn release(&mut self, key: Key) {
        match key {
            Key::Right => self.row0 |= 0b0001,
            Key::Left => self.row0 |= 0b0010,
            Key::Up => self.row0 |= 0b0100,
            Key::Down => self.row0 |= 0b1000,
            Key::A => self.row1 |= 0b0001,
            Key::B => self.row1 |= 0b0010,
            Key::Select => self.row1 |= 0b0100,
            Key::Start => self.row1 |= 0b1000,
        }
        self.update();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypad_press() {
        let mut keypad = Keypad::new();
        keypad.press(Key::Right);
        assert_eq!(keypad.read(), 0b1110_1111);
    }

    #[test]
    fn test_keypad_release() {
        let mut keypad = Keypad::new();
        keypad.press(Key::Right);
        keypad.press(Key::Left);
        keypad.release(Key::Right);
        assert_eq!(keypad.read(), 0b1101_1111);
    }

    #[test]
    fn test_keypad_interrupt() {
        let mut keypad = Keypad::new();
        keypad.press(Key::Right);
        assert_eq!(keypad.interrupt, 0x10);
    }

    #[test]
    fn test_keypad_is_pressed() {
        let mut keypad = Keypad::new();
        keypad.press(Key::Right);
        assert_eq!(keypad.is_pressed(Key::Right), true);
    }

    #[test]
    fn test_keypad_is_not_pressed() {
        let keypad = Keypad::new();
        assert_eq!(keypad.is_pressed(Key::Left), false);
    }
}

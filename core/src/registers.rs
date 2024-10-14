
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Flag {
    Zero = 1 << 7,
    Sub = 1 << 6,
    HalfCarry = 1 << 5,
    Carry = 1 << 4,
    None = 0,
}

/* #[derive(Debug, Copy, Clone, PartialEq)]
pub enum Flag {
    Z = 1 << 7,
    N = 1 << 6,
    H = 1 << 5,
    C = 1 << 4,
} */

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub sp: u16,
    pub pc: u16,
}

// Impl displays the registers in a readable format hex
impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "A: {:#04x} F: {:#04x}\nB: {:#04x} C: {:#04x}\nD: {:#04x} E: {:#04x}\nH: {:#04x} L: {:#04x}\nSP: {:#06x} PC: {:#06x}",
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc
        )
    }
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xb0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            sp: 0xfffe,
            pc: 0x0100,
        }
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn hli(&mut self) -> u16 {
        let value = self.hl();
        self.set_hl(value.wrapping_add(1));
        value
    }

    pub fn hld(&mut self) -> u16 {
        let value = self.hl();
        self.set_hl(value.wrapping_sub(1));
        value
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xff00) >> 8) as u8;
        self.c = (value & 0x00ff) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xff00) >> 8) as u8;
        self.e = (value & 0x00ff) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xff00) >> 8) as u8;
        self.l = (value & 0x00ff) as u8;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xff00) >> 8) as u8;
        self.f = (value & 0x00ff) as u8;
    }

    pub fn up_flag(&mut self, flag: Flag) {
        self.f |= flag as u8;
    }

    pub fn down_flag(&mut self, flag: Flag) {
        self.f &= !(flag as u8);
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.up_flag(flag);
        } else {
            self.down_flag(flag);
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.f & (flag as u8) != 0
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values_registers() {
        let registers = Registers::new();
        assert_eq!(registers.a, 0x01);
        assert_eq!(registers.b, 0x00);
        assert_eq!(registers.c, 0x13);
        assert_eq!(registers.d, 0x00);
        assert_eq!(registers.e, 0xd8);
        assert_eq!(registers.h, 0x01);
        assert_eq!(registers.l, 0x4d);
        assert_eq!(registers.f, 0xb0);
        assert_eq!(registers.sp, 0xfffe);
        assert_eq!(registers.pc, 0x0100);
    }

    #[test]
    fn test_registers_get_bc() {
        let mut registers = Registers::new();
        registers.b = 0x12;
        registers.c = 0x34;
        assert_eq!(registers.bc(), 0x1234);
    }

    #[test]
    fn test_registers_get_de() {
        let mut registers = Registers::new();
        registers.d = 0x12;
        registers.e = 0x34;
        assert_eq!(registers.de(), 0x1234);
    }

    #[test]
    fn test_registers_get_hl() {
        let mut registers = Registers::new();
        registers.h = 0x12;
        registers.l = 0x34;
        assert_eq!(registers.hl(), 0x1234);
    }

    #[test]
    fn test_registers_get_af() {
        let mut registers = Registers::new();
        registers.a = 0x12;
        registers.f = 0x34;
        assert_eq!(registers.af(), 0x1234);
    }

    #[test]
    fn test_registers_fn_hli() {
        let mut registers = Registers::new();
        registers.h = 0x12;
        registers.l = 0x34;
        assert_eq!(registers.hli(), 0x1234);
        assert_eq!(registers.hl(), 0x1235);
    }

    #[test]
    fn test_registers_fn_hld() {
        let mut registers = Registers::new();
        registers.h = 0x12;
        registers.l = 0x34;
        assert_eq!(registers.hld(), 0x1234);
        assert_eq!(registers.hl(), 0x1233);
    }

    #[test]
    fn test_registers_set_bc() {
        let mut registers = Registers::new();
        registers.set_bc(0x1234);
        assert_eq!(registers.b, 0x12);
        assert_eq!(registers.c, 0x34);
    }

    #[test]
    fn test_registers_set_de() {
        let mut registers = Registers::new();
        registers.set_de(0x1234);
        assert_eq!(registers.d, 0x12);
        assert_eq!(registers.e, 0x34);
    }

    #[test]
    fn test_registers_set_hl() {
        let mut registers = Registers::new();
        registers.set_hl(0x1234);
        assert_eq!(registers.h, 0x12);
        assert_eq!(registers.l, 0x34);
    }

    #[test]
    fn test_registers_set_af() {
        let mut registers = Registers::new();
        registers.set_af(0x1234);
        assert_eq!(registers.a, 0x12);
        assert_eq!(registers.f, 0x34);
    }

    /* #[test]
    fn test_registers_up_flag() {
        let mut registers = Registers::new();
        registers.up_flag(Flag::Sub);
        assert_eq!(registers.f, 0xb0 | Flag::Sub as u8);
    }

    #[test]
    fn test_registers_down_flag() {
        let mut registers = Registers::new();
        registers.up_flag(Flag::Sub);
        registers.down_flag(Flag::Sub);
        assert_eq!(registers.f, 0xb0);
    }

    #[test]
    fn test_registers_set_flag_true_and_false() {
        let mut registers = Registers::new();
        registers.f = 0x0;
        registers.set_flag(Flag::Zero, true);
        assert_eq!(registers.f, Flag::Zero as u8);
        registers.set_flag(Flag::Zero, false);
        assert_eq!(registers.f, 0x0);
    }

    #[test]
    fn test_registers_get_flag() {
        let mut registers = Registers::new();
        registers.f = 0xb0;
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Sub), false);
        assert_eq!(registers.get_flag(Flag::HalfCarry), true);
        assert_eq!(registers.get_flag(Flag::Carry), true);
    } */
    

}
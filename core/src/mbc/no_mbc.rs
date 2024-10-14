use super::MBC;

pub struct NoMBC {
    rom: Vec<u8>,
}

impl NoMBC {
    pub fn new(rom: &Vec<u8>) -> Self {
        NoMBC { rom: rom.to_vec() }
    }
}

impl MBC for NoMBC {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {}

    fn read_ram(&self, _address: u16) -> u8 {
        0
    }

    fn write_ram(&mut self, _address: u16, _value: u8) {}

    fn has_battery(&self) -> bool {
        false
    }

    fn info(&self) -> String {
        "No MBC".to_string()
    }
}

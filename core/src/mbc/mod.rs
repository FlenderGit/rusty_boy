use mbc1::MBC1;
use mbc5::MBC5;
use no_mbc::NoMBC;

mod mbc1;
mod no_mbc;
mod mbc5;

pub trait MBC: Send {
    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
    fn has_battery(&self) -> bool;
    fn info(&self) -> String;
}

pub fn from_rom(rom: &Vec<u8>) -> Result<Box<dyn MBC>, std::io::Error> {
    if rom.len() <= 0x0150 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid ROM size",
        ));
    }

    match rom[0x147] {
        0x00 => Ok(Box::new(NoMBC::new(rom))),
        0x01..=0x03 => Ok(Box::new(MBC1::new(rom))),
        0x19..=0x1E => Ok(Box::new(MBC5::new(rom))),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Unsupported MBC type: {:02x}", rom[0x147]),
        )),
    }
}

fn get_number_rom_banks(value: u8) -> usize {
    match value {
        0x00 => 2,
        0x01 => 4,
        0x02 => 8,
        0x03 => 16,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => panic!("Invalid MBC1 bank size: {:02x}", value),
    }
}

fn get_number_ram_banks(value: u8) -> usize {
    match value {
        0x00 => 0,
        0x01 => 1,
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => panic!("Invalid MBC1 RAM bank size: {:02x}", value),
    }
}

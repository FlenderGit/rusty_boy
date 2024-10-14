use std::cmp::max;

use super::{get_number_ram_banks, get_number_rom_banks, MBC};

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
enum Mode {
    Mode0,
    Mode1,
}

impl From<bool> for Mode {
    fn from(value: bool) -> Self {
        match value {
            false => Mode::Mode0,
            true => Mode::Mode1,
        }
    }
}

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    rom_banks_number: usize,
    ram_banks_number: usize,

    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
    ram_updated: bool,
    mode: Mode,

    has_battery: bool,
}

impl MBC1 {
    pub fn new(rom: &Vec<u8>) -> Self {
        let (has_battery, ram_size) = match rom[0x0149] {
            0x00 | 0x01 => (false, 0),
            0x02 => (false, get_number_ram_banks(rom[0x0149])),
            0x03 => (true, get_number_ram_banks(rom[0x0149])),
            _ => panic!("Invalid MBC1 RAM size: {:02x}", rom[0x0149]),
        };

        MBC1 {
            rom: rom.to_vec(),
            ram: std::iter::repeat(0).take(ram_size * 0x2000).collect(),
            rom_bank: 1,
            ram_bank: 0,

            ram_enabled: false,
            ram_updated: false,
            mode: Mode::Mode0,

            rom_banks_number: get_number_rom_banks(rom[0x0148]),
            ram_banks_number: ram_size,

            has_battery: has_battery,
        }
    }
}

impl MBC for MBC1 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = match (address < 0x4000, self.mode) {
            (true, Mode::Mode0) => 0,
            (true, _) => self.rom_bank & 0xE0,
            _ => self.rom_bank,
        };
        let index = bank * 0x4000 + (address as usize & 0x3FFF);
        self.rom.get(index).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0xF == 0xA, // Value with 0xa on the lowest but enable the RAM. Else disable.
            0x2000..=0x3FFF => {
                let rom_bank = max(1, value & 0x1f) as usize; // Bank is selected using 5 lower bits. 0x00 -> 0x01
                self.rom_bank = ((self.rom_bank & 0x60) | rom_bank) % self.rom_banks_number;
            }
            0x4000..=0x5FFF => {
                if self.rom_banks_number > 0x20 {
                    let upper_bits = (value as usize & 0x03) % (self.ram_banks_number >> 5);
                    self.rom_bank = (self.rom_bank & 0x1F) | (upper_bits << 5);
                }
                if self.rom_banks_number > 1 {
                    self.rom_bank = (value as usize) & 0x03;
                }
                
                //self.ram_bank = value as usize & 0x03;      // Redo : https://gbdev.io/pandocs/MBC1.html#40005fff--ram-bank-number--or--upper-bits-of-rom-bank-number-write-only
            }
            0x6000..=0x7FFF => self.mode = Mode::from(value & 0x01 == 0),
            _ => { /* Do nothing */ }
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled { return 0xFF; }
        let bank = if self.mode == Mode::Mode0 { 0 } else { self.ram_bank };
        let offset = (address as usize) & 0x1FFF;
        self.ram[bank * 0x2000 | offset]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled { return }
        let rambank = if self.mode == Mode::Mode0 { 0 } else { self.ram_bank };
        let address = (rambank * 0x2000) | ((address & 0x1FFF) as usize);
        if address < self.ram.len() {
            self.ram[address] = value;
            self.ram_updated = true;
        }
    }

    fn has_battery(&self) -> bool { self.has_battery }

    fn info(&self) -> String {
        format!(
            "MBC1: {:02x}, {:02x}, {}, {}",
            self.rom_bank, self.ram_bank, self.ram_enabled, self.mode == Mode::Mode1
        )
    }
}

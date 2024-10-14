use super::{get_number_ram_banks, get_number_rom_banks};
use crate::mbc::MBC;

pub struct MBC5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_on: bool,
    has_battery: bool,
    rombanks: usize,
    rambanks: usize,
}

impl MBC5 {
    pub fn new(data: &Vec<u8>) -> MBC5 {
        let rom_type = data[0x147];
        let has_battery = match rom_type {
            0x1B | 0x1E => true,
            _ => false,
        };
        let rambanks = match rom_type {
            0x1A | 0x1B | 0x1D | 0x1E => get_number_ram_banks(data[0x149]),
            _ => 0,
        };
        let ramsize = 0x2000 * rambanks;
        let rombanks = get_number_rom_banks(data[0x148]);

        let res = MBC5 {
            rom: data.to_vec(),
            ram: ::std::iter::repeat(0).take(ramsize).collect(),
            rom_bank: 1,
            ram_bank: 0,
            ram_on: false,
            has_battery,
            rombanks,
            rambanks,
        };
        res
    }
}

impl MBC for MBC5 {
    fn read_rom(&self, a: u16) -> u8 {
        let index = if a < 0x4000 {
            a as usize
        } else {
            self.rom_bank * 0x4000 | ((a as usize) & 0x3FFF)
        };

        self.rom.get(index).copied().unwrap_or(0x00)
    }
    fn read_ram(&self, a: u16) -> u8 {
        if !self.ram_on {
            return 0;
        }
        self.ram[self.ram_bank * 0x2000 | ((a as usize) & 0x1FFF)]
    }
    fn write_rom(&mut self, a: u16, v: u8) {
        match a {
            0x0000..=0x1FFF => self.ram_on = v & 0x0F == 0x0A,
            0x2000..=0x2FFF => {
                self.rom_bank = ((self.rom_bank & 0x100) | (v as usize)) % self.rombanks
            }
            0x3000..=0x3FFF => {
                self.rom_bank =
                    ((self.rom_bank & 0x0FF) | (((v & 0x1) as usize) << 8)) % self.rombanks
            }
            0x4000..=0x5FFF => self.ram_bank = ((v & 0x0F) as usize) % self.rambanks,
            0x6000..=0x7FFF => {}
            _ => panic!("Could not write to {:04X} (MBC5)", a),
        }
    }
    fn write_ram(&mut self, a: u16, v: u8) {
        if self.ram_on == false {
            return;
        }
        self.ram[self.ram_bank * 0x2000 | ((a as usize) & 0x1FFF)] = v;
    }

    fn has_battery(&self) -> bool {
        self.has_battery
    }

    fn info(&self) -> String {
        format!(
            "MBC5: ROM bank: {:02X}, RAM bank: {:02X}, RAM enabled: {}",
            self.rom_bank, self.ram_bank, self.ram_on
        )
    }
}

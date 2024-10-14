use std::io::Seek;

use crate::cpu::CPU;
use crate::gpu::SCREEN_SIZE_RGB;
use crate::header::Header;
use crate::keypad::KeyEvent;

const FRAME_TIME: f64 = 1.0 / 60.0;
const CYCLES_PER_SECOND: u32 = 4_194_304;
const CYCLES_PER_FRAME: u32 = CYCLES_PER_SECOND / 60;

#[derive(PartialEq)]
pub enum GBMode {
    DMG,
    CGB,
}

pub struct Gameboy {
    pub cpu: CPU,
    header: Header,
}

impl Gameboy {
    fn new_abs(rom: &Vec<u8>, header: Header) -> Result<Gameboy, std::io::Error> {
        match crate::mbc::from_rom(&rom) {
            Ok(mbc) => Ok(Gameboy {
                cpu: CPU::new(mbc),
                header,
            }),
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn update_input(&mut self, event: KeyEvent) {
        match event {
            KeyEvent::Press(key) => self.cpu.memory.keypad.press(key),
            KeyEvent::Release(key) => self.cpu.memory.keypad.release(key),
        }
    }

    pub fn run_frame(&mut self) {
        let mut cycles = 0;
        while cycles < CYCLES_PER_FRAME {
            cycles += self.cpu.step() as u32;
        }
    }

    pub fn new_from_data(rom: &Vec<u8>, skip_checksum: bool) -> Result<Gameboy, std::io::Error> {
        if rom.len() <= 0x0150 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid ROM size",
            ));
        }

        let header = Header::load_rom(&rom[0x0100..=0x014F].try_into().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to convert ROM slice to header",
            )
        })?);
        Gameboy::new_abs(rom, header)
    }

    pub fn new_from_file(file_path: &str, skip_checksum: bool) -> Result<Gameboy, std::io::Error> {
        let mut file = std::fs::File::open(file_path)?;
        let header = Header::load_from_file(&mut file, skip_checksum)?;
        let rom = std::fs::read(file_path)?;
        Gameboy::new_abs(&rom, header)
    }

    /// Get the screen data
    pub fn get_screen_data(&self) -> &[u8; SCREEN_SIZE_RGB] {
        return self.cpu.memory.gpu.screen_data();
    }

    #[deprecated]
    pub fn save_vram(&self, path: &str) {
        use std::io::Write;
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(&self.cpu.memory.gpu.vram).unwrap();
    }

    /// Get the header of the loaded ROM
    /// This function will return the header of the loaded ROM or panic if no ROM is loaded
    pub fn header(&self) -> &Header {
        &self.header
    }
}

use crate::{gpu::GPU, keypad::Keypad, mbc::MBC, serial::Serial, timer::Timer};

const ROM_SIZE: usize = 0x8000;
const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7F;

pub struct Memory {
    pub mbc: Box<dyn MBC+'static>,
    pub gpu: GPU,
    
    pub keypad: Keypad,
    timer: Timer,
    serial: Serial,

    pub interrupt_flags: u8,
    pub interrupt_enable: u8,

    wram: [u8; WRAM_SIZE],
    wram_bank: u8,
    hram: [u8; HRAM_SIZE],
}

impl Memory {
    pub fn new(mbc: Box<dyn MBC+'static>) -> Memory {
        let mut memory = Memory {
            mbc,
            gpu: GPU::new(),
            
            keypad: Keypad::new(),
            serial: Serial::new(),
            timer: Timer::new(),

            wram: [0; WRAM_SIZE],
            wram_bank: 0,
            hram: [0; HRAM_SIZE],

            interrupt_flags: 0,
            interrupt_enable: 0,
        };
        memory.init();
        memory
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc.read_rom(address),
            0x8000..=0x9FFF => self.gpu.vram[address as usize - ROM_SIZE], // VRAM
            0xA000..=0xBFFF => self.mbc.read_ram(address),           // External RAM
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000], // Work RAM (WRAM) -- TD Handle WRAM bank switching
            0xE000..=0xFDFF => self.read(address - 0x2000),          // Echo RAM
            0xFE00..=0xFE9F => self.gpu.oam[address as usize - 0xFE00], // OAM
            0xFEA0..=0xFEFF => 0,                                   // Unusable
            0xFF00 => self.keypad.read(),                            // Keypad
            
            // TODO: Sound I/O & Serial I/O
            0xFF01..=0xFF02 => self.serial.read(address), // Serial I/O
            0xFF04..=0xFF07 => self.timer.read(address),             // Timer I/O
            
            0xFF0F => self.interrupt_flags,                          // Interrupt Flags
            
            // TODO: Sound I/O
            0xFF10..=0xFF3F => { 0 } // Sound I/O

            0xFF40..=0xFF4B => self.gpu.read(address), //LCD Control, Status, Position, Scrolling, and Palettes
            0xFF4F => self.gpu.vram_bank as u8,              // VRAM Bank
            0xFF50 => 0,                               // Boot ROM disable
            
            // TODO: VRAM DMA
            0xFF51..=0xFF55 => self.gpu.read(address), // VRAM DMA
            
            0xFF68..=0xFF6b => self.gpu.read(address), // Background/Object Palette Data
            0xFF70 => self.wram_bank,                  // WRAM Bank
            0xFF80..=0xFFFE => self.hram[address as usize & HRAM_SIZE], // High RAM
            0xFFFF => self.interrupt_enable,           // Interrupt Enable
            _ => { /* panic!("Unimplemented memory read at address: {:#06x}", address); */ 0 }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_rom(address, value), // Rom
            0x8000..=0x9FFF => self.gpu.vram[address as usize - 0x8000] = value, // VRAM
            0xA000..=0xBFFF => self.mbc.write_ram(address, value),           // External RAM
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = value, // Work RAM (WRAM) -- TD Handle WRAM bank switching
            0xE000..=0xFDFF => self.write(address - 0x2000, value),          // Echo RAM
            0xFE00..=0xFE9F => self.gpu.oam[address as usize - 0xFE00] = value, // OAM
            0xFEA0..=0xFEFF => (),                                           // Unusable
            0xFF00 => self.keypad.write(value),                              // Keypad
            
            // TODO: Sound I/O & Serial I/O
            0xFF01..=0xFF02 => self.serial.write(address, value), // Serial I/O
            0xFF04..=0xFF07 => self.timer.write(address, value),             // Timer I/O
            
            0xFF0F => self.interrupt_flags = value,                           // Interrupt Flags
            
            // TODO: Sound I/O
            0xFF10..=0xFF3F => {} // Sound I/O
            
            0xFF46 => { self.dma_transfer(value); } // OAM DMA
            0xFF40..=0xFF4B => self.gpu.write(address, value), //LCD Control, Status, Position, Scrolling, and Palettes
            0xFF4f => self.gpu.vram_bank = value as usize,              // VRAM Bank
            0xFF50 => (),                                      // Boot ROM disable
            
            // TODO: VRAM DMA
            0xFF51..=0xFF55 => self.gpu.write(address, value), // VRAM DMA
            
            0xFF68..=0xFF6B => self.gpu.write(address, value), // Background/Object Palette Data
            0xFF70 => self.wram_bank = value,                  // WRAM Bank
            0xFF80..=0xFFFE => self.hram[address as usize & HRAM_SIZE] = value, // High RAM
            0xFFFF => self.interrupt_enable = value,           // Interrupt Enable
            _ => { /* panic!("Unimplemented memory write at address: {:#06x}", address); */ }
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
    }

    pub fn read_word(&self, address: u16) -> u16 {
        (self.read(address) as u16) | ((self.read(address + 1) as u16) << 8)
    }

    pub fn step(&mut self, cycles: u8) {
        self.interrupt_flags = self.keypad.interrupt | self.gpu.interrupt | self.timer.interrupt;
        self.keypad.interrupt = 0;

        self.gpu.step(cycles);
        self.interrupt_flags |= self.gpu.interrupt;
        self.gpu.interrupt = 0;

        self.timer.step(cycles);
        self.interrupt_flags |= self.timer.interrupt;
        self.timer.interrupt = 0;
    }

    fn dma_transfer(&mut self, address: u8) {
        let start = address as u16 * 0x100;
        for i in 0..0xA0 {
            self.write(0xFE00 | i, self.read(start + i));
        }
    }

    fn init(&mut self) {
        self.write(0xFF05, 0x00);
        self.write(0xFF06, 0x00);
        self.write(0xFF07, 0x00);
        self.write(0xFF10, 0x80);
        self.write(0xFF11, 0xBF);
        self.write(0xFF12, 0xF3);
        self.write(0xFF14, 0xBF);
        self.write(0xFF16, 0x3F);
        self.write(0xFF17, 0x00);
        self.write(0xFF19, 0xBF);
        self.write(0xFF1A, 0x7F);
        self.write(0xFF1B, 0xFF);
        self.write(0xFF1C, 0x9F);
        self.write(0xFF1E, 0xFF);
        self.write(0xFF20, 0xFF);
        self.write(0xFF21, 0x00);
        self.write(0xFF22, 0x00);
        self.write(0xFF23, 0xBF);
        self.write(0xFF24, 0x77);
        self.write(0xFF25, 0xF3);
        self.write(0xFF26, 0xF1);
        self.write(0xFF40, 0x91);
        self.write(0xFF42, 0x00);
        self.write(0xFF43, 0x00);
        self.write(0xFF45, 0x00);
        self.write(0xFF47, 0xFC);
        self.write(0xFF48, 0xFF);
        self.write(0xFF49, 0xFF);
        self.write(0xFF4A, 0x00);
        self.write(0xFF4B, 0x00);
    }
}



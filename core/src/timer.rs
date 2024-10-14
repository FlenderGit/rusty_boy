#[derive(Clone, Copy)]
enum TimerMode {
    Clock256Mhz,
    Clock4Mhz,
    Clock16Mhz,
    Clock64Khz,
}


impl Into<u8> for TimerMode {
    fn into(self) -> u8 {
        match self {
            TimerMode::Clock256Mhz => 64,
            TimerMode::Clock4Mhz => 1,
            TimerMode::Clock16Mhz => 4,
            TimerMode::Clock64Khz => 16,
        }
    }
}

/**
* @see: https://gbdev.io/pandocs/Timer_and_Divider_Registers.html
*/
pub struct Timer {
    
    // 0xFF04 — DIV: Divider register
    div: u8,

    // 0xFF05 — TIMA: Timer counter
    tima: u8,

    // 0xFF06 — TMA: Timer Modulo
    tma: u8,

    // 0xFF07 — TAC: Timer Control
    enable: bool,
    mode: TimerMode,

    timer_clock: u8,

    /* active: bool,
    timer_clock: u8,
    internal_clock: u8, */

    pub interrupt: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            
            enable: false,
            mode: TimerMode::Clock256Mhz,
            
            interrupt: 0,
            timer_clock: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => {
                0xF8 |
                (self.enable as u8) << 2 |
                match self.mode {
                    TimerMode::Clock256Mhz => 0b00,
                    TimerMode::Clock4Mhz => 0b01,
                    TimerMode::Clock16Mhz => 0b10,
                    TimerMode::Clock64Khz => 0b11,
                }
            }
            _ => panic!("Invalid timer read address: {:04x}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => {
                self.enable = value & 0b100 != 0;
                self.mode = match value & 0b11 {
                    0b00 => TimerMode::Clock256Mhz,
                    0b01 => TimerMode::Clock4Mhz,
                    0b10 => TimerMode::Clock16Mhz,
                    0b11 => TimerMode::Clock64Khz,
                    _ => { unreachable!() },
                };
            }
            _ => panic!("Invalid timer write address: {:04x}", address),
        }
    }

    pub fn step(&mut self, cycles: u8) {
        
        self.timer_clock += cycles;

        let mode_value= TimerMode::into(self.mode);
        while self.timer_clock >= mode_value {
            self.div = self.div.wrapping_add(1);
            self.timer_clock = self.timer_clock.wrapping_sub(mode_value);
        }

        if self.enable && (255 - self.tima) < cycles {
            self.tima = self.tma;
            self.interrupt = 0x04;
        }
    }
}
/**
* @see: https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html
*/
pub struct Serial {
    // 0xFF01 — SB: Serial transfer data
    sb: u8,
    // 0xFF02 — SC: Serial transfer control
    sc: u8,
}

impl Serial {
    
    pub fn new() -> Self {
        Serial {
            sb: 0,
            sc: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            _ => panic!("Invalid serial address: {:04x}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => self.sb = value,
            0xFF02 => self.sc = value,
            _ => panic!("Invalid serial address: {:04x}", address),
        }
    }
}
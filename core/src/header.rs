use std::io::{Error, ErrorKind, Seek, SeekFrom};

pub struct Header {
    title: String,
    manufacturer_code: String,
    cgb_flag: u8,
    new_licensee_code: u16,
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: u16,
}

#[inline]
fn extract_string(slice: &[u8]) -> String {
    let end = slice.iter().position(|&x| x == 0x00).unwrap_or(slice.len());
    let bytes = &slice[..end];
    String::from_utf8_lossy(bytes).to_string()
}

impl Header {
    pub fn load_rom(header: &[u8; 0x50]) -> Self {
        Header {
            title: extract_string(&header[0x34..0x44]),
            manufacturer_code: extract_string(&header[0x3F..0x43]),
            cgb_flag: header[0x43],
            new_licensee_code: (header[0x44] as u16) | (header[0x45] as u16) << 8,
            sgb_flag: header[0x46],
            cartridge_type: header[0x47],
            rom_size: header[0x48],
            ram_size: header[0x49],
            destination_code: header[0x4A],
            old_licensee_code: header[0x4B],
            mask_rom_version_number: header[0x4C],
            header_checksum: header[0x4D],
            global_checksum: (header[0x4E] as u16) << 8 | header[0x4F] as u16,
        }
    }

    pub fn load_from_file(
        file: &mut std::fs::File,
        skip_checksum: bool,
    ) -> Result<Self, std::io::Error> {
        use std::io::Read;

        // Skip the first 0x100 bytes & read the next 0x50 bytes
        let mut buffer = [0; 0x50];
        file.seek(SeekFrom::Start(0x100))?;
        file.read_exact(&mut buffer)?;

        if skip_checksum == false {
            // Check the header checksum
            let mut checksum = 0u8;
            for i in 0x34..0x4D {
                checksum = checksum.wrapping_sub(buffer[i] + 1);
            }

            if checksum != buffer[0x4D] {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid header checksum",
                ));
            }

            // /!\ The global checksum (0x014E-0x014F) is not checked, 'cause it's not mandatory and not checked by the Gameboy
        }

        let header = Header::load_rom(&buffer);
        Ok(header)
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn manufacturer_code(&self) -> &str {
        &self.manufacturer_code
    }

    pub fn cgb_flag(&self) -> u8 {
        self.cgb_flag
    }

    pub fn cartridge_type(&self) -> u8 {
        self.cartridge_type
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Title: {}\nManufacturer code: {}\nLicencee: {}\nCGB flag: {}\nCartridge type: {}\nROM size: {}\nRAM size: {}\nDestination code: {}\nOld licensee code: {}\nMask ROM version number: {}\nHeader checksum: {:02x}\nGlobal checksum: {:04x}",
            self.title,
            self.manufacturer_code,
            get_licencee_name(self.new_licensee_code as u16, self.old_licensee_code),
            self.cgb_flag,
            self.cartridge_type,
            self.rom_size,
            self.ram_size,
            self.destination_code,
            self.old_licensee_code,
            self.mask_rom_version_number,
            self.header_checksum,
            self.global_checksum
        )
    }
}

fn get_licencee_name<'a>(new_code: u16, old_code: u8) -> &'a str {
    if old_code == 0x33 {
        match (new_code >> 4, new_code & 0xff) {
            (0, 0) => "None",
            (0, 1) => "Nintendo R&D1",
            (0, 8) => "Capcom",
            (1, 3) => "Electronic Arts",
            /* 0x18 => "Hudson Soft",
            0x19 => "B-AI",
            0x20 => "KSS",
            0x22 => "Planning Office WADA",
            0x24 => "PCM Complete",
            0x25 => "San-x",
            0x28 => "Kemco",
            0x29 => "SETA Corporation",
            0x30 => "Viacom",
            0x31 => "Nintendo",
            0x32 => "Bandai",
            0x33 => "Ocean/Acclaim Entertainment",
            0x34 => "Konami",
            0x35 => "Hector Soft",
            0x37 => "Taito",
            0x38 => "Hudson Soft",
            0x39 => "Banpresto",
            0x41 => "Ubi Soft",
            0x42 => "Atlus",
            0x44 => "Malibu Interactive",
            0x46 => "Angel",
            0x47 => "Bullet-Proof Software",
            0x49 => "Irem",
            0x50 => "Absolute",
            0x51 => "Acclaim Entertainment",
            0x52 => "Activision",
            0x53 => "Sammy USA Corporation",
            0x54 => "Konami",
            0x55 => "Hi tech entertainment",
            0x56 => "LJN",
            0x57 => "Matchbox",
            0x58 => "Mattel",
            0x59 => "Milton Bradley Company",
            0x60 => "Titus Interactive",
            0x61 => "Virgin Games Ltd.",
            0x64 => "LucasArts Games",
            0x67 => "Ocean Software",
            0x69 => "Electronic Arts",
            0x70 => "Infogrames",
            0x71 => "Interplay Entertainment",
            0x72 => "Broderbund",
            0x73 => "Sculptured Software",
            0x75 => "The Sales Curve Limited",
            0x78 => "THQ",
            0x79 => "Accolade",
            0x80 => "Misawa Entertainment",
            0x83 => "lozc",
            0x86 => "Tokuma Shoten",
            0x87 => "Tsukuda Original",
            0x91 => "Chunsoft Co.",
            0x92 => "Video System",
            0x93 => "Ocean/Acclaim Entertainment",
            0x95 => "Varie",
            0x96 => "Yonezawa/S'pal",
            0x97 => "Kaneko",
            0x99 => "Pack in Video",
            0x9h => "Bottom Up",
            0xa4 => "Konami",
            0xbl => "MTO",
            0xdk => "Kodansha", */
            _ => "Unknown new",
        }
    } else {
        match old_code {
            0x00 => "None",
            0x01 => "Nintendo",
            0x08 => "Capcom",
            _ => "Unknown old",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let mut file = std::fs::File::open("../roms/tetris.gb").unwrap();
        let header = Header::load_from_file(&mut file, true).unwrap();

        assert_eq!(header.title(), "TETRIS");
        assert_eq!(header.manufacturer_code(), "");
        assert_eq!(header.cgb_flag(), 0x00);
    }
}

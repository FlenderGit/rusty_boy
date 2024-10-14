use std::cmp::Ordering;
// use crate::gbmode::GbMode;

const VRAM_SIZE: usize = 0x4000;
const VOAM_SIZE: usize = 0xA0;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

const OAM_SIZE: usize = 0xA0;

pub const SCREEN_SIZE_RGB: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

#[derive(PartialEq, Copy, Clone)]
enum PrioType {
    Color0,
    PrioFlag,
    Normal,
}

enum PaletteType {
    Bg,
    Obj0,
    Obj1,
}

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
enum Mode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
}

struct Sprite {
    y: u8,
    x: u8,
    tile: u8,
    flags: u8,
}

/**
* @see: https://gbdev.io/pandocs/Graphics.html
*/
pub struct GPU {
    wy_pos: i32,

    pub data: [u8; SCREEN_SIZE_RGB],

    // Mode of the screen (HBlank, VBlank, OAM, VRAM)
    mode: Mode,

    // Video RAM & current bank
    pub vram: [u8; VRAM_SIZE],

    // Object Attribute Memory - see https://gbdev.io/pandocs/OAM.html
    pub oam: [u8; VOAM_SIZE],

    clock: u32,

    // === LCDC (0xFF40) === see https://gbdev.io/pandocs/LCDC.html
    // LCDC.7 - LCD Display Enable (0=Off, 1=On)
    lcd_on: bool,

    // LCDC.6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    win_tilemap: u16,

    // LCDC.5 - Window Display Enable (0=Off, 1=On)
    win_on: bool,

    // LCDC.4 - BG & Window Tile Data Select (0=8800-97FF, 1=8000-8FFF)
    bgw_tiles: u16,

    // LCDC.3 - BG Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    bg_tilemap: u16,

    // LCDC.2 - Sprite Size (0=8x8, 1=8x16)
    sprite_size: u8,

    // LCDC.1 - Sprite Display Enable (0=Off, 1=On)
    sprite_on: bool,

    // LCDC.0 - BG Display (0=Off, 1=On)
    bgw_on: bool,

    // === STAT (0xFF41) === see https://gbdev.io/pandocs/STAT.html
    // STAT.6 - LYC=LY Coincidence Interrupt (1=Enable) (Read/Write)
    lyc_interrupt: bool,

    // STAT.5 - Mode 2 OAM Interrupt (1=Enable) (Read/Write)
    mode2_interrupt: bool,

    // STAT.4 - Mode 1 V-Blank Interrupt (1=Enable) (Read/Write)
    mode1_interrupt: bool,

    // STAT.3 - Mode 0 H-Blank Interrupt (1=Enable) (Read/Write)
    mode0_interrupt: bool,

    // STAT.2 - LYC=LY Coincidence (0:LYC<>LY, 1:LYC=LY) (Read Only)
    lyc_eq_ly: bool,

    // STAT.1-0 - Mode Flag (Mode 0-3) (Read Only)
    // 0: HBlank, 1: VBlank, 2: OAM, 3: VRAM
    // see variable `mode`

    // === SCY (0xFF42) === see https://gbdev.io/pandocs/SCY.html
    // SCY - Scroll Y (R/W)
    scy: u8,

    // === SCX (0xFF43) === see https://gbdev.io/pandocs/SCX.html
    // SCX - Scroll X (R/W)
    scx: u8,

    // === LY (0xFF44) === see https://gbdev.io/pandocs/LY.html
    // LY - LCDC Y-Coordinate (R)
    line: u8,

    // === LYC (0xFF45) === see https://gbdev.io/pandocs/LYC.html
    // LYC - LY Compare (R/W)
    lyc: u8,

    // === Monochrome palettes (0xFF47/0xFF48/0xFF49) === see https://gbdev.gg8.se/wiki/articles/Video_Display#LCD_Monochrome_Palettes
    palette_bg_value: u8,
    palette_obp0_value: u8,
    palette_obp1_value: u8,

    // === WY/WX (0xFF4A/0xFF4B) === see https://gbdev.io/pandocs/Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7
    // WY - Window Y Position (R/W)
    wy: u8,
    // WX - Window X Position minus 7 (R/W)
    wx: u8,

    // === VBK (0xFF4F) === see https://gbdev.io/pandocs/VBK.html
    // VBK - VRAM Bank (R/W)
    pub vram_bank: usize,

    palette_bg: [u8; 4],
    palette_obp0: [u8; 4],
    palette_obp1: [u8; 4],

    wy_trigger: bool,

    pub interrupt: u8,
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            mode: Mode::HBlank,
            clock: 0,

            vram: [0; VRAM_SIZE],
            vram_bank: 0,
            oam: [0; VOAM_SIZE],

            line: 0,
            lyc: 0,
            lcd_on: false,
            win_tilemap: 0x9C00,
            win_on: false,
            bgw_tiles: 0x8000,
            bg_tilemap: 0x9C00,
            sprite_size: 8,
            sprite_on: false,
            bgw_on: false,
            lyc_eq_ly: false,
            mode2_interrupt: false,
            mode1_interrupt: false,
            mode0_interrupt: false,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,

            wy_trigger: false,
            wy_pos: -1,

            palette_bg_value: 0,
            palette_obp0_value: 0,
            palette_obp1_value: 1,
            palette_bg: [0; 4],
            palette_obp0: [0; 4],
            palette_obp1: [0; 4],

            data: [255; SCREEN_SIZE_RGB],
            interrupt: 0,
            lyc_interrupt: false,
        }
    }

    pub fn new_cgb() -> GPU {
        GPU::new()
    }

    pub fn step(&mut self, ticks: u8) {
        if !self.lcd_on {
            return;
        }

        self.clock += ticks as u32;

        match self.mode {
            Mode::HBlank => {
                if self.clock >= 204 {
                    self.clock = 0;
                    self.line += 1;
                    if self.line == 144 {
                        self.change_mode(Mode::VBlank);
                    } else {
                        self.change_mode(Mode::OAM);
                    }
                }
            }
            Mode::VBlank => {
                if self.clock >= 456 {
                    self.clock = 0;
                    self.line += 1;
                    if self.line > 153 {
                        self.line = 0;
                        self.change_mode(Mode::OAM);
                    }
                }
            }
            Mode::OAM => {
                if self.clock >= 80 {
                    self.clock = 0;
                    self.change_mode(Mode::VRAM);
                }
            }
            Mode::VRAM => {
                if self.clock >= 172 {
                    self.clock = 0;
                    self.change_mode(Mode::HBlank);
                    self.check_interrupt_lyc();
                }
            }
        }
    }

    #[inline(always)]
    fn check_interrupt_lyc(&mut self) {
        if self.lyc_interrupt && self.line == self.lyc {
            self.interrupt |= 0x02;
        }
    }

    fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;

        if match self.mode {
            Mode::HBlank => {
                self.renderscan();
                self.mode0_interrupt
            }
            Mode::VBlank => {
                // Vertical blank
                self.wy_trigger = false;
                self.interrupt |= 0x01;
                self.mode1_interrupt
            }
            Mode::OAM => self.mode2_interrupt,
            Mode::VRAM => {
                if self.win_on && !self.wy_trigger && self.line == self.wy {
                    self.wy_trigger = true;
                    self.wy_pos = -1;
                }
                false
            }
        } {
            self.interrupt |= 0x02;
        }
    }

    pub fn read(&self, a: u16) -> u8 {
        match a {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (a as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.oam[a as usize - 0xFE00],
            0xFF40 => {
                ((self.lcd_on as u8) << 7)
                    | (((self.win_tilemap == 0x9C00) as u8) << 6)
                    | ((self.win_on as u8) << 5)
                    | (((self.bgw_tiles == 0x8000) as u8) << 4)
                    | (((self.bg_tilemap == 0x9C00) as u8) << 3)
                    | (((self.sprite_size == 16) as u8) << 2)
                    | ((self.sprite_on as u8) << 1)
                    | (self.bgw_on as u8)
            }
            0xFF41 => {
                0x80 | ((self.lyc_interrupt as u8) << 6)
                    | ((self.mode2_interrupt as u8) << 5)
                    | ((self.mode1_interrupt as u8) << 4)
                    | ((self.mode0_interrupt as u8) << 3)
                    | ((self.line == self.lyc) as u8) << 2
                    | self.mode as u8
            }
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.line,
            0xFF45 => self.lyc,
            0xFF46 => 0, // Write only
            0xFF47 => self.palette_bg_value,
            0xFF48 => self.palette_obp0_value,
            0xFF49 => self.palette_obp1_value,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4C => 0xFF,
            0xFF4E => 0xFF,
            0xFF4F => self.vram_bank as u8 | 0xFE,
            _ => 0xFF,
        }
    }

    /**
     * Can return Result<u8, &'static str> instead of u8 but idk if it's worth it
     */
    fn rbvram0(&self, a: u16) -> u8 {
        if a < 0x8000 || a >= 0xA000 {
            0xFF
        } else {
            self.vram[a as usize & 0x1FFF]
        }
    }
    fn rbvram1(&self, a: u16) -> u8 {
        if a < 0x8000 || a >= 0xA000 {
            0xFF
        } else {
            self.vram[a as usize & 0x1FFF]
        }
    }

    pub fn write(&mut self, a: u16, v: u8) {
        match a {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (a as usize & 0x1FFF)] = v,
            0xFE00..=0xFE9F => self.oam[a as usize - 0xFE00] = v,
            0xFF40 => {
                let orig_lcd_on = self.lcd_on;
                self.lcd_on = v & 0x80 == 0x80;
                self.win_tilemap = if v & 0x40 == 0x40 { 0x9C00 } else { 0x9800 };
                self.win_on = v & 0x20 == 0x20;
                self.bgw_tiles = if v & 0x10 == 0x10 { 0x8000 } else { 0x8800 };
                self.bg_tilemap = if v & 0x08 == 0x08 { 0x9C00 } else { 0x9800 };
                self.sprite_size = if v & 0x04 == 0x04 { 16 } else { 8 };
                self.sprite_on = v & 0x02 == 0x02;
                self.bgw_on = v & 0x01 == 0x01;
                if orig_lcd_on && !self.lcd_on {
                    self.clock = 0;
                    self.line = 0;
                    self.mode = Mode::HBlank;
                    self.wy_trigger = false;
                    self.clear_screen();
                }
                if !orig_lcd_on && self.lcd_on {
                    self.change_mode(Mode::VBlank);
                    self.clock = 4;
                }
            }
            0xFF41 => {
                self.lyc_interrupt = v & 0x40 == 0x40;
                self.mode2_interrupt = v & 0x20 == 0x20;
                self.mode1_interrupt = v & 0x10 == 0x10;
                self.mode0_interrupt = v & 0x08 == 0x08;
            }
            0xFF42 => self.scy = v,
            0xFF43 => self.scx = v,
            0xFF44 => {} // Read-only
            0xFF45 => {
                self.lyc = v;
                self.check_interrupt_lyc();
            }
            0xFF46 => panic!("0xFF46 should be handled by MMU"),
            0xFF47 => {
                self.palette_bg_value = v;
                self.update_palette(PaletteType::Bg);
            }
            0xFF48 => {
                self.palette_obp0_value = v;
                self.update_palette(PaletteType::Obj0);
            }
            0xFF49 => {
                self.palette_obp1_value = v;
                self.update_palette(PaletteType::Obj1);
            }
            0xFF4A => self.wy = v,
            0xFF4B => self.wx = v,
            0xFF4C => {}
            0xFF4E => {}
            0xFF4F => self.vram_bank = (v & 0x01) as usize,
            _ => panic!("GPU does not handle write {:04X}", a),
        }
    }

    fn clear_screen(&mut self) {
        for v in self.data.iter_mut() {
            *v = 255;
        }
    }

    fn update_palette(&mut self, palette: PaletteType) {
        let (palette, value) = match palette {
            PaletteType::Bg => (&mut self.palette_bg, self.palette_bg_value),
            PaletteType::Obj0 => (&mut self.palette_obp0, self.palette_obp0_value),
            PaletteType::Obj1 => (&mut self.palette_obp1, self.palette_obp1_value),
        };
        for i in 0..4 {
            palette[i] = GPU::get_monochrome_palette_value(value, i);
        }
    }

    fn get_monochrome_palette_value(value: u8, index: usize) -> u8 {
        match (value >> 2 * index) & 0x03 {
            0 => 255,
            1 => 192,
            2 => 96,
            _ => 0,
        }
    }

    fn renderscan(&mut self) {
        self.draw_bg();
        self.draw_sprites();
    }

    fn set_color(&mut self, x: usize, color: u8) {
        let index = self.line as usize * SCREEN_WIDTH * 3 + x * 3;
        self.data[index + 0] = color;
        self.data[index + 1] = color;
        self.data[index + 2] = color;
    }

    fn draw_bg(&mut self) {
        let w_trigger = self.win_on && self.wy_trigger && self.wx <= 166;

        // Can be calculated with:
        // self.line as i32 - self.wy as i32
        let winy = if w_trigger {
            self.wy_pos += 1;
            self.wy_pos
        } else {
            -1
        };

        if winy < 0 && self.bgw_on == false {
            return;
        }

        let wintiley = (winy as u16 >> 3) & 31;

        let bgy = self.scy.wrapping_add(self.line);
        let bgtiley = (bgy as u16 >> 3) & 31;

        for x in 0..SCREEN_WIDTH {
            let winx = -((self.wx as i32) - 7) + (x as i32);
            let bgx = self.scx as u32 + x as u32;

            let (tilemapbase, tiley, tilex, pixely, pixelx) = if winy >= 0 && winx >= 0 {
                (
                    self.win_tilemap,
                    wintiley,
                    (winx as u16 >> 3),
                    winy as u16 & 0x07,
                    winx as u8 & 0x07,
                )
            } else {
                (
                    self.bg_tilemap,
                    bgtiley,
                    (bgx as u16 >> 3) & 31,
                    bgy as u16 & 0x07,
                    bgx as u8 & 0x07,
                )
            };

            let tile_number: u8 = self.rbvram0(tilemapbase + tiley * 32 + tilex);

            let offset = if self.bgw_tiles == 0x8000 {
                tile_number as u16
            } else {
                (tile_number as i8 as i16 + 128) as u16
            };
            let tile_address = self.bgw_tiles + offset * 16;

            // Tile data (2 bytes per line)
            let data = tile_address + (pixely * 2);
            let (b1, b2) = (self.rbvram0(data), self.rbvram0(data + 1));

            // Shift bit
            let xbit = 7 - pixelx as u32;

            // Color number
            let color_number = ((b1 >> xbit) & 1) | (((b2 >> xbit) & 1) << 1);

            let color = self.palette_bg[color_number as usize];
            self.set_color(x, color);
        }
    }

    fn draw_sprites(&mut self) {
        if !self.sprite_on {
            return;
        }

        let line = self.line;
        let sprite_size = self.sprite_size;

        let mut sprites_to_draw = Vec::<Sprite>::with_capacity(10);
        let mut sprite_count = 0;

        for i in 0..40 {
            let sprite_address = i * 4;
            let sprite_y = self.oam[sprite_address].wrapping_sub(16);

            // If the sprite is not on the current line, skip it
            if line < sprite_y || line >= sprite_y + sprite_size {
                continue;
            }

            let sprite_x = self.oam[sprite_address + 1].wrapping_sub(8);

            let sprite = Sprite {
                y: sprite_y,
                x: sprite_x,
                tile: self.oam[sprite_address + 2],
                flags: self.oam[sprite_address + 3],
            };

            sprites_to_draw.push(sprite);
            sprite_count += 1;
            if sprite_count >= 10 {
                break;
            }
        }

        for sprite in sprites_to_draw {
            let sprite_x = sprite.x as i32;
            if sprite_x < -7 || sprite_x >= SCREEN_WIDTH as i32 {
                continue;
            }

            let flip_x = sprite.flags & 0x20 != 0;
            let flip_y = sprite.flags & 0x40 != 0;
            let below_bg = sprite.flags & 0x80 != 0;
            let palette = sprite.flags & 0x10 != 0;

            let tile_y = if flip_y {
                7 - (line - sprite.y)
            } else {
                line - sprite.y
            };

            let tile_address = 0x8000 + (sprite.tile as u16) * 16 + (tile_y as u16) * 2;
            let (low_byte, high_byte) =
                (self.rbvram0(tile_address), self.rbvram0(tile_address + 1));

            for x in 0..8 {
                let tile_x = if flip_x { x } else { 7 - x };

                let color_number = ((low_byte >> tile_x) & 1) | (((high_byte >> tile_x) & 1) << 1);
                if color_number == 0 {
                    continue;
                }

                let x = sprite_x + x;
                if x >= SCREEN_WIDTH as i32 {
                    continue;
                }

                if below_bg && self.data[(line as usize * SCREEN_WIDTH + x as usize) * 3] != 255 {
                    continue;
                }

                let color = if palette {
                    self.palette_obp1[color_number as usize]
                } else {
                    self.palette_obp0[color_number as usize]
                };

                let index = (line as usize) * SCREEN_WIDTH * 3 + x as usize * 3;
                self.data[index + 0] = color;
                self.data[index + 1] = color;
                self.data[index + 2] = color;
            }
        }
    }

    pub fn screen_data(&self) -> &[u8; 160 * 144 * 3] {
        &self.data
    }
}

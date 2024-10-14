use core::str;

use rusty_boy_core::{gameboy::Gameboy, keypad::{Key, KeyEvent}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct RustyBoy {
    gameboy: Gameboy,
}

#[wasm_bindgen]
impl RustyBoy {
    
    #[wasm_bindgen(constructor)]
    pub fn new(rom: Vec<u8>, skip_checksum: bool) -> Result<RustyBoy, JsValue> {
        Gameboy::new_from_data(&rom, skip_checksum)
            .map(|gameboy| RustyBoy { gameboy })
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen]
    pub fn run_frame(&mut self) {
        self.gameboy.run_frame();
    }

    #[wasm_bindgen]
    pub fn get_screen_data(&self) -> Vec<u8> {
        self.gameboy.get_screen_data().to_vec()
    }

    #[wasm_bindgen]
    pub fn press_key(&mut self, i: u8) {
        if let Some(key) = keycode_to_key(i) {
            self.gameboy.update_input(KeyEvent::Press(key));
        }
    }

    #[wasm_bindgen]
    pub fn release_key(&mut self, i: u8) {
        if let Some(key) = keycode_to_key(i) {
            self.gameboy.update_input(KeyEvent::Release(key));
        }
    }
    
}

fn keycode_to_key(key: u8) -> Option<Key> {
    match key {
        81 => Some(Key::Left),
        90 => Some(Key::Up),
        68 => Some(Key::Right),
        83 => Some(Key::Down),
        74 => Some(Key::A),
        75 => Some(Key::B),
        32 => Some(Key::Select),
        13 => Some(Key::Start),
        _ => None,
    }
}

use pyo3::{exceptions::PyValueError, prelude::*};
use rusty_boy_core::{gameboy::Gameboy, keypad::{Key, KeyEvent}};

#[pyclass]
struct RustyBoy {
    gameboy: Gameboy,
}

#[pymethods]
impl RustyBoy {
    #[new]
    fn new(file_path: &str, skip_checksum: bool) -> Self {

        if let Ok(gameboy) = Gameboy::new_from_file(file_path, skip_checksum) {
            RustyBoy { gameboy }
        } else {
            panic!("Error loading gameboy")
        }

    }

    fn run_frame(&mut self) -> PyResult<()> {
        self.gameboy.run_frame();
        Ok(())
    }

    /**
     * Test if the gameboy is running
     */
    pub fn press_key(&mut self, key: &str) -> PyResult<()> {
        let key = match key {
            "Up" => Key::Up,
            "Down" => Key::Down,
            "Left" => Key::Left,
            "Right" => Key::Right,
            "A" => Key::A,
            "B" => Key::B,
            "Start" => Key::Start,
            "Select" => Key::Select,
            _ => return Err(PyValueError::new_err("Invalid key"))
        };
        self.gameboy.update_input(KeyEvent::Press(key));
        Ok(())
    }

    pub fn release_key(&mut self, key: &str) -> PyResult<()> {
        let key = match key {
            "Up" => Key::Up,
            "Down" => Key::Down,
            "Left" => Key::Left,
            "Right" => Key::Right,
            "A" => Key::A,
            "B" => Key::B,
            "Start" => Key::Start,
            "Select" => Key::Select,
            _ => return Err(PyValueError::new_err("Invalid key"))
        };
        self.gameboy.update_input(KeyEvent::Release(key));
        Ok(())
    }

    pub fn get_screen(&self) -> PyResult<[u8; 69120]> {
        Ok(self.gameboy.get_screen_data().clone())
    }

}

// This function name should be same as your project name
#[pymodule]
fn rusty_boy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RustyBoy>()?;
    Ok(())
}
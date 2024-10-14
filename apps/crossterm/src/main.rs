use std::{
    io::{stdout, Write},
    path::Path,
    time::Duration,
};

use clap::{Arg, ArgAction, Command};
use crossterm::{
    cursor,
    event::{Event, KeyEventKind},
    style::ResetColor,
    terminal::Clear,
};
use rusty_boy_core::{
    gameboy::Gameboy,
    keypad::{Key, KeyEvent},
};

use crossterm::ExecutableCommand;

const WEIGHT: u32 = 160;
const HEIGHT: u32 = 144;

fn main() -> Result<(), std::io::Error> {
    let matches = Command::new("cli-rusty_boy")
        .version("0.1")
        .author("Flender <tristan.deloeil@gmail.com>")
        .about("Play Gameboy games inside terminal")
        .args([
            Arg::new("file").required(true).index(1),
            Arg::new("info").short('i').action(ArgAction::SetTrue),
            Arg::new("skip-checksup")
                .short('s')
                .action(ArgAction::SetTrue),
        ])
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap();
    let info = matches.get_flag("info");
    let skip_checksum = matches.get_flag("skip-checksup");

    if Path::new(file).exists() == false {
        panic!("The file doesn't exists.")
    }

    let mut gb = Gameboy::new_from_file(file, skip_checksum)?;

    if info == true {
        println!("{}", gb.header());

        // Wait for user input to start the game
        println!("Press any key to start the game");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
    }

    let mut last_time = std::time::Instant::now();
    loop {
        // Handle input
        if let Some(key) = cb_input() {
            gb.update_input(key);
        }

        gb.run_frame();

        // Render screen
        let screen = gb.get_screen_data();
        let mut stdout = stdout();
        stdout.execute(cursor::Hide).unwrap();

        let mut buffer = String::with_capacity(((WEIGHT * HEIGHT * 4) + HEIGHT) as usize);

        for y in 0..144 {
            for x in 0..160 {
                let index = (y * 160 + x) * 3;
                let ch = match screen[index] {
                    0 => "  ",
                    96 => "░░",
                    192 => "▒▒",
                    255 => "▓▓",
                    _ => "XX",
                };
                buffer.push_str(ch)
            }
            buffer.push_str("\x1b[0m\n");
        }
        stdout
            .execute(Clear(crossterm::terminal::ClearType::FromCursorDown))
            .unwrap(); // Effacer à partir du curseur pour éviter le scintillement
        stdout.execute(cursor::MoveTo(0, 0)).unwrap(); // Assurer que le curseur est à la bonne position
        stdout.write_all(buffer.as_bytes()).unwrap();
        stdout.flush().unwrap();

        // Réinitialiser la couleur et montrer le curseur après le rendu
        stdout.execute(ResetColor).unwrap();
        stdout.execute(cursor::Show).unwrap();

        // Attendre pour atteindre 60 FPS
        let elapsed = last_time.elapsed();
        if elapsed < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - elapsed);
        }
        last_time = std::time::Instant::now();
    }
}

fn cb_input() -> Option<KeyEvent> {
    let key = None;
    use crossterm::event::{self, KeyCode, KeyEvent};
    if event::poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(KeyEvent { code, kind, .. }) = event::read().unwrap() {
            let k = match code {
                KeyCode::Char('z') => Some(Key::Up),
                KeyCode::Char('s') => Some(Key::Down),
                KeyCode::Char('q') => Some(Key::Left),
                KeyCode::Char('d') => Some(Key::Right),
                KeyCode::Char(' ') => Some(Key::A),
                KeyCode::Char('g') => Some(Key::B),
                KeyCode::Enter => Some(Key::Start),
                KeyCode::Backspace => Some(Key::Select),
                _ => None,
            };

            if let Some(k) = k {
                match kind {
                    KeyEventKind::Press => return Some(rusty_boy_core::keypad::KeyEvent::Press(k)),
                    KeyEventKind::Release => {
                        return Some(rusty_boy_core::keypad::KeyEvent::Release(k))
                    }
                    _ => {}
                }
            }
        }
    }
    key
}

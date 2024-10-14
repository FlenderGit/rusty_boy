pub mod gameboy;
mod header;
mod cpu;
mod registers;
mod memory;
mod mbc;
pub mod keypad;
mod gpu;
mod timer;
mod serial;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}

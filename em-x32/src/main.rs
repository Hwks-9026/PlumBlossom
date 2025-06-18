#[allow(dead_code)]
#[allow(warnings)]
use std::{
    env,
    fs::{self, File},
    io::Read,
};
mod emulator;
mod memory;
mod registers;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() != 1 {
        eprintln!("Expected 1 Argument: Binary File to Load to Memory");
        return;
    }

    let binary_file_path = args[0].clone();
    let rom: Vec<u8> = file_as_byte_vec(binary_file_path);
    crate::emulator::start(rom);
}

fn file_as_byte_vec(binary_file_path: String) -> Vec<u8> {
    let mut f = File::open(&binary_file_path)
        .expect(&format!("No file found with path {}", binary_file_path));
    let metadata = fs::metadata(&binary_file_path).expect("Unable to read file metadata.");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

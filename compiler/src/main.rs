use clap::Parser;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io;
use std::io::{BufWriter, Seek, SeekFrom, Write};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Output file name
    #[arg(short = 'o', long = "output")]
    output: String,

    /// Input directory name
    #[arg(short = 'i', long = "input")]
    input: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    const TOTAL_SIZE: u64 = 4_294_967_296;
    const CHUNK_SIZE: usize = 1024 * 1024; // 1 MiB
    let zero_chunk = vec![0u8; CHUNK_SIZE];

    let file = File::create(&args.output)?;
    let mut writer = BufWriter::new(file);

    let mut written = 0;
    while written < TOTAL_SIZE {
        writer.write_all(&zero_chunk)?;
        written += CHUNK_SIZE as u64;
    }

    writer.flush()?;

    return assemble_from_directory(args);
}

fn assemble_from_directory(args: Args) -> io::Result<()> {
    for entry in fs::read_dir(&args.input)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("as") {
            let contents = fs::read_to_string(&path)?;
            parse_file(&args.output, contents)?;
        }
    }
    Ok(())
}
fn parse_file(rom_name: &String, contents: String) -> io::Result<()> {
    let mut file: File = OpenOptions::new().read(true).write(true).open(rom_name)?;

    //RESET VECTOR WRITE TO START OF GENERAL MEMORY
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&[0x00, 0x09, 0x60, 0x40])?;

    //JUMP TO START OF GENERAL MEMORY
    file.seek(SeekFrom::Start(614464))?;

    let mut tokens: Vec<String> = contents
        .split_whitespace()
        .map(|str| str.to_string())
        .collect();

    let mut byte_vec: Vec<u8> = Vec::new();

    for tok in tokens.iter() {
        for byte in parse_byte_from_token(&tok) {
            byte_vec.push(byte);
        }
    }
    file.write_all(&byte_vec)?;

    Ok(())
}

fn parse_byte_from_token(tok: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut token_match_failed: bool = false;
    match tok {
        //INSTRUCTIONS
        "NOP" => bytes.push(0x00),
        "JMP" => bytes.push(0x01),
        "CAL" => bytes.push(0x02),
        "RET" => bytes.push(0x03),
        "JNZ" => bytes.push(0x10),
        "JIZ" => bytes.push(0x11),
        "PSH" => bytes.push(0x12),
        "POP" => bytes.push(0x13),
        "NOT" => bytes.push(0x14),
        "SHL" => bytes.push(0x15),
        "SHR" => bytes.push(0x16),
        "MOV" => bytes.push(0x20),
        "MVI" => bytes.push(0x21),
        "LDA" => bytes.push(0x22),
        "STA" => bytes.push(0x23),
        "ADD" => bytes.push(0x30),
        "SUB" => bytes.push(0x31),
        "MUL" => bytes.push(0x32),
        "DIV" => bytes.push(0x33),
        "AND" => bytes.push(0x34),
        "XOR" => bytes.push(0x35),
        //REGISTERS
        "%r0" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
        }
        "%r1" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x01);
        }
        "%r2" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x02);
        }
        "%r3" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x03);
        }
        "%r4" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x04);
        }
        "%r5" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x05);
        }
        "%r6" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x06);
        }
        "%r7" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x07);
        }
        "%r8" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x08);
        }
        "%r9" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x09);
        }
        "%r10" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x0A);
        }
        "%r11" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x0B);
        }
        "%r12" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x0C);
        }
        "%r13" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x0D);
        }
        "%r14" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x0E);
        }
        "%r15" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x0F);
        }
        "%J" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x10);
        }
        "%PC" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x20);
        }
        "%SP" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x30);
        }
        "%IP" => {
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0x40);
        }
        "" => {
            return bytes;
        }
        _ => token_match_failed = true,
    }
    if token_match_failed {
        match parse_number(tok) {
            Some(num) => {
                let b0 = ((num >> 24) & 0xFF) as u8;
                let b1 = ((num >> 16) & 0xFF) as u8;
                let b2 = ((num >> 8) & 0xFF) as u8;
                let b3 = (num & 0xFF) as u8;

                bytes.push(b0);
                bytes.push(b1);
                bytes.push(b2);
                bytes.push(b3);
            }
            None => {}
        }
    }
    return bytes;
}

fn parse_number(s: &str) -> Option<u32> {
    if let Some(hex) = s.strip_prefix("0x").or(s.strip_prefix("0X")) {
        match u32::from_str_radix(hex, 16) {
            Err(_) => None,
            Ok(num) => Some(num),
        }
    } else {
        match s.parse::<u32>() {
            Err(_) => None,
            Ok(num) => Some(num),
        }
    }
}

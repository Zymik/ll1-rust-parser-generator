mod parser_generator;

use std::{fs, io};
use std::fs::File;
use std::io::Write;
use crate::parser_generator::generate_parser_from_string;

fn main() -> io::Result<()> {

    println!("Input grammar file");
    let mut file_in = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut file_in)?;
    file_in = file_in.trim().to_string();

    println!("Output file");
    let mut file_out = String::new();
    stdin.read_line(&mut file_out)?;
    file_out = file_out.trim().to_string();

    let file_in_content = fs::read_to_string(file_in)?;
    let parser = generate_parser_from_string(file_in_content);

    let mut file = File::create(file_out)?;
    file.write_all(&parser.into_bytes())?;

    Ok(())
}

use std::io::Error;
use arithmetic_coding::{decode_routine, encode_routine};

fn cmd_err () -> Result<(),Error> {
    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Usage: <bin_name> [-e | -d] --"))
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut input_handle = stdin.lock();
    let mut output_handle = stdout.lock();

    if args.len() != 2 {
        cmd_err()?;
    } else {
        match args[1].as_str() {
            "-e" => encode_routine(&mut input_handle, &mut output_handle)?,
            "-d" => decode_routine(&mut input_handle, &mut output_handle)?,
            _ => cmd_err()?,
        }
    }

    Ok(())
}

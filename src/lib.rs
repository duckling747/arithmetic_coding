#![forbid(unsafe_code)]

use std::io::{BufRead, Write};

use io::{Next, Push};

mod codec;
mod fenwick;
mod io;
mod bitvec;

const SIZE: usize = 257;
const EOF: usize = 256;

pub fn encode_routine<I,O> (input_handle: &mut I, output_handle: &mut O) -> Result<(), std::io::Error>
where I: BufRead, O: Write
{
    let mut encoder = codec::ArithmeticEncoder::new(SIZE);
    let mut bits_out = io::Output::new(output_handle, 8000);
    let mut bits_in = io::Input::new(input_handle)?;

    while let Some(byte) = bits_in.next_byte()? {
        encoder.encode(byte as usize, &mut bits_out)?;
        encoder.discover(byte as usize);
    }
    encoder.encode(EOF, &mut bits_out)?;
    encoder.finish(&mut bits_out)?;
    bits_out.flush()?;

    Ok(())
}

pub fn decode_routine<I,O> (input_handle: &mut I, output_handle: &mut O) -> Result<(), std::io::Error>
where I: BufRead, O: Write
{
    let mut decoder = codec::ArithmeticDecoder::new(SIZE);
    let mut bits_out = io::Output::new(output_handle, 8000);
    let mut bits_in = io::Input::new(input_handle)?;

    decoder.begin(&mut bits_in)?;
    loop {
        let s = decoder.decode(&mut bits_in)?;
        if s == EOF {
            break;
        }
        bits_out.push_byte(s as u8)?;
        decoder.discover(s);
    }
    bits_out.flush()?;
    Ok(())
}

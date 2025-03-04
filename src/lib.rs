#![forbid(unsafe_code)]

use std::{collections::VecDeque, io::{BufRead, Write}};

use bitvec::BitVec;
use io::{Next, Push};

mod codec;
mod fenwick;
mod io;
mod bitvec;

const SIZE: usize = 257;
const EOF: usize = 256;

const BUFFER_SIZE: usize = 16000;

/**
 * The encoding routine for arithmetic coding. Takes an input and an output.
 */
pub fn encode_routine<I,O> (input_handle: &mut I, output_handle: &mut O) -> Result<(), std::io::Error>
where I: BufRead, O: Write
{
    let mut encoder = codec::ArithmeticEncoder::new(SIZE);
    let mut bits_out = io::Output::new(output_handle, BUFFER_SIZE);
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

/**
 * The decoding routine for arithmetic coding. Takes an input and an output.
 */
pub fn decode_routine<I,O> (input_handle: &mut I, output_handle: &mut O) -> Result<(), std::io::Error>
where I: BufRead, O: Write
{
    let mut decoder = codec::ArithmeticDecoder::new(SIZE);
    let mut bits_out = io::Output::new(output_handle, BUFFER_SIZE);
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

/**
 * Convenience iterator for arithmetic decoding. Panics if IO errors occur during _next_. Take care that you know what you're doing when using iterator methods on this, such as _filter_ or _step_by_, etc.
 */
pub struct ArithmeticStreamDecoder<'a,I>
where I: BufRead
{
    decoder: codec::ArithmeticDecoder,
    bits_in: io::Input<&'a mut I>,
}

impl<'a,I> ArithmeticStreamDecoder<'a,I>
where I: BufRead
{
    pub fn new(input: &'a mut I) -> Result<Self, std::io::Error>
    {
        let mut decoder = codec::ArithmeticDecoder::new(SIZE);
        let mut bits_in = io::Input::new(input)?;

        decoder.begin(&mut bits_in)?;

        Ok(Self { decoder, bits_in })
    }
}

impl<I> Iterator for ArithmeticStreamDecoder<'_,I>
where I: BufRead
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item>
    {
        let s = self.decoder.decode(&mut self.bits_in).unwrap();
        if s == EOF {
            return None;
        }
        self.decoder.discover(s);
        Some(s as u8)
    }
}

struct SmallBuffer {
    data: BitVec,
    q: VecDeque<u8>,
    eof: bool,
}

impl SmallBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            data: BitVec::with_capacity(capacity),
            q: VecDeque::new(),
            eof: false,
        }
    }
}

impl Push for SmallBuffer {
    fn push_bit(&mut self, bit: bool) -> Result<(), std::io::Error> {
        if self.data.len() == self.data.capacity() {
            self.flush()?;
        }
        self.data.push(bit);
        Ok(())
    }
    fn push_byte(&mut self, byte: u8) -> Result<(), std::io::Error> {
        for i in (0..8).rev() {
            self.data.push(((byte >> i) & 1) != 0);
        }
        Ok(())
    }
    fn flush (&mut self) -> Result<(), std::io::Error> {
        if !self.data.is_empty() {
            self.q.write_all(&self.data.get_bytes())?;
            self.data.clear();
        }
        Ok(())
    }
}

/**
 * Convenience iterator for arithmetic encoding. Panics if IO errors occur during _next_. Take care that you know what you're doing when using iterator methods on this, such as _filter_ or _step_by_, etc.
 */
pub struct ArithmeticStreamEncoder<'a, I>
where I: BufRead
{
    encoder: codec::ArithmeticEncoder,
    bits_in: io::Input<&'a mut I>,
    buf: SmallBuffer,
}

impl<'a, I> ArithmeticStreamEncoder<'a, I>
where I: BufRead
{
    pub fn new(input: &'a mut I) -> Result<Self, std::io::Error>
    {
        let encoder = codec::ArithmeticEncoder::new(SIZE);
        let bits_in = io::Input::new(input)?;
        let buf = SmallBuffer::new(BUFFER_SIZE/4);

        Ok(Self { encoder, bits_in, buf })
    }
}

impl<I> Iterator for ArithmeticStreamEncoder<'_, I>
where I: BufRead
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item>
    {
        while self.buf.q.is_empty() && !self.buf.eof {
            if let Some(byte) = self.bits_in.next_byte().unwrap() {
                self.encoder.encode(byte as usize, &mut self.buf).unwrap();
                self.encoder.discover(byte as usize);
            } else {
                self.encoder.encode(EOF, &mut self.buf).unwrap();
                self.encoder.finish(&mut self.buf).unwrap();
                self.buf.flush().unwrap();
                self.buf.eof = true;
            }
        }

        self.buf.q.pop_front()
    }
}






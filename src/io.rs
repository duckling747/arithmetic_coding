use std::io::{BufRead, Write};

use crate::bitvec::BitVec;

pub struct Input <I>
where I: BufRead
{
    input_handle: I,
    input_bits: BitVec,
    idx: usize
}

impl<I> Input<I>
where I: BufRead
{
    pub fn new (mut input_handle: I) -> Result<Input<I>, std::io::Error> {
        let b = input_handle.fill_buf()?;
        let input_bits = BitVec::from_bytes(b);
        let len = b.len();
        input_handle.consume(len);
        Ok(Input {
            input_handle,
            input_bits,
            idx: 0
        })
    }
}

pub trait Next {
    fn next_bit (&mut self) -> Result<Option<bool>, std::io::Error>;
    fn next_byte (&mut self) -> Result<Option<u8>, std::io::Error>;
}

impl<I> Next for Input<I>
where I: BufRead
{
    fn next_bit (&mut self) -> Result<Option<bool>, std::io::Error> {
        if self.idx >= self.input_bits.len() {
            let b = self.input_handle.fill_buf()?;
            self.input_bits = BitVec::from_bytes(b);
            let len = b.len();
            self.input_handle.consume(len);
            self.idx = 0;
        }
        if let Some(o) = self.input_bits.get(self.idx) {
            self.idx += 1;
            Ok(Some(o))
        } else {
            Ok(None)
        }
    }

    fn next_byte (&mut self) -> Result<Option<u8>, std::io::Error>
    {
        let mut byte: u8 = 0;
        for shft in (0..8).rev() {
            if let Some(b) = self.next_bit()? {
                byte |= (b as u8) << shft;
            } else {
                return Ok(None);
            }
        }
        Ok(Some(byte))
    }
}


pub struct Output<'a, O>
where O: Write
{
    output_handle: &'a mut O,
    output_bits: BitVec
}

impl <'a, O> Output <'a, O>
where O: Write
{
    pub fn new(output_handle: &'a mut O, buffer_capacity: usize) -> Self {
        assert!(buffer_capacity % 8 == 0);
        Output {
            output_handle,
            output_bits: BitVec::with_capacity(buffer_capacity)
        }
    }

    pub fn flush (&mut self) -> Result<(), std::io::Error> {
        if !self.output_bits.is_empty() {
            self.output_handle.write_all(&self.output_bits.get_bytes())?;
            self.output_bits.clear();
        }
        Ok(())
    }
}

pub trait Push {
    fn push_bit(&mut self, bit: bool) -> Result<(), std::io::Error>;
    fn push_byte(&mut self, byte: u8) -> Result<(), std::io::Error>;
}

impl<'a, O> Push for Output<'a, O>
where O: Write
{
    fn push_bit(&mut self, bit: bool) -> Result<(), std::io::Error>
    {
        if self.output_bits.len() == self.output_bits.capacity() {
            self.flush()?;
        }
        self.output_bits.push(bit);
        Ok(())
    }

    fn push_byte(&mut self, byte: u8) -> Result<(), std::io::Error>
    {
        for shft in (0..8).rev() {
            self.push_bit(((byte >> shft) & 1) != 0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;
    #[test]
    fn test_input() {
        let file = std::fs::File::open("./war_and_peace.txt").unwrap();
        let bufreader = std::io::BufReader::new(file);
        let mut input = Input::new(bufreader).unwrap();
        let mut bytes_a: Vec<u8> = Vec::new();
        while let Some(b) = input.next_byte().unwrap() {
            bytes_a.push(b);
        }

        let mut bytes_b: Vec<u8> = Vec::new();
        let file = std::fs::File::open("./war_and_peace.txt").unwrap();
        let bufreader = std::io::BufReader::new(file);
        for b in bufreader.bytes() {
            bytes_b.push(b.unwrap());
        }

        assert_eq!(bytes_a, bytes_b);
    }
    #[test]
    fn test_output() {
        let mut stdout: Vec<u8> = Vec::new();
        let mut output = Output::new(&mut stdout, 4000);
        let mut v: Vec<u8> = Vec::new();
        let file = std::fs::File::open("./war_and_peace.txt").unwrap();
        let bufreader = std::io::BufReader::new(file);
        for b in bufreader.bytes() {
            let a = b.unwrap();
            v.push(a);
            output.push_byte(a).unwrap();
        }
        output.flush().unwrap();

        assert_eq!(stdout, v);
    }
}








use num_traits::PrimInt;
use crate::{fenwick::FenwickTree, io};

type Value = u32;

const VALB: u8 = 16;
const TOP: Value = (1 << VALB) - 1;
const FIRST_QTR: Value = TOP / 4 + 1;
const HALF: Value = 2 * FIRST_QTR;
const THIRD_QTR: Value = 3 * FIRST_QTR;

const MAX_FREQUENCY: Value = (u16::max_value()>>2) as Value;

pub struct ArithmeticEncoder<Value>
where Value: PrimInt
{
    model: FenwickTree<Value>,
    low: Value,
    high: Value,
    pending: Value
}

impl ArithmeticEncoder<Value>
{
    pub fn new (size: usize) -> Self {
        ArithmeticEncoder {
            model: FenwickTree::<Value>::new(size, Some(1)),
            low: 0,
            high: TOP,
            pending: 0
        }
    }

    pub fn discover(&mut self, s: usize) -> () {
        if self.model.total() == MAX_FREQUENCY {
            self.model.scale(2);
        }
        self.model.add(s, 1);
    }

    pub fn encode(&mut self, s: usize, bits_out: &mut impl io::Push) -> Result<(), std::io::Error>
    {
        let lower = self.model.sum(s-1);
        let upper = self.model.sum(s);
        let denom = self.model.total();

        let range = (self.high - self.low) + 1;
        self.high = self.low + (range * upper) / denom - 1;
        self.low  = self.low + (range * lower) / denom;
        loop {
            if self.high < HALF {
                self.write_bit_plus_pending(false, bits_out)?;
            } else if self.low >= HALF {
                self.write_bit_plus_pending(true, bits_out)?;
            } else if (self.low >= FIRST_QTR) && (self.high < THIRD_QTR) {
                self.pending += 1;
                self.low -= FIRST_QTR;
                self.high -= FIRST_QTR;
            } else {
                break;
            }
            self.low = (self.low << 1) & TOP;
            self.high = (self.high << 1) & TOP | 1;
        }
        Ok(())
    }

    fn write_bit_plus_pending(&mut self, bit: bool, bits_out: &mut impl io::Push) -> Result<(), std::io::Error>
    {
        bits_out.push_bit(bit)?;
        for _ in 0..self.pending {
            bits_out.push_bit(!bit)?;
        }
        self.pending = 0;
        Ok(())
    }

    pub fn finish(&mut self, bits_out: &mut impl io::Push) -> Result<(), std::io::Error> {
        self.pending += 1;
        if self.low < FIRST_QTR {
            self.write_bit_plus_pending(false, bits_out)?;
        } else {
            self.write_bit_plus_pending(true, bits_out)?;
        }
        Ok(())
    }
}

pub struct ArithmeticDecoder<Value>
where Value: PrimInt
{
    model: FenwickTree<Value>,
    low: Value,
    high: Value,
    value: Value,
}

impl ArithmeticDecoder<Value>
{
    pub fn new (size: usize) -> Self {
        ArithmeticDecoder {
            model: FenwickTree::<Value>::new(size, Some(1)),
            low: 0,
            high: TOP,
            value: 0
        }
    }

    pub fn begin (&mut self, bits_in: &mut impl io::Next) -> Result<(), std::io::Error>
    {
        for _ in 0..VALB {
            if let Some(b) = bits_in.next_bit()? {
                self.value = (self.value << 1) | b as Value;
            } else {
                break;
            }
        }
        Ok(())
    }

    pub fn discover(&mut self, s: usize) -> () {
        if self.model.total() == MAX_FREQUENCY {
            self.model.scale(2);
        }
        self.model.add(s, 1);
    }

    pub fn decode(&mut self, bits_in: &mut impl io::Next) -> Result<usize, std::io::Error>
    {
        let range = (self.high - self.low) + 1;
        let denom = self.model.total();
        let cum = (((self.value - self.low) + 1) * denom - 1) / range;
        let s = self.model.upper(cum);
        let upper = self.model.sum(s);
        let lower = self.model.sum(s-1);

        self.high = self.low + (range * upper) / denom - 1;
        self.low  = self.low + (range * lower) / denom;

        loop {
            if self.high < HALF {
                // Do nothing
            } else if self.low >= HALF {
                self.value -= HALF;
                self.low -= HALF;
                self.high -= HALF;
            } else if (self.low >= FIRST_QTR) && (self.high < THIRD_QTR) {
                self.value -= FIRST_QTR;
                self.low -= FIRST_QTR;
                self.high -= FIRST_QTR;
            } else {
                break;
            }
            self.low <<= 1;
            self.high = (self.high << 1) | 1;

            if let Some(b) = bits_in.next_bit()? {
                self.value = (self.value << 1) | b as Value;
            } else {
                // input is "garbage" (zeroes)
                self.value <<= 1;
            }
        }
        Ok(s)
    }
}





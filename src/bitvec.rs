
pub struct BitVec {
    data: Box<[u64]>,
    i: usize
}

impl BitVec {
    pub fn from_bytes(b: &[u8]) -> Self {
        let chunks = b.chunks_exact(8);
        let remainder = chunks.remainder();

        let mut data: Vec<u64> = chunks.into_iter()
            .map(|n| u64::from_be_bytes(
                n[0..8].try_into().unwrap())
            )
            .map(|n| n.reverse_bits())
            .collect();
        let mut last = [0u8; 8];
        let len = 8.min(remainder.len());
        last[0..len].copy_from_slice(&remainder[0..len]);
        data.push(u64::from_be_bytes(last)
            .reverse_bits());
        let i = b.len()*8;
        BitVec { data: data.into_boxed_slice(), i }
    }

    pub fn with_capacity(cap: usize) -> Self {
        BitVec {
            data: vec![0; cap.div_ceil(64)].into_boxed_slice(),
            i: 0
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()*64
    }

    pub fn push (&mut self, bit: bool) -> () {
        assert!(self.i < self.capacity());
        let bit = bit as u64;
        self.data[self.i/64] |= bit << self.i%64;
        self.i+=1;
    }

    pub fn get (&self, idx: usize) -> Option<bool> {
        if idx >= self.i {
            return None
        }
        Some((self.data[idx/64] >> idx%64) & 1 != 0)
    }

    pub fn is_empty (&self) -> bool {
        self.i == 0
    }

    pub fn len (&self) -> usize {
        self.i
    }

    pub fn clear(&mut self) -> () {
        self.i = 0;
        self.data.fill(0);
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.data.iter()
            .map(|b| b.reverse_bits())
            .flat_map(|n| n.to_be_bytes())
            .enumerate()
            .take_while(|e| e.0*8 < self.i)
            .map(|e| e.1)
            .collect::<Vec<u8>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bitvec_1() {
        let mut bitvec = BitVec::with_capacity(10);
        let arr = vec![true,true,true,true,false,false,false,false,true,false];
        for e in arr.iter() {
            bitvec.push(*e);
        }
        for (i,e) in arr.iter().enumerate() {
            assert_eq!(bitvec.get(i).unwrap(), *e);
        }
    }
    #[should_panic]
    #[test]
    fn test_bitvec_2() {
        let mut bv = BitVec::with_capacity(5);
        for _ in 0..65 {
            bv.push(true);
        }
    }
    #[test]
    fn test_bitvec_3() {
        let mut bv = BitVec::with_capacity(64);
        for _ in 0..64 {
            bv.push(true);
        }
    }
    #[should_panic]
    #[test]
    fn test_bitvec_4() {
        let mut bv = BitVec::with_capacity(127);
        for _ in 0..129 {
            bv.push(true);
        }
    }
    #[test]
    fn test_get_bytes_1() {
        let bytes = vec![9,8,7,6];
        let bitvec = BitVec::from_bytes(&bytes);
        let by = bitvec.get_bytes();
        assert_eq!(bytes, by);
    }
    #[test]
    fn test_get_bytes_2() {
        let bytes = vec![
            0b00000001,
            0b00000010
        ];
        let mut bitvec = BitVec::with_capacity(64);
        for b in bytes.iter() {
            for i in (0..8).rev() {
                bitvec.push(((b >> i)&1) != 0);
            }
        }
        let by = bitvec.get_bytes();
        assert_eq!(bytes, by);
    }
    #[test]
    fn test_next_bit_1() {
        let bytes = vec![
            0b00000001,
            0b00000010,
            0b11111111,
            0b10101010,
            0b01010101,
            0b00000000,
            0b00001000,
            0b00000001,
        ];
        let mut bitvec = BitVec::with_capacity(64);
        for b in bytes.iter() {
            for i in (0..8).rev() {
                bitvec.push(((b >> i)&1) != 0);
            }
        }
        let mut k = 0;
        for b in bytes.iter() {
            for j in (0..8).rev() {
                let bit = ((b >> j)&1) != 0;
                if let Some(b) = bitvec.get(k) {
                    assert_eq!(bit, b);
                }
                k+=1;
            }
        }
    }
    #[test]
    fn test_next_bit_2() {
        let bytes = vec![
            0b00000001,
            0b00000010,
            0b11111111,
            0b10101010,
            0b01010101,
            0b00000000,
            0b00001000,
            0b00000001,
        ];
        let bitvec = BitVec::from_bytes(&bytes);
        let mut k = 0;
        for b in bytes.iter() {
            for j in (0..8).rev() {
                let bit = ((b >> j)&1) != 0;
                if let Some(b) = bitvec.get(k) {
                    assert_eq!(bit, b);
                }
                k+=1;
            }
        }
    }
    #[test]
    fn test_next_message() {
        let bytes = "asdoijfasöodfjaosidjfioasd".as_bytes();
        let bitvec = BitVec::from_bytes(&bytes);
        assert_eq!(bytes, bitvec.get_bytes());
    }
}






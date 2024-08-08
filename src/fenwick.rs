use std::ops::{AddAssign, SubAssign};

use num_traits::int::PrimInt;

pub struct FenwickTree<T>
where T: PrimInt
{
    tree: Box<[T]>
}

const fn lsb(i: usize) -> usize
{
    //1usize << i.trailing_zeros()
    let i = i as isize;
    (i & (-i)) as usize
}

#[allow(dead_code)]
impl<T> FenwickTree<T>
where T: PrimInt + AddAssign + SubAssign + From<u8>
{
    pub fn new(size: usize, initial_value: Option<T>) -> Self {
        let mut tree: Vec<T> = vec![0.into(); size+1];
        for i in 1..tree.len() {

            tree[i] += initial_value.unwrap_or(1.into());
            let v = tree[i];

            let parent_idx = i + lsb(i);
            if let Some(r) = tree.get_mut(parent_idx) {
                *r += v;
            }
        }
        FenwickTree { tree: tree.into_boxed_slice() }
    }

    pub fn sum(&self, mut i: usize) -> T {
        assert!(i < self.tree.len()-1);
        let mut ret: T = 0.into();
        i+=1;
        while i > 0 {
            ret += self.tree[i];
            i -= lsb(i);
        }
        ret
    }

    pub fn add(&mut self, mut i: usize, amt: T) -> () {
        assert!(i < self.tree.len()-1);
        i+=1;
        while i < self.tree.len() {
            self.tree[i] += amt;
            i += lsb(i);
        }
    }

    pub fn sub(&mut self, mut i: usize, amt: T) -> () {
        assert!(i < self.tree.len()-1);
        i+=1;
        while i < self.tree.len() {
            self.tree[i] -= amt;
            i += lsb(i);
        }
    }

    pub fn freq(&self, mut i: usize) -> T {
        assert!(i < self.tree.len()-1);
        i+=1;
        let mut ret: T = self.tree[i];
        let z = i - lsb(i);
        i-=1;
        while i > z {
            ret -= self.tree[i];
            i -= lsb(i);
        }
        ret
    }

    pub fn scale(&mut self, factor: T) -> () {
        for i in (0..(self.tree.len()-1)).rev() {
            let a = self.freq(i)/factor;
            self.sub(i, a);
        }
    }

    fn binary_search(&self, mut sum: T, compare: impl Fn(T,T) -> bool) -> usize {
        let mut mask = (self.tree.len()-1).next_power_of_two();
        let mut i = 0;

        while mask > 0 {
            let k = i + mask;
            if k < self.tree.len() && compare(sum, self.tree[k]) {
                i = k;
                sum -= self.tree[k];
            }
            mask >>= 1;
        }
        i
    }

    pub fn lower(&self, sum: T) -> usize {
        self.binary_search(sum, |a,b|{a>b})
    }

    pub fn upper (&self, sum: T) -> usize {
        self.binary_search(sum, |a,b|{a>=b})
    }

    pub fn total (&self) -> T {
        self.sum(self.tree.len()-2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_1 () {
        let ft = FenwickTree::<u32>::new(1, None);
        assert_eq!(ft.tree[0], 0);
        assert_eq!(ft.tree[1], 1);
    }
    #[test]
    fn test_create_2 () {
        let ft = FenwickTree::<i16>::new(1, Some(5));
        assert_eq!(ft.tree[0], 0);
        assert_eq!(ft.tree[1], 5);
    }
    #[test]
    #[should_panic]
    fn test_create_3 () {
        let mut ft = FenwickTree::<u32>::new(257, Some(1));
        ft.add(257, 1);
    }
    #[test]
    fn test_create_4 () {
        let mut ft = FenwickTree::<u32>::new(257, Some(1));
        ft.add(256, 1);
    }
    #[test]
    fn test_sum_medium () {
        let size = 8000;
        let ft = FenwickTree::<u32>::new(size, None);
        for i in 1..=size {
            assert_eq!(ft.sum(i-1), i as u32);
        }
    }
    #[test]
    fn test_add_medium () {
        let size = 8000;
        let mut ft = FenwickTree::<u64>::new(size, Some(2));
        assert_eq!(ft.sum(size-1), 2*size as u64);
        ft.add(599, 1);
        ft.add(599, 1);
        ft.add(777, 1);
        ft.add(111, 1);
        ft.add(7999, 1);
        assert_eq!(ft.sum(size-1), 5+2*size as u64);
    }
    #[test]
    #[should_panic]
    fn test_bounds_1 () {
        let size = 8000;
        let mut ft = FenwickTree::<u64>::new(size, Some(2));
        ft.add(8000, 1);
    }
    #[test]
    #[should_panic]
    fn test_bounds_2 () {
        let size = 8000;
        let ft = FenwickTree::<u64>::new(size, Some(4));
        ft.sum(8000);
    }
    #[test]
    fn test_scale_1 () {
        let size = 8;
        let mut ft = FenwickTree::<u64>::new(size, Some(2));
        ft.scale(2);
        assert_eq!(ft.sum(7), 8);
    }
    #[test]
    fn test_scale_2 () {
        let size = 8;
        let mut ft = FenwickTree::<u64>::new(size, Some(4));
        ft.scale(2);
        assert_eq!(ft.sum(7), 16);
    }
    #[test]
    fn test_scale_3 () {
        let size = 8;
        let mut ft = FenwickTree::<u64>::new(size, Some(3));
        ft.scale(2);
        assert_eq!(ft.sum(7), 16);
    }
    #[test]
    fn test_lower_1 () {
        let size = 8;
        let ft = FenwickTree::<u64>::new(size, None);
        assert_eq!(ft.lower(2), 1);
    }
    #[test]
    fn test_lower_2 () {
        let size = 8;
        let mut ft = FenwickTree::<u64>::new(size, Some(0));
        ft.add(5, 1);
        assert_eq!(ft.lower(1), 5);
        assert_eq!(ft.lower(0), 0);
    }
    #[test]
    fn test_lower_3 () {
        let size = 8;
        let mut ft = FenwickTree::<u64>::new(size, Some(0));
        ft.add(5, 2);
        ft.add(6, 1);
        assert_eq!(ft.lower(3), 6);
        assert_eq!(ft.lower(2), 5);
        assert_eq!(ft.sum(4), 0);
        assert_eq!(ft.lower(1), 5);
    }
    #[test]
    fn test_lower_4 () {
        let size = 8;
        let mut ft = FenwickTree::<u64>::new(size, Some(0));
        ft.add(0, 1);
        ft.add(2, 1);
        assert_eq!(ft.lower(1), 0);
        assert_eq!(ft.lower(2), 2);
        assert_eq!(ft.lower(3), 8);
    }
    #[test]
    fn test_upper_1 () {
        let size = 8;
        let mut ft = FenwickTree::<u64>::new(size, Some(0));
        ft.add(0, 1);
        ft.add(2, 1);
        assert_eq!(ft.upper(1), 2);
        assert_eq!(ft.upper(2), 8);
        assert_eq!(ft.upper(3), 8);
    }
    #[test]
    fn test_total_1 () {
        let size = 16;
        let ft = FenwickTree::<u64>::new(size, Some(1));
        assert_eq!(ft.total(), 16);
    }
    #[test]
    fn test_total_2 () {
        let size = 127;
        let ft = FenwickTree::<u32>::new(size, Some(1));
        assert_eq!(ft.total(), 127);
    }
    #[test]
    fn test_freq_1 () {
        let size = 127;
        let ft = FenwickTree::<u32>::new(size, Some(1));
        for i in 0..127 {
            assert_eq!(ft.freq(i), 1);
        }
    }
    #[test]
    fn test_freq_2 () {
        let size = 127;
        let ft = FenwickTree::<u32>::new(size, Some(10));
        for i in 0..127 {
            assert_eq!(ft.freq(i), 10);
        }
    }
}










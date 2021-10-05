#![allow(dead_code)]
use bit_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
        com: [(i32, usize); q]
    }
    let mut bit = Bit::<i64, _, _>::new(200_000, |a, b| a + b, |e| -e, 0);
    for &(query, num) in &com {
        if query == 1 {
            bit.add(num - 1, 1);
        } else {
            let index = num;
            let index = {
                let mut upper_bound = bit.len() - 1;
                let mut lower_bound = 0;
                while upper_bound - lower_bound > 1 {
                    let est = (upper_bound + lower_bound) / 2;
                    if bit.sum(0..est + 1) >= index as i64 {
                        upper_bound = est
                    } else {
                        lower_bound = est
                    }
                }
                if bit.sum(0..lower_bound + 1) >= index as i64 {
                    lower_bound
                } else {
                    upper_bound
                }
            } + 1;
            bit.add(index - 1, (bit.inverse)(1));
            println!("{}", index)
        }
    }
}

pub mod bit_library {
    #[derive(Debug)]
    pub struct Bit<T, F, I>
    where
        F: Fn(T, T) -> T,
        I: Fn(T) -> T,
    {
        tree: Vec<T>,
        f: F,
        pub inverse: I,
        unit: T,
    }

    impl<T, F, I> Bit<T, F, I>
    where
        T: Copy,
        F: Fn(T, T) -> T,
        I: Fn(T) -> T,
    {
        #[inline]
        pub fn new(n: usize, f: F, inverse: I, unit: T) -> Self {
            Self {
                tree: vec![unit; n],
                f,
                inverse,
                unit,
            }
        }

        #[inline]
        pub fn sum(&self, range: std::ops::Range<usize>) -> T {
            let s = if range.end == 0 {
                self.unit
            } else {
                self.sum_inner(range.end)
            };
            let s_inverse = (self.inverse)(if range.start == 0 {
                self.unit
            } else {
                self.sum_inner(range.start)
            });
            (self.f)(s, s_inverse)
        }

        fn sum_inner(&self, mut index: usize) -> T {
            let mut sum = self.unit;
            while index > 0 {
                sum = (self.f)(sum, self[index - 1]);
                index -= (index as i128 & -(index as i128)) as usize;
            }
            sum
        }

        #[inline]
        pub fn add(&mut self, index: usize, value: T) {
            let mut index = index + 1;
            while index <= self.tree.len() {
                self[index - 1] = (self.f)(self[index - 1], value);
                index += (index as i128 & -(index as i128)) as usize;
            }
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.tree.len()
        }

        #[inline]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl<T, F, I> std::ops::Index<usize> for Bit<T, F, I>
    where
        F: Fn(T, T) -> T,
        I: Fn(T) -> T,
    {
        type Output = T;
        #[inline]
        fn index(&self, index: usize) -> &Self::Output {
            &self.tree[index]
        }
    }

    impl<T, F, I> std::ops::IndexMut<usize> for Bit<T, F, I>
    where
        F: Fn(T, T) -> T,
        I: Fn(T) -> T,
    {
        #[inline]
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.tree[index]
        }
    }
}

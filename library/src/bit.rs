#![allow(dead_code)]

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

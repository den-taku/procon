#![allow(dead_code)]

pub mod rmq_library {

    /// Range Minimum Query Tree.
    /// This tree sustain n elements.
    /// This has update (Rmq[i] <- new_value) and find (Min[i..j]).
    #[derive(Debug, Clone)]
    pub struct Rmq<T> {
        tree: Vec<T>,
        n: usize,
    }

    impl<T> Rmq<T>
    where
        T: Ord + Clone + Max,
    {
        /// Make Range Minimum Query Tree.
        /// This tree is initialized by T::MAX and sustains n elements.
        #[inline]
        pub fn new(n: usize) -> Self {
            let mut n_ = 1;
            while n_ < n {
                n_ *= 2;
            }
            Self {
                tree: vec![T::MAX; n_ * 2 - 1],
                n: n_,
            }
        }

        /// Make Range Minimum Query Tree.
        /// This tree is initialized by init: T and sustains n elements.
        #[inline]
        pub fn init(n: usize, init: T) -> Self {
            let mut n_ = 1;
            while n_ < n {
                n_ *= 2;
            }
            Self {
                tree: vec![init; n_ * 2 - 1],
                n: n_,
            }
        }
    }

    impl<T> Rmq<T>
    where
        T: Ord + Copy,
    {
        /// update Rmq[index] as value.
        /// also update whole of tree.
        #[inline]
        pub fn update(&mut self, index: usize, value: T) {
            if index >= self.n {
                panic!("Rmq::update fail: index is out of bounds");
            }
            let mut k = self.n - 1 + index;
            self[k] = value;
            while k > 0 {
                k = (k - 1) / 2;
                self[k] = std::cmp::min(self[k * 2 + 1], self[k * 2 + 2]);
            }
        }
    }

    impl<T> Rmq<T>
    where
        T: Copy + Max + Ord,
    {
        /// find minimum value in given range.
        /// tree.find(a..b) -> find minimum in [a, b).
        #[inline]
        pub fn find(&self, range: std::ops::Range<usize>) -> T {
            self.find_rec(range, 0, 0, self.n)
        }

        #[inline]
        fn find_rec(&self, range: std::ops::Range<usize>, k: usize, l: usize, r: usize) -> T {
            if r <= range.start || range.end <= l {
                T::MAX
            } else if range.start <= l && r <= range.end {
                self[k]
            } else {
                std::cmp::min(
                    self.find_rec(range.clone(), k * 2 + 1, l, (l + r) / 2),
                    self.find_rec(range, k * 2 + 2, (l + r) / 2, r),
                )
            }
        }
    }

    impl<T> Rmq<T> {
        /// return Rmq's length.
        /// Notice that this is not N that first given.
        #[inline]
        pub fn len(&self) -> usize {
            self.tree.len()
        }

        #[inline]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl<T> std::ops::Index<usize> for Rmq<T> {
        type Output = T;
        #[inline]
        fn index(&self, index: usize) -> &Self::Output {
            &self.tree[index]
        }
    }

    impl<T> std::ops::IndexMut<usize> for Rmq<T> {
        #[inline]
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.tree[index]
        }
    }

    /// Return T::MAX
    pub trait Max
    where
        Self: Copy,
    {
        const MAX: Self;
    }

    impl Max for i32 {
        const MAX: Self = std::i32::MAX;
    }

    impl Max for u32 {
        const MAX: Self = std::u32::MAX;
    }

    impl Max for i64 {
        const MAX: Self = std::i64::MAX;
    }

    impl Max for u64 {
        const MAX: Self = std::u64::MAX;
    }

    impl Max for i128 {
        const MAX: Self = std::i128::MAX;
    }

    impl Max for u128 {
        const MAX: Self = std::u128::MAX;
    }

    impl Max for usize {
        const MAX: Self = std::usize::MAX;
    }

    impl Max for isize {
        const MAX: Self = std::isize::MAX;
    }

    #[cfg(test)]
    mod tests_range_minimum_query {
        use super::Rmq;
        #[test]
        fn for_rmq1() {
            let n = 3;
            let _q = 5;
            let com = [(0, 0, 1), (0, 1, 2), (0, 2, 3), (1, 0, 2), (1, 1, 2)];
            let answers = &[1, 2];
            let mut rmq = Rmq::new(n);
            let mut i = 0;
            for &(d, x, y) in &com {
                match d {
                    0 => rmq.update(x, y as i64),
                    1 => {
                        assert_eq!(rmq.find(x..y + 1), answers[i]);
                        i += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }

        #[test]
        fn for_rmq2() {
            let n = 1;
            let _q = 3;
            let com = [(1, 0, 0), (0, 0, 5), (1, 0, 0)];
            let answers = &[std::i64::MAX, 5];
            let mut rmq = Rmq::new(n);
            let mut i = 0;
            for &(d, x, y) in &com {
                match d {
                    0 => rmq.update(x, y as i64),
                    1 => {
                        assert_eq!(rmq.find(x..y + 1), answers[i]);
                        i += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

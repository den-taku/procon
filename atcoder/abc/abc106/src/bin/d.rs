#![allow(dead_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        lr: [(usize, usize); m],
        pq: [(usize, usize); q]
    }
    let f = |a, b| a + b;
    let inverse = |a: i64| -a;
    let mut tree = vec![bit_library::Bit::new(n, f, inverse, 0i64); n];
    for (l, r) in lr {
        tree[l - 1].add(r - 1, 1)
    }
    let mut ans = vec![];
    for (p, q) in pq {
        let mut sum = 0;
        for i in p..=q {
            sum += tree[i - 1].sum(i - 1..q);
        }
        ans.push(sum.to_string());
    }
    println!("{}", ans.join("\n"))
}

/// BIT for Group (T, F, I)
/// T: Type
/// F: Binary operator
/// I: Inverse
///
/// new
/// sum
/// add
/// len
/// is_empty
/// lower_bound
/// upper_bound
pub mod bit_library {
    /// verified (https://atcoder.jp/contests/arc033/submissions/26370305)
    /// BIT for Group (T, F, I)
    #[derive(Debug, Clone)]
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
        /// O(n)
        #[inline]
        pub fn new(n: usize, f: F, inverse: I, unit: T) -> Self {
            Self {
                tree: vec![unit; n],
                f,
                inverse,
                unit,
            }
        }

        /// O(lg n)
        /// return a_s * a_s+1 * ... * a_t
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

        /// O(lg n)
        /// a_index <- a_index * value
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

    impl<T, F, I> Bit<T, F, I>
    where
        T: Copy + Ord,
        F: Fn(T, T) -> T,
        I: Fn(T) -> T,
    {
        /// Find index that giving least bit_sum[index] s.t. is equal to or more than value.
        #[inline]
        pub fn lower_bound(&self, value: T) -> Option<usize> {
            let mut ub = self.len() - 1;
            let mut lb = 0;
            while ub - lb > 1 {
                let est = (ub + lb) / 2;
                if self.sum(0..est + 1) > value {
                    ub = est
                } else {
                    lb = est
                }
            }
            if self.sum(0..lb + 1) > value {
                Some(lb)
            } else if self.sum(0..ub + 1) > value {
                Some(ub)
            } else {
                None
            }
        }

        /// Find index that giving largest bit_sum[index] s.t. is smaller than value.
        #[inline]
        pub fn upper_bound(&self, value: T) -> Option<usize> {
            let mut ub = self.len() - 1;
            let mut lb = 0;
            while ub - lb > 1 {
                let est = (ub + lb) / 2;
                if self.sum(0..est + 1) <= value {
                    lb = est
                } else {
                    ub = est
                }
            }
            if self.sum(0..ub + 1) < value {
                Some(ub)
            } else if self.sum(0..lb + 1) < value {
                Some(lb)
            } else {
                None
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

    #[cfg(test)]
    mod tests_bit {
        use super::*;

        #[test]
        fn for_bit_sum() {
            // for lower_bound
            let coms = vec![
                vec![(1, 11), (1, 29), (1, 89), (2, 2), (2, 2)],
                vec![
                    (1, 8932),
                    (1, 183450),
                    (1, 34323),
                    (1, 81486),
                    (1, 127874),
                    (1, 114850),
                    (1, 55277),
                    (1, 112706),
                    (2, 3),
                    (1, 39456),
                    (1, 52403),
                    (2, 4),
                ],
            ];
            let answerss = vec![vec![29, 89], vec![55277, 52403]];
            for (com, answers) in coms.into_iter().zip(answerss.into_iter()) {
                let mut i = 0;
                let mut bit = Bit::new(200_000, |a, b| a + b, |e| -e, 0i64);
                for &(direction, value) in &com {
                    match direction {
                        1 => bit.add(value - 1, 1),
                        2 => {
                            let index = bit.lower_bound(value as i64 - 1).unwrap() + 1;
                            bit.add(index - 1, -1);
                            assert_eq!(index, answers[i]);
                            i += 1;
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        #[test]
        fn for_ub() {
            let mut bit = Bit::new(10, |a, b| a + b, |e| -e, 0i32);
            for i in 1..=10 {
                bit.add(i - 1, i as i32);
            }
            assert_eq!(bit.upper_bound(35), Some(6));
            bit.add(5, (bit.inverse)(6));
            assert_eq!(bit.upper_bound(16), Some(5));
        }
    }
}

#![allow(dead_code)]
#![allow(unreachable_code)]
use primes_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
        lr: [(usize, usize); q]
    }
    let mut bit = bit_library::Bit::new(100000, |a, b| a + b, |e| -e, 0);
    for p in SegmentSieve::segment_sieve(2, 50000usize) {
        if is_prime(2 * p - 1) {
            bit.add(2 * p - 1 - 1, 1)
        }
    }
    let mut ans = Vec::new();
    for (l, r) in lr {
        ans.push(bit.sum(l - 1..r).to_string());
    }
    println!("{}", ans.join("\n"))
}

pub mod primes_library {
    /// verified (https://atcoder.jp/contests/arc017/submissions/25846247)
    /// decide whiether n is prime or not
    /// O(n^(1/2))
    #[inline]
    pub fn is_prime<T>(n: T) -> bool
    where
        T: Copy
            + Zero
            + One
            + Two
            + std::cmp::Ord
            + std::ops::AddAssign
            + std::ops::Mul<Output = T>
            + std::ops::Rem<Output = T>,
    {
        let mut i = T::TWO;
        while i * i <= n {
            if n % i == T::ZERO {
                return false;
            }
            i += T::ONE;
        }
        n != T::ONE
    }

    /// List all divisors of n
    #[inline]
    pub fn divisor<T>(n: T) -> Vec<T>
    where
        T: Copy
            + One
            + Zero
            + std::cmp::Ord
            + std::ops::AddAssign
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Rem<Output = T>,
    {
        let mut v = Vec::new();
        let mut i = T::ONE;
        while i * i <= n {
            if n % i == T::ZERO {
                v.push(i);
                if i != n / i {
                    v.push(n / i)
                }
            }
            i += T::ONE;
        }
        v
    }

    /// verified (https://atcoder.jp/contests/abc052/submissions/25846932)
    /// Return map s.t. n = p1^b1 * p2^b2 * ... then factor[pi] = bi.
    /// O(n^(1/2))
    #[inline]
    pub fn prime_factor<T>(mut n: T) -> std::collections::HashMap<T, T>
    where
        T: Copy
            + Zero
            + One
            + Two
            + std::cmp::Ord
            + std::hash::Hash
            + std::ops::AddAssign
            + std::ops::DivAssign
            + std::ops::Mul<Output = T>
            + std::ops::Rem<Output = T>,
    {
        let mut factor = std::collections::HashMap::new();
        let mut i = T::TWO;
        while i * i <= n {
            while n % i == T::ZERO {
                *factor.entry(i).or_insert(T::ZERO) += T::ONE;
                n /= i;
            }
            i += T::ONE;
        }
        if n != T::ONE {
            factor.insert(n, T::ONE);
        }
        factor
    }

    /// Return prime number in [a, b)
    /// verified by this (https://algo-method.com/submissions/69387)
    /// and this (https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5878533#1) (00:03)
    /// O((b - a) * b^(1/2) / lg(b))
    pub trait SegmentSieve
    where
        Self: Sized,
    {
        fn segment_sieve(a: Self, b: Self) -> Vec<Self>;
    }

    macro_rules! impl_segment_sieve {
        ( $($e:ty), *) => {
            $(
                impl SegmentSieve for $e {
                    #[inline]
                    fn segment_sieve(a: Self, b: Self) -> Vec<Self> {
                        let mut is_prime_small = vec![true; (b as f64).sqrt() as usize + 1];
                        let mut is_prime = vec![true; (b - a) as usize];

                        let mut i = 2;
                        while i * i < b {
                            if is_prime_small[i as usize] {
                                let mut j = 2 * i;
                                while j * j < b {
                                    is_prime_small[j as usize] = false;
                                    j += i;
                                }
                                j = std::cmp::max(2, (a + i - 1) / i) * i;
                                while j < b {
                                    is_prime[(j - a) as usize] = false;
                                    j += i;
                                }
                            }
                            i += 1;
                        }

                        is_prime
                            .iter()
                            .enumerate()
                            .filter(|(_, &b)| b)
                            .map(|(i, _)| i as $e + a)
                            .collect()
                    }
                }
            )*
        };
    }

    impl_segment_sieve!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    /// verified by this (https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5878112#1) (00:60)
    /// and this (https://atcoder.jp/contests/tenka1-2012-qualc/submissions/25847422)
    /// O(n * n^(1/2))
    pub struct Seive<T> {
        iter: Box<std::iter::Chain<std::ops::Range<T>, P<T>>>,
    }

    struct P<T> {
        dict: std::collections::HashMap<T, T>,
        iter: Box<std::ops::RangeFrom<T>>,
    }

    impl Default for Seive<i32> {
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    macro_rules! impl_seive {
        ( $($e:ty), *) => {
            $(
                impl Seive<$e> {
                    #[inline]
                    pub fn new() -> Self {
                        Self {
                            iter: Box::new((2..3).chain(P::<$e>::new()))
                        }
                    }
                }

                impl Iterator for Seive<$e> {
                    type Item = $e;
                    #[inline]
                    fn next(&mut self) -> Option<Self::Item> {
                        self.iter.next()
                    }
                }

                impl P<$e> {
                    #[inline]
                    fn new() -> Self {
                        Self {
                            dict: std::collections::HashMap::new(),
                            iter: Box::new((1..))
                        }
                    }
                }

                impl Iterator for P<$e> {
                    type Item = $e;
                    #[inline]
                    fn next(&mut self) -> Option<Self::Item> {
                        while let Some(j) = self.iter.next() {
                            let i = 2 * j + 1;
                            let (factor, is_prime) = match self.dict.remove(&i) {
                                Some(f) => (f, false),
                                None => (i * 2, true),
                            };
                            let key = (1..)
                                .filter_map(|j| {
                                    let k = i + j * factor;
                                    if self.dict.contains_key(&k) {
                                        None
                                    } else {
                                        Some(k)
                                    }
                                })
                                .next()
                                .unwrap();
                            self.dict.insert(key, factor);
                            if is_prime {
                                return Some(i);
                            }
                        }
                    None
                    }
                }
            )*
        };
    }

    impl_seive!(isize, i32, i64, i128, usize, u32, u64, u128);

    pub trait Zero {
        const ZERO: Self;
    }

    macro_rules! impl_zero {
        ( $($e:ty),* ) => {
            $(
                impl Zero for $e {
                    const ZERO: Self = 0;
                }
            )*
        };
    }

    impl_zero!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    pub trait One {
        const ONE: Self;
    }

    macro_rules! impl_one {
        ( $($e:ty),* ) => {
            $(
                impl One for $e {
                    const ONE: Self = 1;
                }
            )*
        };
    }

    impl_one!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    pub trait Two {
        const TWO: Self;
    }

    macro_rules! impl_two {
        ( $($e:ty),* ) => {
            $(
                impl Two for $e {
                    const TWO: Self = 2;
                }
            )*
        };
    }

    impl_two!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    #[cfg(test)]
    mod tests_primes {
        use super::*;

        #[test]
        fn for_is_prime() {
            assert_eq!(is_prime(64), false);
            assert_eq!(is_prime(1), false);
            assert_eq!(is_prime(57), false);
            assert_eq!(is_prime(541), true);
            assert_eq!(is_prime(2), true);
        }

        #[test]
        fn for_divisor() {
            assert_eq!(
                divisor(12).iter().collect::<std::collections::HashSet<_>>(),
                vec![1, 2, 3, 4, 6, 12]
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
            );
            assert_eq!(divisor(12), vec![1, 12, 2, 6, 3, 4]);
            assert_eq!(divisor(1), vec![1]);
            assert_eq!(divisor(7), vec![1, 7]);
        }

        #[test]
        fn for_prime_factor() {
            assert_eq!(
                prime_factor(12),
                vec![(2, 2), (3, 1)]
                    .into_iter()
                    .collect::<std::collections::HashMap<_, _>>()
            );
            assert_eq!(
                prime_factor(57),
                vec![(3, 1), (19, 1)]
                    .into_iter()
                    .collect::<std::collections::HashMap<_, _>>()
            );
            assert_eq!(
                prime_factor(3),
                vec![(3, 1)]
                    .into_iter()
                    .collect::<std::collections::HashMap<_, _>>()
            );
        }

        #[test]
        fn for_seive() {
            assert_eq!(Seive::<isize>::new().take(100).last(), Some(541));
            assert_eq!(Seive::<usize>::new().take(100).last(), Some(541));
            assert_eq!(Seive::<i32>::new().take(100).last(), Some(541));
            assert_eq!(Seive::<u32>::new().take(100).last(), Some(541));
            assert_eq!(Seive::<i64>::new().take(100).last(), Some(541));
            assert_eq!(Seive::<u64>::new().take(100).last(), Some(541));
            assert_eq!(Seive::<i128>::new().take(100).last(), Some(541));
            assert_eq!(Seive::<u128>::new().take(100).last(), Some(541));

            assert_eq!(Seive::default().take(10_000).last(), Some(104_729));

            assert_eq!(
                Seive::default().take_while(|&e| e < 200_000).last(),
                Some(199_999)
            );

            assert_eq!(
                Seive::default().take_while(|&e| e < 20).collect::<Vec<_>>(),
                vec![2, 3, 5, 7, 11, 13, 17, 19]
            )
        }

        #[test]
        fn for_segment_seive() {
            assert_eq!(
                SegmentSieve::segment_sieve(2u64, 14),
                vec![2, 3, 5, 7, 11, 13]
            )
        }
    }
}

pub mod bit_library {
    /// verified (https://atcoder.jp/contests/arc033/submissions/26370305)
    /// BIT for Group (T, F, I)
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

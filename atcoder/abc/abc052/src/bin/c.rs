#![allow(unreachable_code)]
#![allow(dead_code)]
use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut factors = std::collections::HashMap::new();
    for i in 1..n + 1 {
        let f = primes_library::prime_factor(i);
        for (k, v) in f {
            *factors.entry(k).or_insert(0) += v;
        }
    }
    println!(
        "{}",
        factors
            .iter()
            .map(|(_, &v)| v)
            .fold(1, |p, m| { p * (m + 1) % MOD })
    )
}

pub mod primes_library {
    // verified (https://atcoder.jp/contests/arc017/submissions/25846247)
    pub fn is_prime<T>(n: T) -> bool
    where
        T: Two
            + Copy
            + One
            + Zero
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

    pub fn prime_factor<T>(mut n: T) -> std::collections::HashMap<T, T>
    where
        T: Copy
            + One
            + Two
            + Zero
            + std::cmp::Ord
            + std::hash::Hash
            + std::ops::AddAssign
            + std::ops::Mul<Output = T>
            + std::ops::DivAssign
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

    // verified (https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5878112#1)
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
    }
}

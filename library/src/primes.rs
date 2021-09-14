#![allow(dead_code)]

pub mod primes_library {
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

    macro_rules! impl_num {
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

    impl_num!(isize, i32, i64, i128, usize, u32, u64, u128);

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
        fn for_primes() {
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

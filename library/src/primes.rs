#![allow(dead_code)]

pub mod primes_library {
    macro_rules! impl_num {
        ( $($e:ty),* ) => {
            $(
                impl Prime<$e> {
                    impl_new!($e);
                }

                impl Iterator for Prime<$e> {
                    impl_next!($e);
                }
            )*
        };
    }

    macro_rules! impl_new {
        ($e:ty) => {
            #[inline]
            pub fn new() -> Self {
                Self {
                    iterator: Box::new(2..),
                    primes: Vec::new(),
                }
            }
        };
    }

    macro_rules! impl_next {
        ($e:ty) => {
            type Item = $e;
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                while let Some(e) = self.iterator.next() {
                    if self.primes.iter().all(|&p| e % p != 0) {
                        self.primes.push(e);
                        return Some(e);
                    }
                }
                None
            }
        };
    }

    pub struct Prime<T> {
        iterator: Box<dyn Iterator<Item = T>>,
        primes: Vec<T>,
    }

    impl Default for Prime<i32> {
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    impl_num!(isize, i32, i64, i128, usize, u32, u64, u128);

    #[cfg(test)]
    mod tests_primes {
        use super::*;

        #[test]
        fn for_primes() {
            assert_eq!(Prime::<isize>::new().take(100).last(), Some(541));
            assert_eq!(Prime::<usize>::new().take(100).last(), Some(541));
            assert_eq!(Prime::<i32>::new().take(100).last(), Some(541));
            assert_eq!(Prime::<u32>::new().take(100).last(), Some(541));
            assert_eq!(Prime::<i64>::new().take(100).last(), Some(541));
            assert_eq!(Prime::<u64>::new().take(100).last(), Some(541));
            assert_eq!(Prime::<i128>::new().take(100).last(), Some(541));
            assert_eq!(Prime::<u128>::new().take(100).last(), Some(541));

            assert_eq!(Prime::default().take(10_000).last(), Some(104_729));

            assert_eq!(
                Prime::default().take_while(|&e| e < 200_000).last(),
                Some(199_999)
            );

            assert_eq!(
                Prime::default().take_while(|&e| e < 20).collect::<Vec<_>>(),
                vec![2, 3, 5, 7, 11, 13, 17, 19]
            )
        }
    }
}

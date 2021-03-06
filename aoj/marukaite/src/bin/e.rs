use primes_library::*;

fn main() {
    loop {
        let input = read_line::<usize>();
        if input.is_empty() {
            break;
        } else {
            let mut iter = Prime::<usize>::new();
            println!(
                "{} {}",
                iter.by_ref().take_while(|&e| e < input[0]).last().unwrap(),
                if let Some(e) = iter.next() {
                    if e == input[0] {
                        iter.next().unwrap()
                    } else {
                        e
                    }
                } else {
                    0
                }
            )
        }
    }
}

fn read_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

pub mod primes_library {
    macro_rules! impl_new {
        ($e:ty) => {
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

    impl Prime<isize> {
        impl_new!(isize);
    }

    impl Iterator for Prime<isize> {
        impl_next!(isize);
    }

    impl Prime<i32> {
        impl_new!(i32);
    }

    impl Iterator for Prime<i32> {
        impl_next!(i32);
    }

    impl Prime<i64> {
        impl_new!(i64);
    }

    impl Iterator for Prime<i64> {
        impl_next!(i64);
    }

    impl Prime<i128> {
        impl_new!(i128);
    }

    impl Iterator for Prime<i128> {
        impl_next!(i128);
    }

    impl Prime<usize> {
        impl_new!(usize);
    }

    impl Iterator for Prime<usize> {
        impl_next!(usize);
    }

    impl Prime<u32> {
        impl_new!(u32);
    }

    impl Iterator for Prime<u32> {
        impl_next!(u32);
    }

    impl Prime<u64> {
        impl_new!(u64);
    }

    impl Iterator for Prime<u64> {
        impl_next!(u64);
    }

    impl Prime<u128> {
        impl_new!(u128);
    }

    impl Iterator for Prime<u128> {
        impl_next!(u128);
    }

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

            assert_eq!(Prime::<i32>::new().take(10_000).last(), Some(104_729));
        }
    }
}

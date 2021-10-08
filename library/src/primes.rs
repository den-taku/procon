#![allow(dead_code)]

pub mod primes_library {
    

    

    

    

    

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

    
    #[cfg(test)]
    mod tests_primes {
        use super::*;

    }
}

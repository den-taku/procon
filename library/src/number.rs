#[allow(dead_code)]

pub mod number_library {
    /// O(lg n)
    #[inline]
    pub fn gcd<T>(a: T, b: T) -> T
    where
        T: std::ops::Rem<Output = T> + Zero + std::cmp::Eq + Copy + std::cmp::Ord,
    {
        let mut max = std::cmp::max(a, b);
        let mut min = std::cmp::min(a, b);
        while min != T::ZERO {
            let tmp = min;
            min = max % min;
            max = tmp;
        }
        max
    }

    /// O(lg n)
    #[inline]
    pub fn lcm<T>(a: T, b: T) -> T
    where
        T: Copy
            + std::ops::Rem<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::cmp::Ord
            + Zero
            + std::cmp::Eq,
    {
        a / gcd(a, b) * b
    }

    /// return gcd(a, b), x, y s.t. ax + by = gcd(a, b)
    /// verified (https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5877610#1)
    /// O(lg n)
    #[inline]
    pub fn ext_gcd<T>(a: T, b: T) -> (T, T, T)
    where
        T: std::ops::Rem<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Sub<Output = T>
            + Zero
            + One
            + std::cmp::Eq
            + Copy,
    {
        if b == T::ZERO {
            (a, T::ONE, T::ZERO)
        } else {
            let (d, y_b, x_b) = ext_gcd(b, a % b);
            (d, x_b, y_b - (a / b) * x_b)
        }
    }

    /// O(lg n)
    #[inline]
    pub fn mod_inverse<T>(a: T, m: T) -> Option<T>
    where
        T: std::ops::Rem<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Sub<Output = T>
            + Zero
            + One
            + std::cmp::Eq
            + Copy,
    {
        let (d, x, _) = ext_gcd(a, m);
        if d == T::ONE {
            Some(x)
        } else {
            None
        }
    }

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
    mod tests_number {
        use super::*;

        #[test]
        fn for_gcd() {
            assert_eq!(gcd(9, 12), 3);
        }

        #[test]
        fn for_lcm() {
            assert_eq!(lcm(9, 12), 36);
        }

        #[test]
        fn for_ext_gcd() {
            assert_eq!(ext_gcd(4, 12), (4, 1, 0));
            assert_eq!(ext_gcd(3, 8), (1, 3, -1));
        }

        #[test]
        fn for_mod_inverse() {
            assert_eq!(mod_inverse(4, 11), Some(3));
            assert_eq!(mod_inverse(7, 13), Some(2));
        }
    }
}

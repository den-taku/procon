#![allow(dead_code)]

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

    /// return inverse element a^-1 s.t. a * a^-1 ≡ 1 (mod m)
    /// verified (https://atcoder.jp/contests/abc024/submissions/26198316)
    /// O(lg n)
    #[inline]
    pub fn mod_inverse<T>(a: T, m: T) -> Option<T>
    where
        T: std::ops::Rem<Output = T>
            + std::ops::Add<Output = T>
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
            Some((m + x % m) % m)
        } else {
            None
        }
    }

    /// solve simultaneous linear congruent
    /// give ai x ≡ bi (mod mi) (1 <= i <= n) as Vec<(ai, bi, mi)>,
    /// then return Option<(b, m)> s.t. x ≡ b (mod m)
    #[inline]
    pub fn solve_simultaneous_linear_congruent<T>(variables: &[(T, T, T)]) -> Option<(T, T)>
    where
        T: Zero
            + One
            + Copy
            + std::cmp::Eq
            + std::cmp::Ord
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::MulAssign
            + std::ops::Div<Output = T>
            + std::ops::Rem<Output = T>,
    {
        // ∀x, x ≡ 0 (mod 1)
        let mut x = T::ZERO;
        let mut m1 = T::ONE;
        for &(a, b, m2) in variables {
            let am1 = a * m1;
            let b2_ab1 = {
                let tmp = (b - a * x) % m2;
                if tmp < T::ZERO {
                    tmp + m2
                } else {
                    tmp
                }
            };
            let d = gcd(am1, m2);
            if b2_ab1 % d != T::ZERO {
                // no answer exists.
                return None;
            }
            let t = b2_ab1 / d * mod_inverse(am1 / d, m2 / d).unwrap() % (m2 / d);
            x = x + m1 * t;
            m1 *= m2 / d;
        }
        Some((x % m1, m1))
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

        #[test]
        fn for_solve_simultaneous() {
            let problems = vec![
                vec![(1, 10, 20), (1, 10, 30), (1, 10, 40)],
                vec![(1, 10, 20), (1, 10, 30), (1, 30, 40)],
                vec![(1, 80712, 221549), (1, 320302, 699312), (1, 140367, 496729)],
            ];
            let answers = vec![10, 70, 38774484298448350i64];
            for (p, a) in problems.iter().zip(answers) {
                assert_eq!(solve_simultaneous_linear_congruent(p).unwrap().0, a);
            }
        }
    }
}

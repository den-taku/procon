#![allow(dead_code)]
#![allow(unreachable_code)]
use number_library::*;
use proconio::{fastout, input};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize
    }
    let mut seeds = vec![0; k];
    for i in 0..k {
        seeds[i] = k * gcd(i, k);
    }
    let mut sum = 0;
    let c = n / k;
    let r = n % k;
    for i in 0..k {
        let coeff = ((c - 1) * i + k * (c - 1) * c / 2) % MOD;
        sum += coeff * seeds[i];
        sum %= MOD;
    }
    for i in 1..=r {
        let coeff = (c * k + i) % MOD;
        sum += coeff * seeds[i];
        sum %= MOD;
    }
    println!("{}", sum);
}

fn compress(a: &[usize]) -> std::collections::HashMap<usize, usize> {
    let mut v = Vec::new();
    let mut map = std::collections::HashMap::new();
    for &e in a.iter() {
        v.push(e);
    }
    v.sort();
    v.dedup();
    for &e in a.iter() {
        if map.get(&e).is_none() {
            let mut ub = v.len();
            let mut lb = 0;
            while ub - lb > 1 {
                let est = (ub + lb) / 2;
                if v[est] > e {
                    ub = est
                } else {
                    lb = est
                }
            }
            if v[lb] == e {
                map.insert(e, lb);
            } else {
                map.insert(e, ub);
            }
        }
    }
    map
}

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

    /// calculate nCr
    /// verified (https://atcoder.jp/contests/arc077/submissions/26354506)
    pub struct BinomialCoefficient {
        maximum: usize,
        modular: usize,
        factorial: Vec<i128>,
        finverse: Vec<i128>,
    }

    impl BinomialCoefficient {
        pub fn new(maximum: usize, modular: usize) -> Self {
            let mut factorial = vec![1i128; maximum + 1];
            let mut finverse = vec![0; maximum + 1];
            finverse[0] = mod_inverse(1, modular as i128).unwrap();
            finverse[1] = finverse[0];
            for i in 2..=maximum {
                factorial[i] = factorial[i - 1] * i as i128 % modular as i128;
                finverse[i] = finverse[i - 1] * mod_inverse(i as i128, modular as i128).unwrap()
                    % modular as i128;
            }
            Self {
                maximum,
                modular,
                factorial,
                finverse,
            }
        }

        /// return nCr
        pub fn comp(&self, n: usize, r: usize) -> i128 {
            if n > self.maximum {
                panic!("out of range: nCr")
            }
            if n < r {
                0
            } else {
                self.factorial[n] * (self.finverse[r] * self.finverse[n - r] % self.modular as i128)
                    % self.modular as i128
            }
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

        #[test]
        fn for_binomial_coefficient() {
            // This is https://atcoder.jp/contests/arc077/tasks/arc077_b.
            let modular = 1_000_000_007;
            let n = 32;
            let a = vec![
                29, 19, 7, 10, 26, 32, 27, 4, 11, 20, 2, 8, 16, 23, 5, 14, 6, 12, 17, 22, 18, 30,
                28, 24, 15, 1, 25, 3, 13, 21, 19, 31, 9,
            ];
            let mut double = vec![false; n + 1];
            let mut value = 0;
            let mut b = n;
            for (i, &e) in a.iter().enumerate() {
                if double[e - 1] {
                    value = e;
                    b -= i;
                    break;
                }
                double[e - 1] = true;
            }
            let f = a.iter().position(|&e| e == value).unwrap();
            let n_c_r = BinomialCoefficient::new(n + 1, modular);
            let mut ans = Vec::new();
            ans.push(n as i128);
            for k in 2..=n {
                // n+1Ck
                let all = n_c_r.comp(n + 1, k);
                // f+bCk-1
                let sub = n_c_r.comp(f + b, k - 1);
                let v = if all > sub {
                    all - sub
                } else {
                    modular as i128 + all - sub
                };
                ans.push(v)
            }
            ans.push(1);
            let answers = vec![
                32, 525, 5453, 40919, 237336, 1107568, 4272048, 13884156, 38567100, 92561040,
                193536720, 354817320, 573166440, 818809200, 37158313, 166803103, 166803103,
                37158313, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100, 13884156,
                4272048, 1107568, 237336, 40920, 5456, 528, 33, 1,
            ];
            assert_eq!(ans, answers);
        }
    }
}

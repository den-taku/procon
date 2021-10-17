#![allow(dead_code)]
#![allow(unreachable_code)]
use number_library::*;
use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        m: usize,
        edges: [(usize, usize); m]
    }
    let mut tree = tree_dist_library::TreeDist::new(n);
    for (u, v) in edges {
        tree.add_edge(u - 1, v - 1);
        tree.add_edge(v - 1, u - 1);
    }
    println!(
        "{}",
        tree.count_path(s - 1, ModInteger::new(0, MOD), ModInteger::new(1, MOD))
            .0[t - 1]
    )
}

/// for numbers
///
/// gcd
/// lcm
/// ext_gcd
/// mod_inverse
/// solve_simultaneous_linear_congruent
/// BinomialCoefficient:new, comp
/// is_prime
/// divisor
/// prime_factor
/// segment_seive
/// Seive (bad implmentation)
/// ModInteger
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
        modular: u64,
        factorial: Vec<ModInteger>,
        finverse: Vec<ModInteger>,
    }

    impl BinomialCoefficient {
        pub fn new(maximum: usize, modular: u64) -> Self {
            let mut factorial = vec![ModInteger::new(1, modular); maximum + 1];
            let mut finverse = vec![ModInteger::new(0, modular); maximum + 1];
            finverse[0] = ModInteger::inverse_from(1, modular);
            finverse[1] = finverse[0];
            for i in 2..=maximum {
                factorial[i] = factorial[i - 1] * i as u64;
                finverse[i] = finverse[i - 1] * ModInteger::inverse_from(i as u64, modular);
            }
            Self {
                maximum,
                modular,
                factorial,
                finverse,
            }
        }

        /// return nCr
        pub fn comp(&self, n: usize, r: usize) -> ModInteger {
            if n > self.maximum {
                panic!("out of range: nCr")
            }
            if n < r {
                ModInteger::new(0, self.modular)
            } else {
                self.factorial[n] * self.finverse[r] * self.finverse[n - r]
            }
        }
    }

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

    /// virifid (https://atcoder.jp/contests/abc151/submissions/26416209)
    #[derive(Copy, Clone, Debug)]
    pub struct ModInteger {
        value: u64,
        modular: u64,
    }

    impl ModInteger {
        pub fn new(value: u64, modular: u64) -> Self {
            Self {
                value: value % modular,
                modular,
            }
        }

        pub fn value(&self) -> u64 {
            self.value
        }

        pub fn powu(&self, n: usize) -> Self {
            let mut n = n;
            let mut value = self.value;
            let mut ret = 1u64;
            while n > 0 {
                if n % 2 == 1 {
                    ret = (ret * value) % self.modular;
                }
                value = (value * value) % self.modular;
                n /= 2;
            }
            Self {
                value,
                modular: self.modular,
            }
        }

        pub fn pow(&self, n: Self) -> Self {
            let n = n.value() as usize;
            self.powu(n)
        }

        pub fn inverse(&self) -> Self {
            let value = mod_inverse(self.value as i64, self.modular as i64).unwrap();
            Self {
                value: (value + self.modular as i64) as u64 % self.modular,
                modular: self.modular,
            }
        }

        pub fn inverse_from(value: u64, modular: u64) -> Self {
            let value = (mod_inverse(value as i64, modular as i64).unwrap() + modular as i64)
                as u64
                % modular;
            Self { value, modular }
        }
    }

    impl std::ops::Add<Self> for ModInteger {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                value: (self.value + rhs.value) % self.modular,
                modular: self.modular,
            }
        }
    }

    impl std::ops::Add<u64> for ModInteger {
        type Output = Self;
        fn add(self, rhs: u64) -> Self::Output {
            Self {
                value: (self.value + rhs) % self.modular,
                modular: self.modular,
            }
        }
    }

    impl std::ops::AddAssign<Self> for ModInteger {
        fn add_assign(&mut self, rhs: Self) {
            self.value += rhs.value;
            self.value %= self.modular;
        }
    }

    impl std::ops::AddAssign<u64> for ModInteger {
        fn add_assign(&mut self, rhs: u64) {
            self.value += rhs;
            self.value %= self.modular;
        }
    }

    impl std::fmt::Display for ModInteger {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl std::ops::Sub<Self> for ModInteger {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                value: (self.value + self.modular - rhs.value) % self.modular,
                modular: self.modular,
            }
        }
    }

    impl std::ops::Sub<u64> for ModInteger {
        type Output = Self;
        fn sub(self, rhs: u64) -> Self::Output {
            Self {
                value: (self.value + self.modular - rhs) % self.modular,
                modular: self.modular,
            }
        }
    }

    impl std::ops::SubAssign<Self> for ModInteger {
        fn sub_assign(&mut self, rhs: Self) {
            self.value += self.modular;
            self.value -= rhs.value;
            self.value %= self.modular;
        }
    }

    impl std::ops::SubAssign<u64> for ModInteger {
        fn sub_assign(&mut self, rhs: u64) {
            self.value += self.modular;
            self.value -= rhs % self.modular;
            self.value %= self.modular;
        }
    }

    impl std::ops::Mul<Self> for ModInteger {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self {
                value: (self.value * rhs.value) % self.modular,
                modular: self.modular,
            }
        }
    }

    impl std::ops::Mul<u64> for ModInteger {
        type Output = Self;
        fn mul(self, rhs: u64) -> Self::Output {
            Self {
                value: rhs % self.modular * self.value % self.modular,
                modular: self.modular,
            }
        }
    }

    impl std::ops::MulAssign<Self> for ModInteger {
        fn mul_assign(&mut self, rhs: Self) {
            self.value *= rhs.value;
            self.value %= self.modular;
        }
    }

    impl std::ops::MulAssign<u64> for ModInteger {
        fn mul_assign(&mut self, rhs: u64) {
            self.value *= rhs % self.modular;
            self.value %= self.modular;
        }
    }

    impl std::ops::Div<Self> for ModInteger {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            Self {
                value: (self.value * rhs.inverse().value) % self.modular,
                modular: self.modular,
            }
        }
    }

    impl std::ops::Div<u64> for ModInteger {
        type Output = Self;
        fn div(self, rhs: u64) -> Self::Output {
            let i = (mod_inverse((rhs % self.modular) as i64, self.modular as i64).unwrap()
                + self.modular as i64) as u64
                % self.modular;
            Self {
                value: i * self.value % self.modular,
                modular: self.modular,
            }
        }
    }

    impl std::ops::DivAssign<Self> for ModInteger {
        fn div_assign(&mut self, rhs: Self) {
            self.value *= rhs.inverse().value;
            self.value %= self.modular;
        }
    }

    impl std::ops::DivAssign<u64> for ModInteger {
        fn div_assign(&mut self, rhs: u64) {
            let i = (mod_inverse((rhs % self.modular) as i64, self.modular as i64).unwrap()
                + self.modular as i64) as u64
                % self.modular;
            self.value *= i;
            self.value %= self.modular;
        }
    }

    impl std::cmp::PartialEq for ModInteger {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl std::cmp::Eq for ModInteger {}

    impl std::cmp::PartialOrd for ModInteger {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::cmp::Ord for ModInteger {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.value.cmp(&other.value)
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
    mod tests_number {
        use super::*;
        const MOD: u64 = 1_000_000_007;

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
            let n_c_r = BinomialCoefficient::new(n + 1, MOD);
            let mut ans = Vec::new();
            ans.push(ModInteger::new(n as u64, MOD));
            for k in 2..=n {
                // n+1Ck
                let all = n_c_r.comp(n + 1, k);
                // f+bCk-1
                let sub = n_c_r.comp(f + b, k - 1);
                let v = all - sub;
                ans.push(v)
            }
            ans.push(ModInteger::new(1, MOD));
            let answers = vec![
                32, 525, 5453, 40919, 237336, 1107568, 4272048, 13884156, 38567100, 92561040,
                193536720, 354817320, 573166440, 818809200, 37158313, 166803103, 166803103,
                37158313, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100, 13884156,
                4272048, 1107568, 237336, 40920, 5456, 528, 33, 1,
            ];
            assert_eq!(
                ans,
                answers
                    .into_iter()
                    .map(|e| ModInteger::new(e, MOD))
                    .collect::<Vec<_>>()
            );
        }

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

/// Manage Tree
///
/// new
/// add_edge
/// remove_edge
/// distance
/// count_distance_for_circuit
/// count_subtree_from
/// eulerian_walk
/// strongly_connected_components
pub mod tree_dist_library {
    /// verified by this(https://atcoder.jp/contests/abc218/submissions/25794641)
    pub struct TreeDist {
        nodes: usize,
        pub adjacent: Vec<Vec<usize>>,
    }

    pub struct TreeDistStrongly {
        nodes: usize,
        pub adjacent: Vec<Vec<usize>>,
        pub rev: Vec<Vec<usize>>,
    }

    impl TreeDist {
        pub fn count_path<T>(&self, start: usize, zero: T, one: T) -> (Vec<T>, Vec<Option<usize>>)
        where
            T: Copy + std::ops::AddAssign,
        {
            let mut distance = vec![None; self.nodes];
            distance[start] = Some(0);
            let mut count = vec![zero; self.nodes];
            count[start] += one;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(u) = queue.pop_front() {
                for &v in &self.adjacent[u] {
                    if distance[v].is_none() {
                        distance[v] = Some(distance[u].unwrap() + 1);
                        count[v] = count[u];
                        queue.push_back(v);
                    } else if let Some(d) = distance[v] {
                        let dis = distance[u].unwrap();
                        if d == dis + 1 {
                            let tmp = count[u];
                            count[v] += tmp;
                        }
                    }
                }
            }
            (count, distance)
        }
    }

    impl TreeDist {
        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                adjacent: vec![Vec::new(); nodes],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u].push(v);
        }

        pub fn remove_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u] = self.adjacent[u]
                .iter()
                .copied()
                .filter(|&e| e != v)
                .collect();
        }

        pub fn distance(&self, start: usize) -> (Vec<Option<usize>>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            dist[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        queue.push_back(e);
                        pre[e] = s;
                    }
                }
            }
            (dist, pre)
        }

        pub fn count_distance_for_circuit(
            &self,
            start: usize,
            terminal: usize,
        ) -> (Vec<Option<usize>>, Vec<usize>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            let mut count = vec![0; self.nodes];
            dist[start] = Some(0);
            count[start] = 1;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        pre[e] = s;
                        count[e] = count[s];
                        if e != terminal {
                            queue.push_back(e);
                        }
                    } else if let Some(d) = dist[e] {
                        if d == dist[s].unwrap() + 1 {
                            count[e] += count[s];
                            if e != terminal {
                                queue.push_back(e);
                            }
                        }
                    }
                }
            }
            (dist, pre, count)
        }

        /// virifid (https://atcoder.jp/contests/abc220/submissions/26208107)
        /// count subtree's size
        pub fn count_subtree_from(&self, start: usize) -> Vec<usize> {
            let mut count = vec![0; self.nodes];
            self.rec_subtree(self.nodes, start, &mut count);
            count
        }

        fn rec_subtree(&self, from: usize, now: usize, count: &mut [usize]) -> usize {
            let mut sum = 1;
            for &v in &self.adjacent[now] {
                if v == from {
                    continue;
                }
                sum += self.rec_subtree(now, v, count);
            }
            count[now] = sum;
            sum
        }

        pub fn eulerian_walk(&self) -> std::collections::LinkedList<(usize, usize)> {
            let mut edges = vec![std::collections::HashSet::new(); self.nodes];
            for (i, v) in self.adjacent.iter().enumerate() {
                for &e in v {
                    edges[i].insert(e);
                }
            }
            self.eulerian_walk_rec(0, &mut edges)
        }

        fn eulerian_walk_rec(
            &self,
            start: usize,
            edges: &mut [std::collections::HashSet<usize>],
        ) -> std::collections::LinkedList<(usize, usize)> {
            let mut path = vec![start];
            let mut v = start;
            while !edges[v].is_empty() {
                let &w = edges[v].iter().last().unwrap();
                edges[v].remove(&w);
                path.push(w);
                v = w;
            }
            let mut walk = std::collections::LinkedList::new();
            if path.len() >= 2 {
                walk.push_back((path[0], path[1]))
            }
            for (&v, &w) in path.iter().zip(path.iter().skip(1)).skip(1) {
                walk.append(&mut self.eulerian_walk_rec(v, edges));
                walk.push_back((v, w));
            }
            walk
        }
    }

    impl TreeDistStrongly {
        /// return strongly connected component id.
        /// if graph has moderate topological order, then return unique id
        pub fn strongly_connected_component(&self) -> Vec<usize> {
            let mut visited = vec![false; self.nodes];
            let mut order = vec![0; self.nodes];
            let mut inverse = vec![0; self.nodes];
            let mut n = 0;
            for v in 0..self.nodes {
                if !visited[v] {
                    self.visit1(v, &mut n, &mut order, &mut inverse, &mut visited);
                }
            }
            let mut visited = vec![false; self.nodes];
            let mut comp = vec![0; self.nodes];
            let mut k = 0;
            for i in (0..self.nodes).rev() {
                if !visited[inverse[i]] {
                    self.visit2(inverse[i], k, &mut comp, &mut visited);
                    k += 1;
                }
            }
            comp
        }

        pub fn eulerian_walk(&self) -> std::collections::LinkedList<(usize, usize)> {
            let mut edges = vec![std::collections::HashSet::new(); self.nodes];
            for (i, v) in self.adjacent.iter().enumerate() {
                for &e in v {
                    edges[i].insert(e);
                }
            }
            self.eulerian_walk_rec(0, &mut edges)
        }

        fn eulerian_walk_rec(
            &self,
            start: usize,
            edges: &mut [std::collections::HashSet<usize>],
        ) -> std::collections::LinkedList<(usize, usize)> {
            let mut path = vec![start];
            let mut v = start;
            while !edges[v].is_empty() {
                let &w = edges[v].iter().last().unwrap();
                edges[v].remove(&w);
                path.push(w);
                v = w;
            }
            let mut walk = std::collections::LinkedList::new();
            if path.len() >= 2 {
                walk.push_back((path[0], path[1]))
            }
            for (&v, &w) in path.iter().zip(path.iter().skip(1)).skip(1) {
                walk.append(&mut self.eulerian_walk_rec(v, edges));
                walk.push_back((v, w));
            }
            walk
        }

        fn visit1(
            &self,
            v: usize,
            n: &mut usize,
            order: &mut [usize],
            inverse: &mut [usize],
            visited: &mut [bool],
        ) {
            visited[v] = true;
            for &w in &self.adjacent[v] {
                if !visited[w] {
                    self.visit1(w, n, order, inverse, visited);
                }
            }
            order[v] = *n;
            inverse[*n] = v;
            *n += 1;
        }

        fn visit2(&self, w: usize, k: usize, comp: &mut [usize], visited: &mut [bool]) {
            visited[w] = true;
            for &v in &self.rev[w] {
                if !visited[v] {
                    self.visit2(v, k, comp, visited);
                }
            }
            comp[w] = k;
        }

        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                adjacent: vec![Vec::new(); nodes],
                rev: vec![Vec::new(); nodes],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u].push(v);
            self.rev[v].push(u);
        }

        pub fn remove_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u] = self.adjacent[u]
                .iter()
                .copied()
                .filter(|&e| e != v)
                .collect();
            self.rev[v] = self.rev[v].iter().copied().filter(|&e| e != u).collect();
        }

        pub fn distance(&self, start: usize) -> (Vec<Option<usize>>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            dist[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        queue.push_back(e);
                        pre[e] = s;
                    }
                }
            }
            (dist, pre)
        }

        pub fn count_distance_for_circuit(
            &self,
            start: usize,
            terminal: usize,
        ) -> (Vec<Option<usize>>, Vec<usize>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            let mut count = vec![0; self.nodes];
            dist[start] = Some(0);
            count[start] = 1;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        pre[e] = s;
                        count[e] = count[s];
                        if e != terminal {
                            queue.push_back(e);
                        }
                    } else if let Some(d) = dist[e] {
                        if d == dist[s].unwrap() + 1 {
                            count[e] += count[s];
                            if e != terminal {
                                queue.push_back(e);
                            }
                        }
                    }
                }
            }
            (dist, pre, count)
        }

        /// virifid (https://atcoder.jp/contests/abc220/submissions/26208107)
        /// count subtree's size
        pub fn count_subtree_from(&self, start: usize) -> Vec<usize> {
            let mut count = vec![0; self.nodes];
            self.rec_subtree(self.nodes, start, &mut count);
            count
        }

        fn rec_subtree(&self, from: usize, now: usize, count: &mut [usize]) -> usize {
            let mut sum = 1;
            for &v in &self.adjacent[now] {
                if v == from {
                    continue;
                }
                sum += self.rec_subtree(now, v, count);
            }
            count[now] = sum;
            sum
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
    mod tests_tree_dist {
        use super::*;

        #[test]
        fn for_count() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDist::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0],
                    vec![1, 1, 2, 1, 1]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDist::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), None, None],
                    vec![0, 0, 2, 3],
                    vec![1, 1, 0, 0]
                )
            )
        }

        #[test]
        fn for_dist() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDist::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDist::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (vec![Some(0), Some(1), None, None], vec![0, 0, 2, 3])
            )
        }

        #[test]
        fn for_count_subtree() {
            let nodes = 6;
            let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
            let mut tree = TreeDist::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(tree.count_subtree_from(0), vec![6, 1, 1, 1, 1, 1]);

            let nodes = 12;
            let edges = vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 5),
                (2, 6),
                (3, 7),
                (4, 8),
                (7, 9),
                (7, 10),
                (8, 11),
                (8, 12),
            ];
            let mut tree = TreeDist::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(
                tree.count_subtree_from(0),
                vec![12, 3, 4, 4, 1, 1, 3, 3, 1, 1, 1, 1]
            );
            assert_eq!(
                tree.count_subtree_from(6),
                vec![8, 3, 9, 4, 1, 1, 12, 3, 1, 1, 1, 1]
            );
        }

        #[test]
        fn for_count_strongly() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0],
                    vec![1, 1, 2, 1, 1]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), None, None],
                    vec![0, 0, 2, 3],
                    vec![1, 1, 0, 0]
                )
            )
        }

        #[test]
        fn for_dist_strongly() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (vec![Some(0), Some(1), None, None], vec![0, 0, 2, 3])
            )
        }

        #[test]
        fn for_count_subtree_strongly() {
            let nodes = 6;
            let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
            let mut tree = TreeDistStrongly::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(tree.count_subtree_from(0), vec![6, 1, 1, 1, 1, 1]);

            let nodes = 12;
            let edges = vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 5),
                (2, 6),
                (3, 7),
                (4, 8),
                (7, 9),
                (7, 10),
                (8, 11),
                (8, 12),
            ];
            let mut tree = TreeDistStrongly::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(
                tree.count_subtree_from(0),
                vec![12, 3, 4, 4, 1, 1, 3, 3, 1, 1, 1, 1]
            );
            assert_eq!(
                tree.count_subtree_from(6),
                vec![8, 3, 9, 4, 1, 1, 12, 3, 1, 1, 1, 1]
            );
        }

        #[test]
        fn for_strongly_connected_components() {
            let mut tree = TreeDistStrongly::new(7);
            let edges = vec![
                (0, 6),
                (1, 0),
                (2, 6),
                (2, 3),
                (3, 4),
                (4, 3),
                (5, 0),
                (5, 4),
                (6, 1),
                (6, 3),
                (6, 4),
                (6, 5),
            ];
            for (v, w) in edges {
                tree.add_edge(v, w);
            }
            assert_eq!(
                tree.strongly_connected_component(),
                vec![1, 1, 0, 2, 2, 1, 1]
            );
            // decide this graph can have topological ordering
            assert_eq!(
                {
                    let mut v = tree.strongly_connected_component();
                    v.sort();
                    v.iter().zip(v.iter().skip(1)).all(|(&a, &b)| a != b)
                },
                false
            );
            let mut tree = TreeDistStrongly::new(7);
            let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (4, 6), (5, 6)];
            for (v, w) in edges {
                tree.add_edge(v, w);
            }
            assert_eq!(
                tree.strongly_connected_component(),
                vec![0, 3, 1, 6, 4, 2, 5]
            );
            // decide this graph can have topological ordering
            assert_eq!(
                {
                    let mut v = tree.strongly_connected_component();
                    v.sort();
                    v.iter().zip(v.iter().skip(1)).all(|(&a, &b)| a != b)
                },
                true
            );
        }

        #[test]
        fn for_euler_walk_strongly() {
            let mut tree = TreeDistStrongly::new(5);
            let edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 0),
                (0, 2),
                (2, 4),
                (4, 1),
                (1, 3),
                (3, 0),
            ];
            for &(v, w) in &edges {
                tree.add_edge(v, w);
            }
            let walk = tree.eulerian_walk();
            let mut edges = edges.into_iter().collect::<std::collections::HashSet<_>>();
            let mut past = walk.iter().last().unwrap().1;
            for &(v, w) in &walk {
                // connected
                assert_eq!(past, v);
                // edges contains walk's edge
                assert!(edges.remove(&(v, w)));
                past = w;
            }
            // all edges are used
            assert_eq!(edges.len(), 0);
        }

        #[test]
        fn for_euler_walk() {
            let mut tree = TreeDist::new(5);
            let edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 0),
                (0, 2),
                (2, 4),
                (4, 1),
                (1, 3),
                (3, 0),
            ];
            for &(v, w) in &edges {
                tree.add_edge(v, w);
            }
            let walk = tree.eulerian_walk();
            let mut edges = edges.into_iter().collect::<std::collections::HashSet<_>>();
            let mut past = walk.iter().last().unwrap().1;
            for &(v, w) in &walk {
                // connected
                assert_eq!(past, v);
                // edges contains walk's edge
                assert!(edges.remove(&(v, w)));
                past = w;
            }
            // all edges are used
            assert_eq!(edges.len(), 0);
        }
    }
}
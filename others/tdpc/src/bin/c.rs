use proconio::{fastout, input};

const MOD: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        array: [i64; n * n]
    }
    let matrix = Matrix { array, k: n };
    println!(
        "{}",
        matrix
            .pow(k)
            .to_vec()
            .iter()
            .fold(0, |s, e| { (s + e) % MOD })
    );
}

#[derive(Debug, Clone)]
struct Matrix {
    array: Vec<i64>,
    k: usize,
}

impl Matrix {
    #[inline]
    fn to_vec(&self) -> Vec<i64> {
        self.array.clone()
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let k = self.k;
        let mut array = vec![0; k * k];
        (0..k).fold((), |_, i| {
            (0..k).fold((), |_, j| {
                array[i * k + j] = (0..k).fold(0, |e, l| {
                    (e + ((self[i * k + l] * rhs[j + l * k]) % MOD)) % MOD
                })
            })
        });
        Self { array, k }
    }
}

impl std::ops::Mul<Vec<i64>> for Matrix {
    type Output = Vec<i64>;
    #[inline]
    fn mul(self, rhs: Vec<i64>) -> Self::Output {
        let k = self.k;
        let mut vec = vec![0; k];
        (0..k).fold((), |_, i| {
            vec[i] = (0..k).fold(0, |s, j| (s + ((self[i * k + j] * rhs[j]) % MOD)) % MOD);
        });
        vec
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = i64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl Matrix {
    #[inline]
    fn pow(&self, n: usize) -> Self {
        if n == 0 {
            let mut array = vec![0; self.k * self.k];
            (0..self.k).fold((), |_, i| {
                array[i * self.k + i] = 1;
            });
            Self { array, k: self.k }
        } else if n == 1 {
            self.clone()
        } else if n % 2 == 0 {
            (self.clone() * self.clone()).pow(n / 2)
        } else {
            (self.clone() * self.clone()).pow(n / 2) * self.clone()
        }
    }
}

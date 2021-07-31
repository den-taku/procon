#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        broken: [(usize, usize); m]
    }
    let mut array = vec![1u64; n * n];
    for &(u, v) in &broken {
        array[(u - 1) * n + (v - 1)] = 0;
        array[(v - 1) * n + (u - 1)] = 0;
    }
    for i in 0..n {
        array[i * n + i] = 0;
    }
    let matrix = Matrix { array, k: n };
    // let p = pow(n, &array, k + 1);
    let p = matrix.pow(k);
    println!("{}", p[0] % MOD);
}

// #[inline]
// fn _mul(n: usize, a: &[u64], b: &[u64]) -> Vec<u64> {
//     let mut v = vec![0u64; n * n];
//     for i in 0..n {
//         for j in i..n {
//             v[i * n + j] = (0..n).fold(0, |s, k| (s + ((a[i * n + k] * b[k * n + j]) % MOD)) % MOD);
//         }
//     }
//     for i in 0..n {
//         for j in 0..i {
//             v[i * n + j] = v[j * n + i]
//         }
//     }
//     v
// }

// #[inline]
// fn _pow(n: usize, a: &[u64], k: usize) -> Vec<u64> {
//     if k == 0 {
//         let mut v = vec![0; n * n];
//         for i in 0..n {
//             v[i * n + i] = 1;
//         }
//         v
//     } else if k == 1 {
//         let mut v = vec![0; n * n];
//         for i in 0..n * n {
//             v[i] = a[i];
//         }
//         v
//     } else if k == 2 {
//         mul(n, &a.clone(), &a)
//     } else if k == 3 {
//         let p = mul(n, &a.clone(), &a);
//         mul(n, &a, &p)
//     } else if k % 2 == 0 {
//         let p = pow(n, a, k / 2);
//         mul(n, &p.clone(), &p)
//     } else {
//         let p = pow(n, a, k / 2);
//         let p2 = mul(n, &p.clone(), &p);
//         mul(n, &p2, a)
//     }
// }

#[derive(Debug, Clone)]
struct Matrix {
    array: Vec<u64>,
    k: usize,
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

impl std::ops::Mul<Vec<u64>> for Matrix {
    type Output = Vec<u64>;
    #[inline]
    fn mul(self, rhs: Vec<u64>) -> Self::Output {
        let k = self.k;
        let mut vec = vec![0; k];
        (0..k).fold((), |_, i| {
            vec[i] = (0..k).fold(0, |s, j| (s + ((self[i * k + j] * rhs[j]) % MOD)) % MOD);
        });
        vec
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = u64;
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

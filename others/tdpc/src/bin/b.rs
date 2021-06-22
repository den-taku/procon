use proconio::{fastout, input};

const MOD: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        h: usize,
        r: usize,
        g: [u8; r * r]
    }
    // 1.基底と係数行列を求める -> DONE
    let mut c = vec![1i64; r * r];
    // i に向かう組み合わせ
    for i in 0..r {
        // v[v][S] -> v から始まるiへの道の組み合わせ，ただし領域はS(vを含む)
        let mut v = vec![vec![0i64; 2 << r]; r];
        // bitDP
        for j in 0..2 << r {
            if j >> i & 1 == 1 {
                continue;
            }
            for k in 0..r {
                // k bit目が立っている
                if j >> k & 1 == 1 {
                    if g[k * r + i] == 1 {
                        v[k][j] += 1;
                        v[k][j] %= MOD;
                    }
                    for l in 0..r {
                        if j >> l & 1 == 1 && g[l * r + k] == 1 {
                            v[k][j] += v[l][j - (1 << k)] % MOD;
                            v[k][j] %= MOD;
                        }
                    }
                }
            }
        }
        for j in 0..r {
            if j != i {
                c[i * r + j] = v[j][(2 << r) - 1 - (1 << i)] % MOD;
            }
        }
    }
    // 2.累乗を求める
    // let x: Vec<i64> = c.iter().cloned().take(r).collect();
    let mut x = vec![0i64; r];
    x[0] = 1;
    let c = Matrix { array: c, k: r };
    // println!("{}", (c.pow(h - 1) * x)[0]);
    println!("{}", (c.pow(h) * x)[0]);
}

#[derive(Debug, Clone)]
struct Matrix {
    array: Vec<i64>,
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
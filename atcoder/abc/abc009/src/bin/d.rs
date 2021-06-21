use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        m: usize,
        a: [u64; k],
        mut c: [u64; k]
    }
    if m <= k {
        println!("{}", a[m - 1])
    } else {
        let mut tmp = vec![0u64; k * k - k];
        (0..k - 1).fold((), |_, i| {
            tmp[i * k + i] = 0xffff_ffff;
        });
        c.append(&mut tmp);
        let index = m - k;
        let a: Vec<u64> = a.iter().rev().copied().collect();
        println!("{}", (Matrix::new(c, k).pow(index) * a)[0]);
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    array: Vec<u64>,
    k: usize,
}

impl Matrix {
    #[inline]
    fn new(array: Vec<u64>, k: usize) -> Self {
        Self { array, k }
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = u64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
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
                array[i * k + j] = (0..k).fold(0, |e, l| e ^ (self[i * k + l] & rhs[j + l * k]))
            })
        });
        Self::new(array, k)
    }
}

impl std::ops::Mul<Vec<u64>> for Matrix {
    type Output = Vec<u64>;
    #[inline]
    fn mul(self, rhs: Vec<u64>) -> Self::Output {
        let k = self.k;
        let mut vec = vec![0; k];
        (0..k).fold((), |_, i| {
            vec[i] = (0..k).fold(0, |s, j| s ^ (self[i * k + j] & rhs[j]));
        });
        vec
    }
}

impl Matrix {
    #[inline]
    fn pow(&self, n: usize) -> Self {
        if n == 0 {
            let mut e = vec![0; self.k * self.k];
            (0..self.k).fold((), |_, i| {
                e[i * self.k + i] = 1;
            });
            Self::new(e, self.k)
        } else if n == 1 {
            self.clone()
        } else if n % 2 == 0 {
            (self.clone() * self.clone()).pow(n / 2)
        } else {
            (self.clone() * self.clone()).pow(n / 2) * self.clone()
        }
    }
}

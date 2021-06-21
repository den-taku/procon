use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        m: usize,
        mut a: [u32; k],
        c: [u32; k]
    }
    if m < k {
        println!("{}", a[m - 1])
    } else {
        a.append(&mut vec![0; k - 1]);
        (0..k - 1).fold((), |_, i| {
            let mut ans = 0;
            (0..k).fold((), |_, j| {
                ans ^= c[j] & a[i + k - j - 1];
            });
            a[k + i] = ans;
        });
        // println!("{:?}", a);
        if m < 2 * k {
            println!("{}", a[m - 1]);
        } else {
            let mut tmp = vec![0u32; k * k - k];
            for i in 0..k - 1 {
                for j in 0..k {
                    tmp[i * k + j] = if i == j { 1 } else { 0 };
                }
            }
            let mut a: Vec<u32> = a.iter().skip(k - 1).map(|e| *e).collect();
            a.append(&mut tmp);
            let matrix = Matrix::new(a);
            // println!("{:?}", matrix);
            unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    array: Vec<u32>,
    k: usize,
}

impl Matrix {
    fn new(array: Vec<u32>) -> Self {
        let k = array.len();
        Self { array, k }
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = u32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let k = self.k;
        let mut array = vec![0; k * k];
        (0..k).fold((), |_, i| {
            (0..k).fold((), |_, j| {
                array[i * k + j] = (0..k).fold(0, |e, l| e ^ (self[i * k + l] & rhs[j + l * k]))
            })
        });
        Self::new(array)
    }
}

// TODO : Implement 繰り返し二乗法
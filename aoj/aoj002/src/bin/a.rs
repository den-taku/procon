#![allow(dead_code)]
#![allow(unreachable_code)]
use algebra_library::*;

fn main() {
    while let Some((n, s, t, sign, load)) = input() {
        let mut v = vec![0.0; n * n];
        let mut b = vec![0.0; n];
        for i in 0..n {
            if sign[i] == 1 {
                let &min = load[i].iter().filter(|&e| e > &0).min().unwrap();
                let mut count = 0;
                let mut sum = 0;
                let mut flag = false;
                for j in 0..n {
                    if load[i][j] == min {
                        if i == j {
                            flag = true;
                        }
                        count += 1;
                        v[i * n + j] = -1.0;
                        sum += min;
                    }
                }
                if count != 0 {
                    for j in 0..n {
                        v[i * n + j] /= count as f64;
                    }
                    b[i] = sum as f64 / count as f64;
                }
                if flag {
                    v[i * n + i] = (count - 1) as f64 / count as f64;
                } else {
                    v[i * n + i] = 1.0;
                }
            } else {
                let mut count = 0;
                let mut sum = 0;
                let mut flag = false;
                for j in 0..n {
                    if load[i][j] > 0 {
                        if i == j {
                            flag = true;
                        }
                        count += 1;
                        v[i * n + j] = -1.0;
                        sum += load[i][j];
                    }
                }
                if count != 0 {
                    for j in 0..n {
                        v[i * n + j] /= count as f64;
                    }
                    b[i] = sum as f64 / count as f64;
                }
                if flag {
                    v[i * n + i] = (count - 1) as f64 / count as f64;
                } else {
                    v[i * n + i] = 1.0;
                }
            }
        }
        b[t - 1] = 0.0;
        for i in 0..n {
            if i == t - 1 {
                v[(t - 1) * n + i] = 1.0;
            } else {
                v[(t - 1) * n + i] = 0.0;
            }
        }
        let a = Matrix::make_from(n, n, v);
        let b = Matrix::make_from(n, 1, b);
        println!("{}", a);
        println!("{}", b);
        let solve = Matrix::solve_eqn_gauss(&a, &b)[s - 1];
        if solve == 0.0 {
            println!("impossible")
        } else {
            println!("{}", solve);
        }
    }
}

#[inline(always)]
fn input() -> Option<(usize, usize, usize, Vec<usize>, Vec<Vec<usize>>)> {
    let (n, s, t) = {
        let e = read_line::<usize>();
        (e[0], e[1], e[2])
    };
    if n == 0 && s == 0 && t == 0 {
        None
    } else {
        let sign = read_line::<usize>();
        let mut load = Vec::new();
        for _ in 0..n {
            load.push(read_line::<usize>())
        }
        Some((n, s, t, sign, load))
    }
}

#[inline]
fn read_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

pub mod algebra_library {
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Matrix<T> {
        n: usize,
        m: usize,
        array: Vec<T>,
    }

    impl Matrix<f64> {
        pub fn solve_eqn_gauss(a: &Self, b: &Self) -> Self {
            if !(a.is_square() && a.n == b.n && b.m == 1) {
                panic!("`Matix::solve_eqn_gauss` needs n * n matrix and n vector")
            }
            Matrix::backward_substitute(Matrix::forward_erase(a, b))
        }

        #[allow(clippy::needless_range_loop)]
        fn forward_erase(a: &Self, b: &Self) -> Self {
            let a = a.clone();
            let b = b.clone();
            let mut v_a = vec![vec![]; a.n];
            for i in 0..a.n {
                for j in 0..a.m {
                    v_a[i].push(a[i * a.m + j])
                }
            }
            for i in 0..a.n {
                v_a[i].push(b[i]);
            }
            for i in 0..a.n {
                let index = {
                    let mut v_tmp = Vec::new();
                    for j in i..a.m {
                        v_tmp.push((v_a[j][i], j));
                    }
                    v_tmp.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    v_tmp.pop().unwrap().1
                };
                v_a.swap(i, index);
                let a0 = v_a[i][i];
                for j in i..a.m + 1 {
                    v_a[i][j] /= a0;
                }
                for k in i + 1..a.n {
                    let c = v_a[k][i];
                    for l in i..a.m + 1 {
                        v_a[k][l] -= c * v_a[i][l];
                    }
                }
            }
            Matrix::make_from_lines(v_a)
        }

        fn backward_substitute(mut ab: Self) -> Self {
            let nsize = ab.n + 1;
            for i in (0..ab.n).rev() {
                for j in 0..i {
                    ab[(j + 1) * (nsize) - 1] -= ab[j * nsize + i] * ab[(i + 1) * (nsize) - 1];
                }
            }
            let mut v = Vec::new();
            for i in 0..ab.n {
                v.push(ab[i * nsize + nsize - 1])
            }
            Matrix::make_from(ab.n, 1, v)
        }
    }

    impl<T> std::ops::Index<usize> for Matrix<T> {
        type Output = T;
        fn index(&self, index: usize) -> &T {
            if index >= self.n * self.m {
                panic!("index fail: {} is out of range.", index)
            }
            &self.array[index]
        }
    }

    impl<T> std::ops::IndexMut<usize> for Matrix<T> {
        fn index_mut(&mut self, index: usize) -> &mut T {
            if index >= self.n * self.m {
                panic!("index_mut fail: {} is out of range.", index);
            }
            &mut self.array[index]
        }
    }

    impl<T> std::fmt::Display for Matrix<T>
    where
        T: std::fmt::Display + Clone + std::cmp::PartialOrd + Zero,
    {
        fn fmt(&self, dest: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut string = "".to_string();
            for i in 0..self.n {
                for j in 0..self.m {
                    let pad = if self[i * self.m + j] >= T::ZERO {
                        " ".to_string()
                    } else {
                        "".to_string()
                    };
                    string = format!("{}{}{} ", string, pad, self[i * self.m + j].clone());
                }
                string = format!("{}\n", string);
            }
            write!(dest, "{}", string)
        }
    }

    impl<T> Matrix<T> {
        pub fn is_square(&self) -> bool {
            self.n == self.m
        }
    }

    impl<T> Matrix<T>
    where
        T: Clone + Zero,
    {
        pub fn new(n: usize, m: usize) -> Self {
            Matrix {
                n,
                m,
                array: vec![T::ZERO; n * m],
            }
        }
    }

    impl<T> Matrix<T>
    where
        T: Clone,
    {
        pub fn make_from(n: usize, m: usize, array: Vec<T>) -> Self {
            if array.len() != n * m {
                panic!("`Matrix::make_from` needs appropriately sized &[T]")
            }
            Matrix { n, m, array }
        }

        pub fn make_from_lines(vec: Vec<Vec<T>>) -> Self {
            let n = vec.len();
            let m = vec[0].len();
            if !vec.iter().all(|e| e.len() == m) {
                panic!("`Matrix::make_from_lines` needs appropriatly sized Vec<Vec<T>>.");
            }
            Matrix {
                n,
                m,
                array: vec.concat(),
            }
        }
    }

    impl<T> std::ops::Sub<Self> for &Matrix<T>
    where
        T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::sub` needs two Matrix<T> the same sized.")
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() + (-rhs.array[i].clone()))
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::Sub<T> for &Matrix<T>
    where
        T: std::ops::Sub<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn sub(self, rhs: T) -> Self::Output {
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() - rhs.clone());
                    }
                    v
                },
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

    macro_rules! impl_zero_f {
        ( $($e:ty),* ) => {
            $(
                impl Zero for $e {
                    const ZERO: Self = 0.0;
                }
            )*
        };
    }

    impl_zero_f!(f32, f64);

    #[cfg(test)]
    mod tests_matrix {
        use super::*;
        #[test]
        fn for_solve_eqn_gauss() {
            let a = Matrix::make_from(3, 3, vec![5.0, 4.0, 3.0, 4.0, 2.0, 4.0, 1.0, 1.0, 1.0]);
            let b = Matrix::make_from(3, 1, vec![4.2, 3.6, 1.0]);
            let ans = Matrix::solve_eqn_gauss(&a, &b);
            let left = Matrix::make_from(3, 1, vec![0.5, 0.2, 0.3]);
            assert!((&ans - &left).array.iter().all(|&e| e < 1e-5));
        }
    }
}

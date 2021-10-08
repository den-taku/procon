#![allow(dead_code)]

/// Too big
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

        pub fn make_from_culumns(vec: Vec<Vec<T>>) -> Self {
            let n = vec[0].len();
            let m = vec.len();
            if !vec.iter().all(|e| e.len() == n) {
                panic!("`Matrix::append_column` needs appropriatly sized Vec<Vec<T>>.");
            }
            Matrix {
                n,
                m,
                array: {
                    let mut v = Vec::new();
                    for j in 0..n {
                        for ve in vec.iter().take(m) {
                            v.push((ve[j]).clone());
                        }
                    }
                    v
                },
            }
        }

        pub fn transpose(&mut self) {
            let mut new_array = Vec::new();
            for j in 0..self.m {
                for i in 0..self.n {
                    new_array.push(self.array[i * self.m + j].clone());
                }
            }
            self.array = new_array;
            let tmp = self.n;
            self.n = self.m;
            self.m = tmp;
        }

        pub fn map(&mut self, f: std::rc::Rc<dyn Fn(T) -> T>) -> Self {
            for i in 0..self.n * self.m {
                self.array[i] = f(self.array[i].clone())
            }
            self.clone()
        }

        pub fn map_new<R>(&self, f: std::rc::Rc<dyn Fn(T) -> R>) -> Matrix<R> {
            let mut mapped_array = Vec::new();
            for i in 0..self.n * self.m {
                mapped_array.push(f(self.array[i].clone()))
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: mapped_array,
            }
        }
    }

    impl<R, T> Matrix<std::rc::Rc<dyn Fn(R) -> T>>
    where
        R: Clone,
    {
        pub fn applicate(&self, x: &[R]) -> Matrix<T> {
            if self.n * self.m != x.len() {
                panic!("Matrix<R>::applicate needs {} elements", self.n * self.m);
            }
            let mut mapped_array = Vec::new();
            for (i, v) in x.iter().enumerate().take(self.n * self.m) {
                mapped_array.push(self.array[i](v.clone()))
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: mapped_array,
            }
        }
    }

    impl<T> std::ops::Neg for Matrix<T>
    where
        T: std::ops::Neg<Output = T> + Clone,
    {
        type Output = Self;
        fn neg(self) -> Self {
            let new_field = self.array.iter().map(|e| e.clone().neg()).collect();
            Matrix {
                array: new_field,
                ..self
            }
        }
    }

    impl<T> std::ops::Not for Matrix<T>
    where
        T: std::ops::Not<Output = T> + Clone,
    {
        type Output = Self;
        fn not(self) -> Self {
            let new_field = self.array.iter().map(|e| e.clone().not()).collect();
            Matrix {
                array: new_field,
                ..self
            }
        }
    }

    impl<T> std::ops::Add<Self> for &Matrix<T>
    where
        T: std::ops::Add<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn add(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::add` needs two Matrix<T> the same sized.");
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() + rhs.array[i].clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::Add<T> for &Matrix<T>
    where
        T: std::ops::Add<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn add(self, rhs: T) -> Self::Output {
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() + rhs.clone());
                    }
                    v
                },
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
    impl<T> std::ops::Mul<Self> for &Matrix<T>
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Clone + Zero,
    {
        type Output = Matrix<T>;
        fn mul(self, rhs: Self) -> Self::Output {
            if self.m != rhs.n {
                panic!("`Matrix::mul` needs n * m Matrix<T> and m * k Matrix<T>.")
            }
            Matrix {
                n: self.n,
                m: rhs.m,
                array: {
                    let mut v = Vec::<T>::new();
                    for i in 0..self.n {
                        for j in 0..rhs.m {
                            let mut sum = T::ZERO;
                            for k in 0..self.m {
                                sum = sum
                                    + self.array[i * self.m + k].clone()
                                        * rhs.array[j + k * rhs.m].clone()
                            }
                            v.push(sum)
                        }
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::Mul<T> for &Matrix<T>
    where
        T: std::ops::Mul<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn mul(self, rhs: T) -> Self::Output {
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() * rhs.clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::Div<T> for &Matrix<T>
    where
        T: std::ops::Div<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn div(self, rhs: T) -> Self::Output {
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() / rhs.clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::BitAnd for &Matrix<T>
    where
        T: std::ops::BitAnd<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn bitand(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitand` needs two Matrix<T> the same sized.")
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() & rhs.array[i].clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::BitOr for &Matrix<T>
    where
        T: std::ops::BitOr<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn bitor(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitor` needs two Matrix<T> the same sized.")
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() | rhs.array[i].clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::BitXor for &Matrix<T>
    where
        T: std::ops::BitXor<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn bitxor(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitxor` needs two Matrix<T> the same sized.")
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() ^ rhs.array[i].clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::AddAssign<&Self> for Matrix<T>
    where
        T: std::ops::AddAssign + Clone,
    {
        fn add_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::add_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] += rhs.array[i].clone()
            }
        }
    }

    impl<T> std::ops::AddAssign<T> for Matrix<T>
    where
        T: std::ops::AddAssign + Clone,
    {
        fn add_assign(&mut self, rhs: T) {
            for i in 0..self.n * self.m {
                self.array[i] += rhs.clone()
            }
        }
    }

    impl<T> std::ops::SubAssign<&Self> for Matrix<T>
    where
        T: std::ops::SubAssign + Clone,
    {
        fn sub_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::sub_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] -= rhs.array[i].clone()
            }
        }
    }

    impl<T> std::ops::SubAssign<T> for Matrix<T>
    where
        T: std::ops::SubAssign + Clone,
    {
        fn sub_assign(&mut self, rhs: T) {
            for i in 0..self.n * self.m {
                self.array[i] -= rhs.clone()
            }
        }
    }

    impl<T> std::ops::MulAssign<&Self> for Matrix<T>
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Clone + Zero,
    {
        fn mul_assign(&mut self, rhs: &Matrix<T>) {
            if self.m != rhs.n {
                panic!("`Matrix::mul_assign` needs n * m Matrix<T> and m * k Matrix<T>.");
            }
            let mut v = Vec::<T>::new();
            for i in 0..self.n {
                for j in 0..rhs.m {
                    let mut sum = T::ZERO;
                    for k in 0..self.m {
                        sum = sum
                            + self.array[i * self.m + k].clone() * rhs.array[j + k * rhs.m].clone()
                    }
                    v.push(sum)
                }
            }
            self.m = rhs.m;
            self.array = v;
        }
    }

    impl<T> std::ops::MulAssign<T> for Matrix<T>
    where
        T: std::ops::MulAssign<T> + Clone,
    {
        fn mul_assign(&mut self, rhs: T) {
            for i in 0..self.n * self.m {
                self.array[i] *= rhs.clone()
            }
        }
    }

    impl<T> std::ops::DivAssign<T> for Matrix<T>
    where
        T: std::ops::DivAssign<T> + Clone,
    {
        fn div_assign(&mut self, rhs: T) {
            for i in 0..self.n * self.m {
                self.array[i] /= rhs.clone()
            }
        }
    }

    impl<T> std::ops::BitAndAssign<&Self> for Matrix<T>
    where
        T: std::ops::BitAndAssign + Clone,
    {
        fn bitand_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitand_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] &= rhs.array[i].clone()
            }
        }
    }

    impl<T> std::ops::BitOrAssign<&Self> for Matrix<T>
    where
        T: std::ops::BitOrAssign + Clone,
    {
        fn bitor_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitor_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] |= rhs.array[i].clone()
            }
        }
    }

    impl<T> std::ops::BitXorAssign<&Self> for Matrix<T>
    where
        T: std::ops::BitXorAssign + Clone,
    {
        fn bitxor_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitxor_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] ^= rhs.array[i].clone()
            }
        }
    }

    impl<T> std::ops::Shl<usize> for &Matrix<T>
    where
        T: std::ops::Shl<usize, Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn shl(self, rhs: usize) -> Self::Output {
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() << rhs)
                    }
                    v
                },
            }
        }
    }

    impl<T> std::ops::Shr<usize> for &Matrix<T>
    where
        T: std::ops::Shr<usize, Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn shr(self, rhs: usize) -> Self::Output {
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() >> rhs)
                    }
                    v
                },
            }
        }
    }

    pub trait Zero {
        const ZERO: Self;
    }

    macro_rules! impl_ZERO {
            ( $($e:ty),* ) => {
                $(
                    impl Zero for $e {
                        const ZERO: Self = 0;
                    }
                )*
            };
        }

    impl_ZERO!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    macro_rules! impl_ZERO_f {
        ( $($e:ty),* ) => {
            $(
                impl Zero for $e {
                    const ZERO: Self = 0.0;
                }
            )*
        };
    }

    impl_ZERO_f!(f32, f64);

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

        #[test]
        fn test_matrix_new() {
            assert_eq!(
                Matrix::<f32>::new(3, 4),
                Matrix {
                    n: 3,
                    m: 4,
                    array: vec![0.0; 12]
                }
            );
        }

        #[test]
        fn test_matrix_make_from() {
            assert_eq!(
                Matrix::make_from(4, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                }
            );
        }

        #[test]
        fn test_matrix_make_from_lines() {
            assert_eq!(
                Matrix::make_from_lines(vec![
                    vec![1, 2, 3],
                    vec![4, 5, 6],
                    vec![7, 8, 9],
                    vec![10, 11, 12]
                ]),
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                }
            );
        }

        #[test]
        fn test_matrix_make_from_columns() {
            assert_eq!(
                Matrix::make_from_culumns(vec![
                    vec![1, 4, 7, 10],
                    vec![2, 5, 8, 11],
                    vec![3, 6, 9, 12]
                ]),
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::append_column` needs appropriatly sized Vec<Vec<T>>.")]
        fn test_matrix_make_from_columns_panic() {
            let _dummy_matrix = Matrix::make_from_culumns(vec![
                vec![1, 4, 7, 10],
                vec![2, 5, 8, 11],
                vec![3, 6, 9],
            ]);
        }

        #[test]
        fn test_matrix_transpose() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            dummy_matrix.transpose();
            assert_eq!(
                dummy_matrix,
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 5, 9, 2, 6, 10, 3, 7, 11, 4, 8, 12]
                }
            );
            dummy_matrix.transpose();
            assert_eq!(
                dummy_matrix,
                Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                }
            )
        }

        #[test]
        fn test_matrix_map() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            let f: std::rc::Rc<dyn Fn(i32) -> i32> = std::rc::Rc::new(|x| x * x);
            assert_eq!(
                dummy_matrix.map(f),
                Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144]
                }
            );
        }

        #[test]
        fn test_matrix_map_new() {
            let dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            let f: std::rc::Rc<fn(i32) -> f32> = std::rc::Rc::new(|x| x as f32);
            assert_eq!(
                dummy_matrix.map_new(f),
                Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]
                }
            )
        }

        #[test]
        fn test_matrix_applicate() {
            let f: std::rc::Rc<dyn Fn(i32) -> f32> = std::rc::Rc::new(|x| x as f32);
            let matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![f.clone(); 3 * 4],
            };
            assert_eq!(
                matrix.applicate(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
                Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]
                }
            );
        }

        #[test]
        #[should_panic(expected = "Matrix<R>::applicate needs 12 elements")]
        fn test_matrix_applicate_panic() {
            let f: std::rc::Rc<dyn Fn(i32) -> f32> = std::rc::Rc::new(|x| x as f32);
            let _ = Matrix {
                n: 3,
                m: 4,
                array: vec![f.clone(); 12],
            }
            .applicate(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        }

        #[test]
        fn test_matrix_neg() {
            assert_eq!(
                -Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12]
                }
            );
        }

        #[test]
        fn test_matrix_not() {
            assert_eq!(
                !Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false
                    ]
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        !true, !true, !false, !false, !true, !false, !true, !false, !false, !true,
                        !true, !false
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_add_self() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                } + &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 + 1,
                        2 + 2,
                        3 + 3,
                        4 + 4,
                        5 + 5,
                        6 + 6,
                        7 + 7,
                        8 + 8,
                        9 + 9,
                        10 + 10,
                        11 + 11,
                        12 + 12
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_add_t() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                } + 8,
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 + 8,
                        2 + 8,
                        3 + 8,
                        4 + 8,
                        5 + 8,
                        6 + 8,
                        7 + 8,
                        8 + 8,
                        9 + 8,
                        10 + 8,
                        11 + 8,
                        12 + 8
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::add` needs two Matrix<T> the same sized.")]
        fn test_matrix_add_self_panic() {
            let _dummy_matrix = &Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            } + &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
        }

        #[test]
        fn test_matrix_addassign_self() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix += &Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 + 1,
                        2 + 2,
                        3 + 3,
                        4 + 4,
                        5 + 5,
                        6 + 6,
                        7 + 7,
                        8 + 8,
                        9 + 9,
                        10 + 10,
                        11 + 11,
                        12 + 12
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_addassign_t() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix += 8;
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 + 8,
                        2 + 8,
                        3 + 8,
                        4 + 8,
                        5 + 8,
                        6 + 8,
                        7 + 8,
                        8 + 8,
                        9 + 8,
                        10 + 8,
                        11 + 8,
                        12 + 8
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::add_assign` needs two Matrix<T> the same sized.")]
        fn test_matrix_addassign_self_panic() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            dummy_matrix += &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
        }

        #[test]
        fn test_matrix_sub_self() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                } - &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 - 1,
                        2 - 2,
                        3 - 3,
                        4 - 4,
                        5 - 5,
                        6 - 6,
                        7 - 7,
                        8 - 8,
                        9 - 9,
                        10 - 10,
                        11 - 11,
                        12 - 12
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_sub_t() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                } - 8,
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 - 8,
                        2 - 8,
                        3 - 8,
                        4 - 8,
                        5 - 8,
                        6 - 8,
                        7 - 8,
                        8 - 8,
                        9 - 8,
                        10 - 8,
                        11 - 8,
                        12 - 8
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::sub` needs two Matrix<T> the same sized.")]
        fn test_matrix_sub_self_panic() {
            let _dummy_matrix = &Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            } - &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
        }

        #[test]
        fn test_matrix_subassign_self() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix -= &Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 - 1,
                        2 - 2,
                        3 - 3,
                        4 - 4,
                        5 - 5,
                        6 - 6,
                        7 - 7,
                        8 - 8,
                        9 - 9,
                        10 - 10,
                        11 - 11,
                        12 - 12
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_subassign_t() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix -= 8;
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 - 8,
                        2 - 8,
                        3 - 8,
                        4 - 8,
                        5 - 8,
                        6 - 8,
                        7 - 8,
                        8 - 8,
                        9 - 8,
                        10 - 8,
                        11 - 8,
                        12 - 8
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::sub_assign` needs two Matrix<T> the same sized.")]
        fn test_matrix_subassign_self_panic() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            dummy_matrix -= &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
        }

        #[test]
        fn test_matrix_mul_self() {
            assert_eq!(
                &Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                } * &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                },
                Matrix {
                    n: 3,
                    m: 3,
                    array: vec![
                        1 * 1 + 2 * 4 + 3 * 7 + 4 * 10,
                        1 * 2 + 2 * 5 + 3 * 8 + 4 * 11,
                        1 * 3 + 2 * 6 + 3 * 9 + 4 * 12,
                        5 * 1 + 6 * 4 + 7 * 7 + 8 * 10,
                        5 * 2 + 6 * 5 + 7 * 8 + 8 * 11,
                        5 * 3 + 6 * 6 + 7 * 9 + 8 * 12,
                        9 * 1 + 10 * 4 + 11 * 7 + 12 * 10,
                        9 * 2 + 10 * 5 + 11 * 8 + 12 * 11,
                        9 * 3 + 10 * 6 + 11 * 9 + 12 * 12,
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_mul_t() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                } * 8,
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 * 8,
                        2 * 8,
                        3 * 8,
                        4 * 8,
                        5 * 8,
                        6 * 8,
                        7 * 8,
                        8 * 8,
                        9 * 8,
                        10 * 8,
                        11 * 8,
                        12 * 8
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::mul` needs n * m Matrix<T> and m * k Matrix<T>.")]
        fn test_matrix_mul_self_panic() {
            let _dummy_matrix = &Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            } * &Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
        }

        #[test]
        fn test_matrix_mulassign_self() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 3,
                        m: 4,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix *= &Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix
                },
                Matrix {
                    n: 3,
                    m: 3,
                    array: vec![
                        1 * 1 + 2 * 4 + 3 * 7 + 4 * 10,
                        1 * 2 + 2 * 5 + 3 * 8 + 4 * 11,
                        1 * 3 + 2 * 6 + 3 * 9 + 4 * 12,
                        5 * 1 + 6 * 4 + 7 * 7 + 8 * 10,
                        5 * 2 + 6 * 5 + 7 * 8 + 8 * 11,
                        5 * 3 + 6 * 6 + 7 * 9 + 8 * 12,
                        9 * 1 + 10 * 4 + 11 * 7 + 12 * 10,
                        9 * 2 + 10 * 5 + 11 * 8 + 12 * 11,
                        9 * 3 + 10 * 6 + 11 * 9 + 12 * 12,
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_mulassign_t() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                    };
                    dummy_matrix *= 8;
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1 * 8,
                        2 * 8,
                        3 * 8,
                        4 * 8,
                        5 * 8,
                        6 * 8,
                        7 * 8,
                        8 * 8,
                        9 * 8,
                        10 * 8,
                        11 * 8,
                        12 * 8
                    ]
                }
            );
        }

        #[test]
        #[should_panic(
            expected = "`Matrix::mul_assign` needs n * m Matrix<T> and m * k Matrix<T>."
        )]
        fn test_matrix_mulassgin_self_panic() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            dummy_matrix *= &Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
        }

        #[test]
        fn test_matrix_div_t() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]
                } / 8.,
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1. / 8.,
                        2. / 8.,
                        3. / 8.,
                        4. / 8.,
                        5. / 8.,
                        6. / 8.,
                        7. / 8.,
                        8. / 8.,
                        9. / 8.,
                        10. / 8.,
                        11. / 8.,
                        12. / 8.
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_divassign_t() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.],
                    };
                    dummy_matrix /= 8.;
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        1. / 8.,
                        2. / 8.,
                        3. / 8.,
                        4. / 8.,
                        5. / 8.,
                        6. / 8.,
                        7. / 8.,
                        8. / 8.,
                        9. / 8.,
                        10. / 8.,
                        11. / 8.,
                        12. / 8.
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_bitand() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false
                    ]
                } & &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true
                    ]
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true & true,
                        true & false,
                        false & false,
                        false & true,
                        true & true,
                        false & false,
                        true & false,
                        false & true,
                        false & false,
                        true & true,
                        true & false,
                        false & true
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::bitand` needs two Matrix<T> the same sized.")]
        fn test_matrix_bitand_panic() {
            let _dummy_matrix = &Matrix {
                n: 3,
                m: 4,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false,
                ],
            } & &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, false, false, true, true, false, false, true, false, true, false, true,
                ],
            };
        }

        #[test]
        fn test_matrix_bitand_assign() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![
                            true, true, false, false, true, false, true, false, false, true, true,
                            false,
                        ],
                    };
                    dummy_matrix &= &Matrix {
                        n: 4,
                        m: 3,
                        array: vec![
                            true, false, false, true, true, false, false, true, false, true, false,
                            true,
                        ],
                    };
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true & true,
                        true & false,
                        false & false,
                        false & true,
                        true & true,
                        false & false,
                        true & false,
                        false & true,
                        false & false,
                        true & true,
                        true & false,
                        false & true
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::bitand_assign` needs two Matrix<T> the same sized.")]
        fn test_matrix_bitand_assign_panic() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false,
                ],
            };
            dummy_matrix &= &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, false, false, true, true, false, false, true, false, true, false, true,
                ],
            };
        }

        #[test]
        fn test_matrix_bitor() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false
                    ]
                } | &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true
                    ]
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true | true,
                        true | false,
                        false | false,
                        false | true,
                        true | true,
                        false | false,
                        true | false,
                        false | true,
                        false | false,
                        true | true,
                        true | false,
                        false | true
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::bitor` needs two Matrix<T> the same sized.")]
        fn test_matrix_bitor_panic() {
            let _dummy_matrix = &Matrix {
                n: 3,
                m: 4,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false,
                ],
            } | &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, false, false, true, true, false, false, true, false, true, false, true,
                ],
            };
        }

        #[test]
        fn test_matrix_bitor_assign() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![
                            true, true, false, false, true, false, true, false, false, true, true,
                            false,
                        ],
                    };
                    dummy_matrix |= &Matrix {
                        n: 4,
                        m: 3,
                        array: vec![
                            true, false, false, true, true, false, false, true, false, true, false,
                            true,
                        ],
                    };
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true | true,
                        true | false,
                        false | false,
                        false | true,
                        true | true,
                        false | false,
                        true | false,
                        false | true,
                        false | false,
                        true | true,
                        true | false,
                        false | true
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::bitor_assign` needs two Matrix<T> the same sized.")]
        fn test_matrix_bitor_assign_panic() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false,
                ],
            };
            dummy_matrix |= &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, false, false, true, true, false, false, true, false, true, false, true,
                ],
            };
        }

        #[test]
        fn test_matrix_bitxor() {
            assert_eq!(
                &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false
                    ]
                } ^ &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true
                    ]
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true ^ true,
                        true ^ false,
                        false ^ false,
                        false ^ true,
                        true ^ true,
                        false ^ false,
                        true ^ false,
                        false ^ true,
                        false ^ false,
                        true ^ true,
                        true ^ false,
                        false ^ true
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::bitxor` needs two Matrix<T> the same sized.")]
        fn test_matrix_bitxor_panic() {
            let _dummy_matrix = &Matrix {
                n: 3,
                m: 4,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false,
                ],
            } ^ &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, false, false, true, true, false, false, true, false, true, false, true,
                ],
            };
        }

        #[test]
        fn test_matrix_bitxor_assign() {
            assert_eq!(
                {
                    let mut dummy_matrix = Matrix {
                        n: 4,
                        m: 3,
                        array: vec![
                            true, true, false, false, true, false, true, false, false, true, true,
                            false,
                        ],
                    };
                    dummy_matrix ^= &Matrix {
                        n: 4,
                        m: 3,
                        array: vec![
                            true, false, false, true, true, false, false, true, false, true, false,
                            true,
                        ],
                    };
                    dummy_matrix
                },
                Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true ^ true,
                        true ^ false,
                        false ^ false,
                        false ^ true,
                        true ^ true,
                        false ^ false,
                        true ^ false,
                        false ^ true,
                        false ^ false,
                        true ^ true,
                        true ^ false,
                        false ^ true
                    ]
                }
            );
        }

        #[test]
        #[should_panic(expected = "`Matrix::bitxor_assign` needs two Matrix<T> the same sized.")]
        fn test_matrix_bitxor_assign_panic() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false,
                ],
            };
            dummy_matrix ^= &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, false, false, true, true, false, false, true, false, true, false, true,
                ],
            };
        }

        #[test]
        fn test_matrix_shl() {
            assert_eq!(
                &Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                } << 4_usize,
                Matrix {
                    n: 3,
                    m: 4,
                    array: vec![
                        1 << 4,
                        2 << 4,
                        3 << 4,
                        4 << 4,
                        5 << 4,
                        6 << 4,
                        7 << 4,
                        8 << 4,
                        9 << 4,
                        10 << 4,
                        11 << 4,
                        12 << 4
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_shr() {
            assert_eq!(
                &Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                } >> 4_usize,
                Matrix {
                    n: 3,
                    m: 4,
                    array: vec![
                        1 >> 4,
                        2 >> 4,
                        3 >> 4,
                        4 >> 4,
                        5 >> 4,
                        6 >> 4,
                        7 >> 4,
                        8 >> 4,
                        9 >> 4,
                        10 >> 4,
                        11 >> 4,
                        12 >> 4
                    ]
                }
            );
        }

        #[test]
        fn test_matrix_index() {
            let dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            for i in 0..12 {
                assert_eq!(dummy_matrix[i], i + 1)
            }
        }

        #[test]
        #[should_panic(expected = "index fail: 12 is out of range.")]
        fn test_matrix_index_panic() {
            let dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            for i in 0..13 {
                assert_eq!(dummy_matrix[i], i + 1)
            }
        }

        #[test]
        fn test_matrix_index_mut() {
            let mut dummy_matrix = Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            };
            for i in 0..12 {
                dummy_matrix[i] -= 1;
            }
            for i in 0..12 {
                assert_eq!(dummy_matrix[i], i as i32)
            }
        }

        #[test]
        fn test_matrix_is_square() {
            assert_eq!(
                Matrix {
                    n: 2,
                    m: 2,
                    array: vec![9.0; 4]
                }
                .is_square(),
                true
            );
            assert_eq!(
                Matrix {
                    n: 2,
                    m: 3,
                    array: vec![3; 6]
                }
                .is_square(),
                false
            );
        }
    }
}

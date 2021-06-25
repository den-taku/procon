use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        step_per_second: usize,
        time_limit: usize,
        verteces_size: usize,
        adjancy_matrix: [i64; verteces_size * verteces_size]
    }
    let matrix = Matrix {
        array: adjancy_matrix,
        k: verteces_size,
    };
    let unit = matrix.pow(step_per_second);
    println!("{}", {
        let cond = (0..time_limit)
            .fold((std::i64::MIN, unit), |(c, m), _| {
                let mm = m.clone() * m;
                (max(c, mm[1]), mm)
            })
            .0;
        if cond == 0 {
            "IMPOSSIBLE".to_string()
        } else {
            format!("{}", cond)
        }
    });
}

#[derive(Clone, Debug)]
struct Matrix {
    array: Vec<i64>,
    k: usize,
}

impl std::ops::Index<usize> for Matrix {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let k = self.k;
        let mut array = vec![std::i64::MIN; k * k];
        (0..k).fold((), |_, i| {
            (0..k).fold((), |_, j| {
                array[i * k + j] = (0..k).fold(0, |c, l| max(c, self[i * k + l] * rhs[l * k + j]))
            })
        });
        Self { array, k }
    }
}

impl Matrix {
    fn pow(&self, k: usize) -> Self {
        if k == 0 {
            Self {
                array: vec![0i64; k * k],
                k: self.k,
            }
        } else if k == 1 {
            self.clone()
        } else if k % 2 == 0 {
            (self.clone() * self.clone()).pow(k / 2)
        } else {
            (self.clone() * self.clone()).pow(k / 2) * self.clone()
        }
    }
}

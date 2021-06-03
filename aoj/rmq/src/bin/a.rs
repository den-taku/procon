#![allow(dead_code)]
// use proconio::{fastout, input};
use std::fmt::Debug;
use std::ops::{Index, IndexMut, Range};
use std::str::FromStr;

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

// #[fastout]
fn main() {
    // input! {
    //     n: usize,
    //     q: usize,
    //     com: [(usize, usize, usize);q]
    // }
    let condition = read_line::<usize>();
    let n = condition[0];
    let q = condition[1];
    let mut com = Vec::with_capacity(q);
    for _ in 0..q {
        let elem = read_line::<usize>();
        com.push((elem[0], elem[1], elem[2]));
    }
    let com = com;

    let mut segment = SegmentTree::init(n, 2_147_483_647i64);
    for &(d, x, y) in &com {
        match d {
            0 => segment.update(x, y as i64),
            1 => println!(
                "{}",
                match segment.find(x..y + 1) {
                    std::i64::MAX => 2_147_483_647i64,
                    a => a,
                }
            ),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SegmentTree<T> {
    tree: Vec<T>,
    n: usize,
}

impl<T> SegmentTree<T>
where
    T: Ord + Copy,
{
    pub fn update(&mut self, index: usize, value: T) {
        if index >= self.n {
            panic!("SegmentTree::update fail: index is out of bounds");
        }
        let mut k = self.n - 1 + index;
        self[k] = value;
        while k > 0 {
            k = (k - 1) / 2;
            self[k] = std::cmp::min(self[k * 2 + 1], self[k * 2 + 2]);
        }
    }
}

impl<T> SegmentTree<T>
where
    T: Copy + Max + Ord,
{
    pub fn find(&self, range: Range<usize>) -> T {
        self.find_rec(range, 0, 0, self.n)
    }

    fn find_rec(&self, range: Range<usize>, k: usize, l: usize, r: usize) -> T {
        if r <= range.start || range.end <= l {
            T::MAX
        } else if range.start <= l && r <= range.end {
            self[k]
        } else {
            std::cmp::min(
                self.find_rec(range.clone(), k * 2 + 1, l, (l + r) / 2),
                self.find_rec(range, k * 2 + 2, (l + r) / 2, r),
            )
        }
    }
}

impl<T> Index<usize> for SegmentTree<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.tree[index]
    }
}

impl<T> IndexMut<usize> for SegmentTree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tree[index]
    }
}

pub trait Max
where
    Self: Copy,
{
    const MAX: Self;
}

impl Max for i64 {
    const MAX: Self = std::i64::MAX;
}

impl<T> SegmentTree<T> {
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> SegmentTree<T>
where
    T: Ord + Clone + Max,
{
    pub fn new(n: usize) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        Self {
            tree: vec![T::MAX; n_ * 2 - 1],
            n: n_,
        }
    }

    pub fn init(n: usize, init: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        Self {
            tree: vec![init; n_ * 2 - 1],
            n: n_,
        }
    }
}

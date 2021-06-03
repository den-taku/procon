#![allow(dead_code)]
use proconio::{fastout, input};
use std::ops::{Index, IndexMut};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        com: [(usize, usize, usize);q]
    }
    let mut segment = SegmentTree::init(n, 2147483647i64);
    println!("segment[0]: {}", segment[0]);
    println! {"len: {}", segment.len()};
    println!("hoge");
}

#[derive(Debug, Clone)]
pub struct SegmentTree<T> {
    tree: Vec<T>,
    n: usize
}

impl<T> SegmentTree<T>
where
    T: Ord + Copy
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

pub trait Max {
    const MAX: Self;
}

impl Max for i64 {
    const MAX: Self = std::i64::MAX;
}

impl<T> SegmentTree<T> {
    pub fn len(&self) -> usize {
        self.tree.len()
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
            n
        }
    }

    pub fn init(n: usize, init: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        Self {
            tree: vec![init; n_ * 2 - 1],
            n
        }
    }
}

#![allow(dead_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
        com: [(i32, usize); q]
    }
    let mut bit = BIT::<i64>::new(200_000);
    for &(query, num) in &com {
        match query {
            1 => one(&mut bit, num),
            2 => println!("{}", two(&mut bit, num)),
            _ => unreachable!(),
        }
    }
}

fn one(bit: &mut BIT<i64>, num: usize) {
    bit.add(num - 1, 1);
}

fn two(bit: &mut BIT<i64>, index: usize) -> i64 {
    // 総和がi以上になるもののうち最小
    let index = binary_search(index, bit, 0, bit.len() - 1).unwrap() + 1;
    bit.add(index - 1, -1);
    // 総和がちょうどiになる最小のindex
    index as i64
}

fn binary_search(
    index: usize,
    bit: &BIT<i64>,
    lower_bound: usize,
    upper_bound: usize,
) -> Option<usize> {
    if upper_bound - lower_bound == 1 {
        if condition(bit, lower_bound, index) {
            Some(lower_bound)
        } else if condition(bit, upper_bound, index) {
            Some(upper_bound)
        } else {
            None
        }
    } else {
        let est = (upper_bound + lower_bound) / 2;
        if condition(bit, est, index) {
            binary_search(index, bit, lower_bound, est)
        } else {
            binary_search(index, bit, est, upper_bound)
        }
    }
}

fn condition(bit: &BIT<i64>, est: usize, index: usize) -> bool {
    // 個数の合計が欲しいX番目未満
    bit.sum_in_range(0..est + 1) >= index as i64
}

#[derive(Debug, Clone)]
struct BIT<T> {
    tree: Vec<T>,
}

impl<T> BIT<T>
where
    T: From<i32> + Copy,
{
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![T::from(0); n],
        }
    }
}

impl<T> BIT<T>
where
    T: From<i32> + std::ops::Sub<Output = T> + Copy + std::ops::AddAssign,
{
    // indirect index; 0-index
    pub fn sum_in_range(&self, range: std::ops::Range<usize>) -> T {
        (match range.end {
            0 => T::from(0),
            end => self.sum(end),
        }) - match range.start {
            0 => T::from(0),
            start => self.sum(start),
        }
    }

    // direct index; 1-index
    fn sum(&self, i: usize) -> T {
        let mut index = i;
        let mut sum = T::from(0);
        while index > 0 {
            sum += self[index - 1];
            index -= (index as i128 & -(index as i128)) as usize;
        }
        sum
    }
}

impl<T> BIT<T>
where
    T: std::ops::AddAssign + Copy,
{
    pub fn add(&mut self, index: usize, value: T) {
        let mut index = index + 1;
        while index <= self.tree.len() {
            self.tree[index - 1] += value;
            index += (index as i128 & -(index as i128)) as usize;
        }
    }
}

impl<T> BIT<T> {
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> std::ops::Index<usize> for BIT<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.tree[index]
    }
}

impl<T> std::ops::IndexMut<usize> for BIT<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tree[index]
    }
}

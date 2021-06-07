#![allow(dead_code)]
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};
use Query::*;

#[derive(Debug, Clone, Copy)]
enum Query {
    Ans(i64),
    Update(usize, i64),
}

// #[fastout]
fn main() {
    let (n, m) = {
        let tmp = read_line::<usize>();
        (tmp[0], tmp[1])
    };
    let mut value = {
        let mut value = Vec::with_capacity(n);
        (0..n).fold((), |_, _| value.push(read_line::<i64>()[0]));
        value
    };
    // for compression
    let mut set: HashSet<i64> = value.clone().iter().cloned().collect();
    let query = {
        let mut query = Vec::new();
        let _ = (0..m)
            .map(|_| {
                let tmp = read_line::<i64>();
                match tmp[0] {
                    1 => query.push(Ans(tmp[1])),
                    2 => {
                        query.push(Update(tmp[1] as usize, tmp[2]));
                        // for compression
                        set.insert(tmp[2]);
                    }
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();
        query
    };
    // for compression
    let mut vec: Vec<i64> = set.iter().cloned().collect();
    vec.sort();
    let map: HashMap<i64, i64> = vec
        .iter()
        .enumerate()
        .map(|(a, &b)| (b, a as i64))
        .collect();
    // let mut bit = BitSum::<i64>::new(n + m + 1);
    // for i in 0..n {
    //     update(i + 1, *map.get(&(i as i64)).unwrap(), &mut value, &mut bit);
    // }
    // for i in 0..m {
    //     if let Ans(_pos) = query[i] {
    //         println!(
    //             "{}",
    //             bit.sum_in_range(0..(*map.get(&(n as i64 + i as i64)).unwrap()) as usize)
    //         );
    //     } else if let Update(pos, new_value) = query[i] {
    //         update(pos, new_value, &mut value, &mut bit);
    //     }
    // }
    unimplemented!()
}

// fn update(pos: usize, new_value: i64, value: &mut Vec<i64>, bit: &mut BitSum<i64>) {
//     let neighbor_min = std::cmp::min(value[pos - 1], value[pos + 1]);
//     let neighbor_max = std::cmp::max(value[pos - 1], value[pos + 1]);
//     if value[pos] < new_value {
//         if value[pos] < neighbor_min {
//             bit.add_in_range(
//                 value[pos] as usize + 1,
//                 std::cmp::min(neighbor_min, new_value) as usize + 1,
//                 -1,
//             );
//         }
//         if new_value > neighbor_max {
//             bit.add_in_range(
//                 std::cmp::max(value[pos], neighbor_max) as usize + 1,
//                 new_value as usize + 1,
//                 1,
//             );
//         }
//     } else if value[pos] > new_value {
//         if value[pos] > neighbor_max {
//             bit.add_in_range(
//                 std::cmp::max(neighbor_max, new_value) as usize + 1,
//                 value[pos] as usize + 1,
//                 -1,
//             );
//         }
//         if new_value < neighbor_min {
//             bit.add_in_range(
//                 new_value as usize + 1,
//                 std::cmp::min(value[pos], neighbor_min) as usize + 1,
//                 1,
//             );
//         }
//     }
//     value[pos] = new_value;
// }

// impl BitSum<i64> {
//     fn add_in_range(&mut self, l: usize, r: usize, v: i64) {
//         self.add(l, v);
//         self.add(r, -v);
//     }
// }

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

/// Binary Index Tree for query sum of range.
/// bit_sum.sum_in_range(a..b) -> sum in [a,b).
#[derive(Debug, Clone)]
pub struct BitSum<T> {
    tree: Vec<T>,
}

impl<T> BitSum<T>
where
    T: From<i32> + Copy,
{
    /// Make BitSum for g[0]...g[n-1].
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![T::from(0); n],
        }
    }
}

impl<T> BitSum<T>
where
    T: From<i32> + std::ops::Sub<Output = T> + Copy + std::ops::AddAssign,
{
    /// `sum_in_range(a..b)` calculate sum in [a..b).
    /// Of cource, 0-indexed.
    pub fn sum_in_range(&self, range: std::ops::Range<usize>) -> T {
        (match range.end {
            0 => T::from(0),
            end => self.sum(end), // This means Σ_{0<=i<end}
        }) - match range.start {
            0 => T::from(0),
            start => self.sum(start), // This means Σ_{0<=i<start}
        }
    }

    /// This is a private function for sum_in_range.
    /// This calculate sum in [0..i).
    /// Notice that in this function, use 1-indexed array.
    /// 1,2,3,4,... means 0,1,2,3,... in 0-index.
    fn sum(&self, mut index: usize) -> T {
        let mut sum = T::from(0);
        while index > 0 {
            sum += self[index - 1];
            index -= (index as i128 & -(index as i128)) as usize;
        }
        sum
    }
}

impl<T> BitSum<T>
where
    T: std::ops::AddAssign + Copy,
{
    /// add(index, value) do (a[index] += value).
    pub fn add(&mut self, index: usize, value: T) {
        let mut index = index + 1;
        while index <= self.tree.len() {
            self.tree[index - 1] += value;
            index += (index as i128 & -(index as i128)) as usize;
        }
    }
}

impl<T> BitSum<T>
where
    T: Ord + From<i32> + std::ops::Sub<Output = T> + Copy + std::ops::AddAssign,
{
    /// Find index that giving least bit_sum[index] such that is more than value.
    pub fn binary_search_least(&self, value: T) -> Option<usize> {
        self.private_binary_search_least(value, 0, self.len() - 1)
    }

    fn condition_least(&self, est: usize, value: T) -> bool {
        self.sum_in_range(0..est + 1) >= value
    }

    fn private_binary_search_least(
        &self,
        value: T,
        lower_bound: usize,
        upper_bound: usize,
    ) -> Option<usize> {
        if upper_bound - lower_bound == 1 {
            if self.condition_least(lower_bound, value) {
                Some(lower_bound)
            } else if self.condition_least(upper_bound, value) {
                Some(upper_bound)
            } else {
                None
            }
        } else {
            let est = (upper_bound + lower_bound) / 2;
            if self.condition_least(est, value) {
                self.private_binary_search_least(value, lower_bound, est)
            } else {
                self.private_binary_search_least(value, est, upper_bound)
            }
        }
    }
}

impl<T> BitSum<T> {
    /// Retrun length of BitSum.
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> std::ops::Index<usize> for BitSum<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.tree[index]
    }
}

impl<T> std::ops::IndexMut<usize> for BitSum<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tree[index]
    }
}

#[cfg(test)]
mod tests_bit_sum {
    use super::*;

    #[test]
    fn for_bit_sum1() {
        let com = [(1, 11), (1, 29), (1, 89), (2, 2), (2, 2)];
        let answers = [29, 89];
        let mut bit_sum = BitSum::<i64>::new(200_000);
        let mut i = 0;
        for &(direction, value) in &com {
            match direction {
                1 => bit_sum.add(value - 1, 1),
                2 => {
                    let index = bit_sum.binary_search_least(value as i64).unwrap() + 1;
                    bit_sum.add(index - 1, -1);
                    assert_eq!(index, answers[i]);
                    i += 1;
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn for_bit_sum2() {
        let com = [
            (1, 8932),
            (1, 183450),
            (1, 34323),
            (1, 81486),
            (1, 127874),
            (1, 114850),
            (1, 55277),
            (1, 112706),
            (2, 3),
            (1, 39456),
            (1, 52403),
            (2, 4),
        ];
        let answers = [55277, 52403];
        let mut bit_sum = BitSum::<i64>::new(200_000);
        let mut i = 0;
        for &(direction, value) in &com {
            match direction {
                1 => bit_sum.add(value - 1, 1),
                2 => {
                    // sum become i and least
                    let index = bit_sum.binary_search_least(value as i64).unwrap() + 1;
                    bit_sum.add(index - 1, -1);
                    assert_eq!(index, answers[i]);
                    i += 1;
                }
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum Query {
        Add(usize, usize, i64),
        Get(usize, usize),
    }
    use Query::{Add, Get};

    #[test]
    fn for_bit_range_sum1() {
        let (n, _q) = (3, 5);
        let querys = [
            Add(1, 2, 1),
            Add(2, 3, 2),
            Add(3, 3, 3),
            Get(1, 2),
            Get(2, 3),
        ];
        let answers = [4, 8];
        let mut i = 0;

        let mut bit1 = BitSum::<i64>::new(n + 2);
        let mut bit0 = BitSum::<i64>::new(n + 2);

        for &query in &querys {
            match query {
                Add(l, r, x) => {
                    bit0.add(l - 1, -x * (l as i64 - 1));
                    bit1.add(l - 1, x);
                    bit0.add(r - 1 + 1, x * r as i64);
                    bit1.add(r - 1 + 1, -x);
                }
                Get(a, b) => {
                    assert_eq!(
                        {
                            bit1.sum_in_range(0..b) * b as i64 + bit0.sum_in_range(0..b)
                                - bit1.sum_in_range(0..a - 1) * (a - 1) as i64
                                - bit0.sum_in_range(0..a - 1)
                        },
                        answers[i]
                    );
                    i += 1;
                }
            }
        }
    }

    #[test]
    fn for_bit_range_sum2() {
        let (n, _q) = (4, 3);
        let querys = [Get(1, 4), Add(1, 4, 1), Get(1, 4)];
        let answers = [0, 4];
        let mut i = 0;

        let mut bit1 = BitSum::<i64>::new(n + 2);
        let mut bit0 = BitSum::<i64>::new(n + 2);

        for &query in &querys {
            match query {
                Add(l, r, x) => {
                    bit0.add(l - 1, -x * (l as i64 - 1));
                    bit1.add(l - 1, x);
                    bit0.add(r - 1 + 1, x * r as i64);
                    bit1.add(r - 1 + 1, -x);
                }
                Get(a, b) => {
                    assert_eq!(
                        {
                            bit1.sum_in_range(0..b) * b as i64 + bit0.sum_in_range(0..b)
                                - bit1.sum_in_range(0..a - 1) * (a - 1) as i64
                                - bit0.sum_in_range(0..a - 1)
                        },
                        answers[i]
                    );
                    i += 1;
                }
            }
        }
    }
}

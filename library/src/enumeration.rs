#![allow(dead_code)]

// use std::collections::HashSet;
// use std::ops::{Index, IndexMut, Range, Sub};
// use std::convert::From;
// use std::cmp::PartialEq;

// fn main(){
//     // print n!
//     let n = 8;
//     path_enumeration_algorithm(n, |set| println!("{:?}", set.to_vec().as_slice()));
// }

pub fn path_enumeration_algorithm<F>(n: usize, mut f: F)
where
    F: FnMut(&SequencialSet<usize>),
{
    if n < 2 {
        panic!("n must be more than 2");
    }
    // 1.
    let mut sequence = SequencialSet::new((1..n + 1).into_iter().collect());
    // first function applied
    f(&sequence);
    let mut i = n - 1;
    // 2
    let mut k;
    while {
        // 2
        k = (&Set::<usize>::from(sequence.extract(i - 1).unwrap() + 1..n + 2)
            - &sequence.extract_with_range(0..i - 1))
            .min()
            .unwrap();
        // 3
        if k <= n {
            sequence[i - 1] = k;
            if i == n {
                f(&sequence);
            }
            if i < n {
                sequence[i] = 0;
                i += 1;
            }
        }
        if k == n + 1 {
            i -= 1;
        }
        i >= 1
    } {}
}

#[derive(Debug, Clone)]
pub struct SequencialSet<T> {
    sequence: Vec<T>,
}

impl<T> std::ops::Index<usize> for SequencialSet<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.sequence[index]
    }
}

impl<T> std::ops::IndexMut<usize> for SequencialSet<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sequence[index]
    }
}

impl<T> SequencialSet<T>
where
    T: Clone + std::cmp::Eq + std::hash::Hash,
{
    pub fn to_set(&self) -> Set<T> {
        Set::new(self.sequence.iter().cloned().collect())
    }
}

impl<T> SequencialSet<T>
where
    T: Clone,
{
    pub fn to_vec(&self) -> Vec<T> {
        self.sequence.clone()
    }
}

impl<T> SequencialSet<T>
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    pub fn extract_with_range(&self, range: std::ops::Range<usize>) -> Set<T> {
        let mut set = std::collections::HashSet::with_capacity(self.sequence.len());
        for e in self
            .sequence
            .iter()
            .skip(range.start)
            .take(range.end - range.start)
        {
            set.insert(*e);
        }
        Set::<T>::new(set)
    }
}

impl<T> SequencialSet<T>
where
    T: Copy,
{
    pub fn extract(&self, index: usize) -> Option<T> {
        if index < self.sequence.len() {
            Some(self.sequence[index])
        } else {
            None
        }
    }
}

impl<T> SequencialSet<T> {
    pub fn new(sequence: Vec<T>) -> Self {
        Self { sequence }
    }
}

#[derive(Debug, Clone)]
pub struct Set<T> {
    set: std::collections::HashSet<T>,
}

impl<T> Set<T> {
    pub fn new(set: std::collections::HashSet<T>) -> Self {
        Self { set }
    }
}

impl<T> From<std::ops::Range<usize>> for Set<T>
where
    T: std::cmp::Eq + std::hash::Hash + From<usize>,
{
    fn from(range: std::ops::Range<usize>) -> Self {
        let mut set = std::collections::HashSet::with_capacity(range.end - range.start);
        for e in range {
            set.insert(T::from(e));
        }
        Self::new(set)
    }
}

impl<T> std::ops::Sub for &Set<T>
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    type Output = Set<T>;

    fn sub(self, other: Self) -> Self::Output {
        let mut set = std::collections::HashSet::new();
        for x in self.set.difference(&other.set) {
            set.insert(*x);
        }
        Set::new(set)
    }
}

impl<T> Set<T>
where
    T: Ord + Copy,
{
    pub fn min(&self) -> Option<T> {
        if self.set.is_empty() {
            None
        } else {
            let mut ans = *self.set.iter().take(1).next().unwrap();
            for e in self.set.iter() {
                ans = std::cmp::min(ans, *e);
            }
            Some(ans)
        }
    }
}

impl<T> PartialEq<Set<T>> for Set<T>
where
    T: Eq + std::hash::Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set
    }
}

impl<T> PartialEq<SequencialSet<T>> for SequencialSet<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence
    }
}

#[cfg(test)]
mod tests_enumeration {
    use super::*;

    #[test]
    fn for_enumeration3() {
        let n = 3;
        let container = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];
        let mut i = 0;
        path_enumeration_algorithm(n, |set| {
            assert_eq!(container[i], set.to_vec().as_slice());
            i += 1;
        });
    }

    #[test]
    fn for_enumeration4() {
        let n = 4;
        let container = [
            [1, 2, 3, 4],
            [1, 2, 4, 3],
            [1, 3, 2, 4],
            [1, 3, 4, 2],
            [1, 4, 2, 3],
            [1, 4, 3, 2],
            [2, 1, 3, 4],
            [2, 1, 4, 3],
            [2, 3, 1, 4],
            [2, 3, 4, 1],
            [2, 4, 1, 3],
            [2, 4, 3, 1],
            [3, 1, 2, 4],
            [3, 1, 4, 2],
            [3, 2, 1, 4],
            [3, 2, 4, 1],
            [3, 4, 1, 2],
            [3, 4, 2, 1],
            [4, 1, 2, 3],
            [4, 1, 3, 2],
            [4, 2, 1, 3],
            [4, 2, 3, 1],
            [4, 3, 1, 2],
            [4, 3, 2, 1],
        ];
        let mut i = 0;
        path_enumeration_algorithm(n, |set| {
            assert_eq!(container[i], set.to_vec().as_slice());
            i += 1;
        });
    }
}

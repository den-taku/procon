/// compress values
///
/// compress
pub mod compress_library {
    pub fn compress<T>(a: &[T]) -> std::collections::HashMap<T, usize>
    where
        T: Ord + Copy + std::hash::Hash,
    {
        let mut v = Vec::new();
        let mut map = std::collections::HashMap::new();
        for &e in a.iter() {
            v.push(e);
        }
        v.sort();
        v.dedup();
        for &e in a.iter() {
            if map.get(&e).is_none() {
                let mut ub = v.len();
                let mut lb = 0;
                while ub - lb > 1 {
                    let est = (ub + lb) / 2;
                    if v[est] > e {
                        ub = est
                    } else {
                        lb = est
                    }
                }
                if v[lb] == e {
                    map.insert(e, lb);
                } else {
                    map.insert(e, ub);
                }
            }
        }
        map
    }

    #[cfg(test)]
    mod tests_compress {
        use super::*;
        #[test]
        fn for_compress() {
            let coordinates = vec![1, 3, 5, 7, 9];
            let map = compress(&coordinates);
            assert_eq!(
                map,
                vec![(1, 0), (3, 1), (5, 2), (7, 3), (9, 4)]
                    .into_iter()
                    .collect()
            );
        }
    }
}

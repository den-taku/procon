#![allow(dead_code)]

/// Reloting for (Value, Weight, Tree, Func, Unit, Merge, Leaf, Nodes)
/// verified (https://atcoder.jp/contests/abc222/submissions/26477219)
///
/// Value: type of object
/// Weight: type of weight
/// Tree: has edge usize x usize -> Weight
/// Func: Value × Value -> Value
/// Unit: unit of Value
/// Merge: Value × Value × usize × usize -> Value
///     Subproblem, Node value, parent, edge number -> Answer
/// Leaf: Value × usize × usize -> Value
///     Node value, parent, edge number -> Answer
/// Nodes: usize -> Value
pub mod rerooting_library {
    use segment_tree_library::*;

    #[derive(Debug, Clone)]
    pub struct Rerooting<Value, Weight, F, Merge, Leaf>
    where
        F: Fn(Value, Value) -> Value,
        Merge: Fn(Value, Value, usize, usize, Weight) -> Value,
        Leaf: Fn(Value, usize, usize, Weight) -> Value,
    {
        adjacent: Vec<Vec<usize>>,
        cost: Vec<Vec<Weight>>,
        nodes: Vec<Value>,
        parents: Vec<usize>,
        root: Vec<SegmentTree<Value, F>>,
        f: F,
        unit: Value,
        merge: Merge,
        leaf: Leaf,
        n: usize,
    }

    impl<Value, Weight, F, Merge, Leaf> Rerooting<Value, Weight, F, Merge, Leaf>
    where
        Value: Clone,
        Weight: Clone,
        F: Fn(Value, Value) -> Value + Clone,
        Merge: Fn(Value, Value, usize, usize, Weight) -> Value,
        Leaf: Fn(Value, usize, usize, Weight) -> Value,
    {
        pub fn new(
            adjacent: Vec<Vec<usize>>,
            cost: Vec<Vec<Weight>>,
            nodes: Vec<Value>,
            f: F,
            unit: Value,
            merge: Merge,
            leaf: Leaf,
        ) -> Self {
            let parents = vec![0; adjacent.len()];
            let mut root = vec![];
            for v in &adjacent {
                root.push(SegmentTree::new(v.len(), f.clone(), unit.clone()));
            }
            Self {
                n: adjacent.len(),
                adjacent,
                cost,
                nodes,
                parents,
                root,
                f,
                unit,
                merge,
                leaf,
            }
        }

        pub fn solve(&mut self) -> Vec<Value> {
            let mut stack = Vec::new();
            for i in 0..self.adjacent[0].len() {
                stack.push((0, i, false));
                while let Some((parent, i, returned)) = stack.pop() {
                    let node = self.adjacent[parent][i];
                    if !returned {
                        stack.push((parent, i, true));
                        for (j, &sub) in self.adjacent[node].iter().enumerate() {
                            if sub == parent {
                                self.parents[node] = j;
                                continue;
                            }
                            stack.push((node, j, false));
                        }
                    } else if self.adjacent[node].len() == 1 {
                        self.root[parent].update(
                            i,
                            (self.leaf)(
                                self.nodes[node].clone(),
                                parent,
                                i,
                                self.cost[parent][i].clone(),
                            ),
                        );
                    } else {
                        let sub_ans = self.root[node].find(0..self.adjacent[node].len());
                        self.root[parent].update(
                            i,
                            (self.merge)(
                                sub_ans,
                                self.nodes[node].clone(),
                                parent,
                                i,
                                self.cost[parent][i].clone(),
                            ),
                        );
                    }
                }
            }
            let mut dp = vec![self.unit.clone(); self.n];
            dp[0] = self.root[0].find(0..self.adjacent[0].len());
            let mut queue = std::collections::VecDeque::new();
            for i in 0..self.adjacent[0].len() {
                queue.push_back((0, i));
                while let Some((parent, i)) = queue.pop_front() {
                    let node = self.adjacent[parent][i];
                    for (j, &sub) in self.adjacent[node].iter().enumerate() {
                        if sub == parent {
                            continue;
                        }
                        queue.push_back((node, j));
                    }
                    let left = self.root[parent].find(0..i);
                    let right = self.root[parent].find(i + 1..self.adjacent[parent].len());
                    let value = (self.f)(left, right);
                    let parent_ans = (self.merge)(
                        value,
                        self.nodes[parent].clone(),
                        node,
                        self.parents[node],
                        self.cost[node][self.parents[node]].clone(),
                    );
                    self.root[node].update(self.parents[node], parent_ans);
                    dp[node] = self.root[node].find(0..self.adjacent[node].len());
                }
            }
            dp
        }
    }

    #[cfg(test)]
    mod tests_rerooting {
        use super::*;

        #[test]
        fn for_rerooting() {
            let ns = vec![3, 6, 6];
            let edges = vec![
                vec![(1, 2, 2), (2, 3, 3)],
                vec![(1, 2, 3), (1, 3, 1), (1, 4, 4), (1, 5, 1), (1, 6, 5)],
                vec![
                    (1, 2, 1000000000),
                    (2, 3, 1000000000),
                    (3, 4, 1000000000),
                    (4, 5, 1000000000),
                    (5, 6, 1000000000),
                ],
            ];
            let ds = vec![
                vec![1, 2, 3],
                vec![9, 2, 6, 5, 3, 100],
                vec![1, 2, 3, 4, 5, 6],
            ];
            let answers = vec![
                vec![8, 6, 6],
                vec![105, 108, 106, 109, 106, 14],
                vec![
                    5000000006,
                    4000000006,
                    3000000006,
                    3000000001,
                    4000000001,
                    5000000001u64,
                ],
            ];
            for (n, (edges, (d, answer))) in ns
                .into_iter()
                .zip(edges.into_iter().zip(ds.into_iter().zip(answers)))
            {
                let mut adjacent = vec![vec![]; n];
                let mut cost = vec![vec![]; n];
                for (u, v, w) in edges {
                    adjacent[u - 1].push(v - 1);
                    cost[u - 1].push(w);
                    adjacent[v - 1].push(u - 1);
                    cost[v - 1].push(w);
                }
                let mut rerooting = Rerooting::new(
                    adjacent,
                    cost,
                    d,
                    std::cmp::max,
                    0,
                    |sub, no, _p, _i, c| std::cmp::max(sub, no) + c,
                    |no, _p, _i, c| no + c,
                );
                let dp = rerooting.solve();
                assert_eq!(dp, answer);
            }
        }
    }

    /// Segment Tree for Monoid (T, F)
    /// T: Type
    /// F: Binomial operator
    ///
    /// new
    /// update
    /// find
    pub mod segment_tree_library {
        /// verified (https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5953641#1)
        /// Segment Tree for Monoid (T, F)
        #[derive(Debug, Clone)]
        pub struct SegmentTree<T, F>
        where
            F: Fn(T, T) -> T,
        {
            tree: Vec<T>,
            n: usize,
            f: F,
            unit: T,
        }

        impl<T, F> SegmentTree<T, F>
        where
            T: Clone,
            F: Fn(T, T) -> T,
        {
            /// O(n)
            #[inline]
            pub fn new(n: usize, f: F, unit: T) -> Self {
                let mut n_ = 1;
                while n_ < n {
                    n_ *= 2
                }
                Self {
                    tree: vec![unit.clone(); n_ * 2 - 1],
                    n: n_,
                    f,
                    unit,
                }
            }

            /// O(lg n)
            /// a_index <- value
            #[inline]
            pub fn update(&mut self, index: usize, value: T) {
                if index >= self.n {
                    panic!("SegmentTree: out of bound")
                }
                let mut k = self.n - 1 + index;
                self[k] = value;
                while k > 0 {
                    k = (k - 1) / 2;
                    self[k] = (self.f)(self[k * 2 + 1].clone(), self[k * 2 + 2].clone())
                }
            }

            /// O(lg n)
            /// return a_s * a_s+1 * ... * a_t
            #[inline]
            pub fn find(&self, range: std::ops::Range<usize>) -> T {
                if range.end <= range.start {
                    self.unit.clone()
                } else {
                    self.find_rec(range, 0, 0, self.n)
                }
            }

            fn find_rec(&self, range: std::ops::Range<usize>, k: usize, l: usize, r: usize) -> T {
                if r <= range.start || range.end <= l {
                    self.unit.clone()
                } else if range.start <= l && r <= range.end {
                    self[k].clone()
                } else {
                    (self.f)(
                        self.find_rec(range.clone(), k * 2 + 1, l, (l + r) / 2),
                        self.find_rec(range, k * 2 + 2, (l + r) / 2, r),
                    )
                }
            }
        }

        impl<T, F> std::ops::Index<usize> for SegmentTree<T, F>
        where
            F: Fn(T, T) -> T,
        {
            type Output = T;
            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                &self.tree[index]
            }
        }

        impl<T, F> std::ops::IndexMut<usize> for SegmentTree<T, F>
        where
            F: Fn(T, T) -> T,
        {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.tree[index]
            }
        }

        #[cfg(test)]
        mod tests_segment_tree {
            use super::*;

            #[test]
            fn for_rmq1() {
                let n = 3;
                let _q = 5;
                let com = [(0, 0, 1), (0, 1, 2), (0, 2, 3), (1, 0, 2), (1, 1, 2)];
                let answers = &[1, 2];
                let mut rmq = SegmentTree::new(n, std::cmp::min, std::usize::MAX);
                let mut i = 0;
                for &(d, x, y) in &com {
                    match d {
                        0 => rmq.update(x, y),
                        1 => {
                            assert_eq!(rmq.find(x..y + 1), answers[i]);
                            i += 1;
                        }
                        _ => unreachable!(),
                    }
                }
                let n = 1;
                let _q = 3;
                let com = [(1, 0, 0), (0, 0, 5), (1, 0, 0)];
                let answers = &[std::usize::MAX, 5];
                let mut rmq = SegmentTree::new(n, std::cmp::min, std::usize::MAX);
                let mut i = 0;
                for &(d, x, y) in &com {
                    match d {
                        0 => rmq.update(x, y),
                        1 => {
                            assert_eq!(rmq.find(x..y + 1), answers[i]);
                            i += 1;
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

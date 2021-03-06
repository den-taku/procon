#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n- 1]
    }
    let mut tree = tree_dist_library::TreeDist::new(n);
    let mut adjacent = vec![vec![]; n];
    let mut cost = vec![vec![]; n];
    for (u, v) in uv {
        tree.add_edge(u - 1, v - 1);
        adjacent[u - 1].push(v - 1);
        cost[u - 1].push(1);
        tree.add_edge(v - 1, u - 1);
        adjacent[v - 1].push(u - 1);
        cost[v - 1].push(1);
    }
    let subtree = tree.count_subtree_from(0);
    let mut rerooting = rerooting_library::Rerooting::new(
        adjacent,
        cost,
        subtree,
        |a, b| a + b,
        0,
        |s, _, t, _, _, _| s + t,
        |s, p, _, _, _, _| s + n - p,
        |_, _, _, _| 1,
    );
    for v in rerooting.solve() {
        println!("{}", v)
    }
}

/// Rerooting for (Value, Weight, Tree, Func, Unit, Merge, Leaf, Nodes)
/// verified (https://atcoder.jp/contests/abc222/submissions/26477219)
///
/// Value: type of object
/// Weight: type of weight
/// Tree: has edge usize x usize -> Weight
/// Func: Value × Value -> Value
/// Unit: unit of Value
/// Merge1: Value × Value × Value × usize × usize × Weight -> Value (for first)
///     Subproblem, Parent value, Node value, parent, edge number -> Answer
/// Merge2: Value × Value × Value × usize × usize × Weight -> Value (for second)
///     Subproblem, Parent value, Node value, parent, edge number -> Answer
/// Leaf: Value × usize × usize -> Value
///     Node value, parent, edge number -> Answer
/// Nodes: usize -> Value
pub mod rerooting_library {
    use segment_tree_library::*;

    #[derive(Debug, Clone)]
    pub struct Rerooting<Value, Weight, F, Merge1, Merge2, Leaf>
    where
        F: Fn(Value, Value) -> Value,
        Merge1: Fn(Value, Value, Value, usize, usize, Weight) -> Value,
        Merge2: Fn(Value, Value, Value, usize, usize, Weight) -> Value,
        Leaf: Fn(Value, usize, usize, Weight) -> Value,
    {
        adjacent: Vec<Vec<usize>>,
        cost: Vec<Vec<Weight>>,
        nodes: Vec<Value>,
        parents: Vec<usize>,
        root: Vec<SegmentTree<Value, F>>,
        f: F,
        unit: Value,
        merge1: Merge1,
        merge2: Merge2,
        leaf: Leaf,
        n: usize,
    }

    impl<Value, Weight, F, Merge1, Merge2, Leaf> Rerooting<Value, Weight, F, Merge1, Merge2, Leaf>
    where
        Value: Clone,
        Weight: Clone,
        F: Fn(Value, Value) -> Value + Clone,
        Merge1: Fn(Value, Value, Value, usize, usize, Weight) -> Value,
        Merge2: Fn(Value, Value, Value, usize, usize, Weight) -> Value,
        Leaf: Fn(Value, usize, usize, Weight) -> Value,
    {
        pub fn new(
            adjacent: Vec<Vec<usize>>,
            cost: Vec<Vec<Weight>>,
            nodes: Vec<Value>,
            f: F,
            unit: Value,
            merge1: Merge1,
            merge2: Merge2,
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
                merge1,
                merge2,
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
                            (self.merge1)(
                                sub_ans,
                                self.nodes[parent].clone(),
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
                    let parent_ans = (self.merge2)(
                        value,
                        self.nodes[node].clone(),
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
                    |sub, _pv, no, _p, _i, c| std::cmp::max(sub, no) + c,
                    |sub, _pv, no, _p, _i, c| std::cmp::max(sub, no) + c,
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

/// Manage Tree
///
/// new
/// add_edge
/// remove_edge
/// distance
/// count_distance_for_circuit
/// count_subtree_from
/// eulerian_walk
/// strongly_connected_components
pub mod tree_dist_library {
    /// verified by this(https://atcoder.jp/contests/abc218/submissions/25794641)
    pub struct TreeDist {
        nodes: usize,
        pub adjacent: Vec<Vec<usize>>,
    }

    pub struct TreeDistStrongly {
        nodes: usize,
        pub adjacent: Vec<Vec<usize>>,
        pub rev: Vec<Vec<usize>>,
    }

    impl TreeDist {
        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                adjacent: vec![Vec::new(); nodes],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u].push(v);
        }

        pub fn remove_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u] = self.adjacent[u]
                .iter()
                .copied()
                .filter(|&e| e != v)
                .collect();
        }

        pub fn distance(&self, start: usize) -> (Vec<Option<usize>>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            dist[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        queue.push_back(e);
                        pre[e] = s;
                    }
                }
            }
            (dist, pre)
        }

        pub fn count_distance_for_circuit(
            &self,
            start: usize,
            terminal: usize,
        ) -> (Vec<Option<usize>>, Vec<usize>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            let mut count = vec![0; self.nodes];
            dist[start] = Some(0);
            count[start] = 1;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        pre[e] = s;
                        count[e] = count[s];
                        if e != terminal {
                            queue.push_back(e);
                        }
                    } else if let Some(d) = dist[e] {
                        if d == dist[s].unwrap() + 1 {
                            count[e] += count[s];
                            if e != terminal {
                                queue.push_back(e);
                            }
                        }
                    }
                }
            }
            (dist, pre, count)
        }

        /// virifid (https://atcoder.jp/contests/abc220/submissions/26208107)
        /// count subtree's size
        pub fn count_subtree_from(&self, start: usize) -> Vec<usize> {
            let mut count = vec![0; self.nodes];
            self.rec_subtree(self.nodes, start, &mut count);
            count
        }

        fn rec_subtree(&self, from: usize, now: usize, count: &mut [usize]) -> usize {
            let mut sum = 1;
            for &v in &self.adjacent[now] {
                if v == from {
                    continue;
                }
                sum += self.rec_subtree(now, v, count);
            }
            count[now] = sum;
            sum
        }

        pub fn eulerian_walk(&self) -> std::collections::LinkedList<(usize, usize)> {
            let mut edges = vec![std::collections::HashSet::new(); self.nodes];
            for (i, v) in self.adjacent.iter().enumerate() {
                for &e in v {
                    edges[i].insert(e);
                }
            }
            self.eulerian_walk_rec(0, &mut edges)
        }

        fn eulerian_walk_rec(
            &self,
            start: usize,
            edges: &mut [std::collections::HashSet<usize>],
        ) -> std::collections::LinkedList<(usize, usize)> {
            let mut path = vec![start];
            let mut v = start;
            while !edges[v].is_empty() {
                let &w = edges[v].iter().last().unwrap();
                edges[v].remove(&w);
                path.push(w);
                v = w;
            }
            let mut walk = std::collections::LinkedList::new();
            if path.len() >= 2 {
                walk.push_back((path[0], path[1]))
            }
            for (&v, &w) in path.iter().zip(path.iter().skip(1)).skip(1) {
                walk.append(&mut self.eulerian_walk_rec(v, edges));
                walk.push_back((v, w));
            }
            walk
        }
    }

    impl TreeDistStrongly {
        /// return strongly connected component id.
        /// if graph has moderate topological order, then return unique id
        pub fn strongly_connected_component(&self) -> Vec<usize> {
            let mut visited = vec![false; self.nodes];
            let mut order = vec![0; self.nodes];
            let mut inverse = vec![0; self.nodes];
            let mut n = 0;
            for v in 0..self.nodes {
                if !visited[v] {
                    self.visit1(v, &mut n, &mut order, &mut inverse, &mut visited);
                }
            }
            let mut visited = vec![false; self.nodes];
            let mut comp = vec![0; self.nodes];
            let mut k = 0;
            for i in (0..self.nodes).rev() {
                if !visited[inverse[i]] {
                    self.visit2(inverse[i], k, &mut comp, &mut visited);
                    k += 1;
                }
            }
            comp
        }

        pub fn eulerian_walk(&self) -> std::collections::LinkedList<(usize, usize)> {
            let mut edges = vec![std::collections::HashSet::new(); self.nodes];
            for (i, v) in self.adjacent.iter().enumerate() {
                for &e in v {
                    edges[i].insert(e);
                }
            }
            self.eulerian_walk_rec(0, &mut edges)
        }

        fn eulerian_walk_rec(
            &self,
            start: usize,
            edges: &mut [std::collections::HashSet<usize>],
        ) -> std::collections::LinkedList<(usize, usize)> {
            let mut path = vec![start];
            let mut v = start;
            while !edges[v].is_empty() {
                let &w = edges[v].iter().last().unwrap();
                edges[v].remove(&w);
                path.push(w);
                v = w;
            }
            let mut walk = std::collections::LinkedList::new();
            if path.len() >= 2 {
                walk.push_back((path[0], path[1]))
            }
            for (&v, &w) in path.iter().zip(path.iter().skip(1)).skip(1) {
                walk.append(&mut self.eulerian_walk_rec(v, edges));
                walk.push_back((v, w));
            }
            walk
        }

        fn visit1(
            &self,
            v: usize,
            n: &mut usize,
            order: &mut [usize],
            inverse: &mut [usize],
            visited: &mut [bool],
        ) {
            visited[v] = true;
            for &w in &self.adjacent[v] {
                if !visited[w] {
                    self.visit1(w, n, order, inverse, visited);
                }
            }
            order[v] = *n;
            inverse[*n] = v;
            *n += 1;
        }

        fn visit2(&self, w: usize, k: usize, comp: &mut [usize], visited: &mut [bool]) {
            visited[w] = true;
            for &v in &self.rev[w] {
                if !visited[v] {
                    self.visit2(v, k, comp, visited);
                }
            }
            comp[w] = k;
        }

        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                adjacent: vec![Vec::new(); nodes],
                rev: vec![Vec::new(); nodes],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u].push(v);
            self.rev[v].push(u);
        }

        pub fn remove_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u] = self.adjacent[u]
                .iter()
                .copied()
                .filter(|&e| e != v)
                .collect();
            self.rev[v] = self.rev[v].iter().copied().filter(|&e| e != u).collect();
        }

        pub fn distance(&self, start: usize) -> (Vec<Option<usize>>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            dist[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        queue.push_back(e);
                        pre[e] = s;
                    }
                }
            }
            (dist, pre)
        }

        pub fn count_distance_for_circuit(
            &self,
            start: usize,
            terminal: usize,
        ) -> (Vec<Option<usize>>, Vec<usize>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            let mut count = vec![0; self.nodes];
            dist[start] = Some(0);
            count[start] = 1;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        pre[e] = s;
                        count[e] = count[s];
                        if e != terminal {
                            queue.push_back(e);
                        }
                    } else if let Some(d) = dist[e] {
                        if d == dist[s].unwrap() + 1 {
                            count[e] += count[s];
                            if e != terminal {
                                queue.push_back(e);
                            }
                        }
                    }
                }
            }
            (dist, pre, count)
        }

        /// virifid (https://atcoder.jp/contests/abc220/submissions/26208107)
        /// count subtree's size
        pub fn count_subtree_from(&self, start: usize) -> Vec<usize> {
            let mut count = vec![0; self.nodes];
            self.rec_subtree(self.nodes, start, &mut count);
            count
        }

        fn rec_subtree(&self, from: usize, now: usize, count: &mut [usize]) -> usize {
            let mut sum = 1;
            for &v in &self.adjacent[now] {
                if v == from {
                    continue;
                }
                sum += self.rec_subtree(now, v, count);
            }
            count[now] = sum;
            sum
        }
    }

    #[cfg(test)]
    mod tests_tree_dist {
        use super::*;

        #[test]
        fn for_count() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDist::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0],
                    vec![1, 1, 2, 1, 1]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDist::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), None, None],
                    vec![0, 0, 2, 3],
                    vec![1, 1, 0, 0]
                )
            )
        }

        #[test]
        fn for_dist() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDist::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDist::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (vec![Some(0), Some(1), None, None], vec![0, 0, 2, 3])
            )
        }

        #[test]
        fn for_count_subtree() {
            let nodes = 6;
            let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
            let mut tree = TreeDist::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(tree.count_subtree_from(0), vec![6, 1, 1, 1, 1, 1]);

            let nodes = 12;
            let edges = vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 5),
                (2, 6),
                (3, 7),
                (4, 8),
                (7, 9),
                (7, 10),
                (8, 11),
                (8, 12),
            ];
            let mut tree = TreeDist::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(
                tree.count_subtree_from(0),
                vec![12, 3, 4, 4, 1, 1, 3, 3, 1, 1, 1, 1]
            );
            assert_eq!(
                tree.count_subtree_from(6),
                vec![8, 3, 9, 4, 1, 1, 12, 3, 1, 1, 1, 1]
            );
        }

        #[test]
        fn for_count_strongly() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0],
                    vec![1, 1, 2, 1, 1]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), None, None],
                    vec![0, 0, 2, 3],
                    vec![1, 1, 0, 0]
                )
            )
        }

        #[test]
        fn for_dist_strongly() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (vec![Some(0), Some(1), None, None], vec![0, 0, 2, 3])
            )
        }

        #[test]
        fn for_count_subtree_strongly() {
            let nodes = 6;
            let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
            let mut tree = TreeDistStrongly::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(tree.count_subtree_from(0), vec![6, 1, 1, 1, 1, 1]);

            let nodes = 12;
            let edges = vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 5),
                (2, 6),
                (3, 7),
                (4, 8),
                (7, 9),
                (7, 10),
                (8, 11),
                (8, 12),
            ];
            let mut tree = TreeDistStrongly::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(
                tree.count_subtree_from(0),
                vec![12, 3, 4, 4, 1, 1, 3, 3, 1, 1, 1, 1]
            );
            assert_eq!(
                tree.count_subtree_from(6),
                vec![8, 3, 9, 4, 1, 1, 12, 3, 1, 1, 1, 1]
            );
        }

        #[test]
        fn for_strongly_connected_components() {
            let mut tree = TreeDistStrongly::new(7);
            let edges = vec![
                (0, 6),
                (1, 0),
                (2, 6),
                (2, 3),
                (3, 4),
                (4, 3),
                (5, 0),
                (5, 4),
                (6, 1),
                (6, 3),
                (6, 4),
                (6, 5),
            ];
            for (v, w) in edges {
                tree.add_edge(v, w);
            }
            assert_eq!(
                tree.strongly_connected_component(),
                vec![1, 1, 0, 2, 2, 1, 1]
            );
            // decide this graph can have topological ordering
            assert_eq!(
                {
                    let mut v = tree.strongly_connected_component();
                    v.sort();
                    v.iter().zip(v.iter().skip(1)).all(|(&a, &b)| a != b)
                },
                false
            );
            let mut tree = TreeDistStrongly::new(7);
            let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (4, 6), (5, 6)];
            for (v, w) in edges {
                tree.add_edge(v, w);
            }
            assert_eq!(
                tree.strongly_connected_component(),
                vec![0, 3, 1, 6, 4, 2, 5]
            );
            // decide this graph can have topological ordering
            assert_eq!(
                {
                    let mut v = tree.strongly_connected_component();
                    v.sort();
                    v.iter().zip(v.iter().skip(1)).all(|(&a, &b)| a != b)
                },
                true
            );
        }

        #[test]
        fn for_euler_walk_strongly() {
            let mut tree = TreeDistStrongly::new(5);
            let edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 0),
                (0, 2),
                (2, 4),
                (4, 1),
                (1, 3),
                (3, 0),
            ];
            for &(v, w) in &edges {
                tree.add_edge(v, w);
            }
            let walk = tree.eulerian_walk();
            let mut edges = edges.into_iter().collect::<std::collections::HashSet<_>>();
            let mut past = walk.iter().last().unwrap().1;
            for &(v, w) in &walk {
                // connected
                assert_eq!(past, v);
                // edges contains walk's edge
                assert!(edges.remove(&(v, w)));
                past = w;
            }
            // all edges are used
            assert_eq!(edges.len(), 0);
        }

        #[test]
        fn for_euler_walk() {
            let mut tree = TreeDist::new(5);
            let edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 0),
                (0, 2),
                (2, 4),
                (4, 1),
                (1, 3),
                (3, 0),
            ];
            for &(v, w) in &edges {
                tree.add_edge(v, w);
            }
            let walk = tree.eulerian_walk();
            let mut edges = edges.into_iter().collect::<std::collections::HashSet<_>>();
            let mut past = walk.iter().last().unwrap().1;
            for &(v, w) in &walk {
                // connected
                assert_eq!(past, v);
                // edges contains walk's edge
                assert!(edges.remove(&(v, w)));
                past = w;
            }
            // all edges are used
            assert_eq!(edges.len(), 0);
        }
    }
}

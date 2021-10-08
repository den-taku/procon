#![allow(dead_code, clippy::unnecessary_fold)]

/// Bad implementation O(n^4)
pub mod hungarian_library {

    #[derive(Clone, Debug)]
    pub struct Hungarian {
        pub graph: Graph,
    }

    impl Hungarian {
        pub fn new(graph: Graph) -> Self {
            Self { graph }
        }
    }

    impl Hungarian {
        #[inline]
        pub fn maximum_matching_with_graph<T>(&self) -> (T, Graph)
        where
            T: std::convert::From<i32> + std::ops::AddAssign,
        {
            let mut matchings = T::from(0);
            let mut matching = Graph::new(self.graph.vertics());
            let mut match_flag = vec![false; self.graph.vertics()];
            for i in 0..self.graph.vertics() {
                if let Some(path) = self.find_augmenting_path(i, &match_flag, &matching) {
                    let mut past;
                    let mut past_match = false;
                    unsafe { past = *path.get_unchecked(0) }
                    unsafe {
                        *match_flag.get_unchecked_mut(past) = true;
                        let len = path.len();
                        *match_flag.get_unchecked_mut(*path.get_unchecked(len - 1)) = true;
                    }
                    for &e in path.iter().skip(1) {
                        if past_match {
                            matching.remove_edge(past, e)
                        } else {
                            matching.add_edge(past, e)
                        }
                        past = e;
                        past_match ^= true;
                    }
                    matchings += T::from(1);
                }
            }
            (matchings, matching)
        }

        #[inline]
        fn find_augmenting_path(
            &self,
            start: usize,
            match_flag: &[bool],
            matching: &Graph,
        ) -> Option<Vec<usize>> {
            unsafe {
                if *match_flag.get_unchecked(start) {
                    None
                } else {
                    let v = self.graph.vertics();
                    let mut visited = vec![0; v];
                    *visited.get_unchecked_mut(start) = 1;
                    let mut turn = 2;
                    let mut queue_s = std::collections::VecDeque::with_capacity(v);
                    let mut queue_t = std::collections::VecDeque::with_capacity(v);
                    queue_s.push_back(start);
                    let mut end;
                    'out: loop {
                        if visited.iter().fold(1, |m, e| m * e) != 0 {
                            return None;
                        }
                        if queue_s.is_empty() {
                            for e in queue_t {
                                'inner: for i in 0..v {
                                    if *visited.get_unchecked(i) == 0
                                        && *match_flag.get_unchecked(e)
                                        && *self.graph.incidence_matrix.get_unchecked(e * v + i)
                                    {
                                        *visited.get_unchecked_mut(i) = turn;
                                        queue_s.push_back(i);
                                        break 'inner;
                                    }
                                }
                            }
                            queue_t = std::collections::VecDeque::new();
                        } else {
                            for e in queue_s {
                                for i in 0..v {
                                    if *visited.get_unchecked(i) == 0
                                        && *self.graph.incidence_matrix.get_unchecked(e * v + i)
                                    {
                                        queue_t.push_back(i);
                                        *visited.get_unchecked_mut(i) = turn;
                                        if !*match_flag.get_unchecked(i) {
                                            end = i;
                                            break 'out;
                                        }
                                    }
                                }
                            }
                            queue_s = std::collections::VecDeque::new();
                        }
                        turn += 1;
                    }
                    let mut path = Vec::with_capacity(v);
                    path.push(end);
                    let mut m_flag = false;
                    while turn > 1 {
                        *visited.get_unchecked_mut(end) = 0;
                        turn -= 1;
                        if m_flag {
                            for i in 0..v {
                                if *visited.get_unchecked(i) == turn
                                    && *matching.incidence_matrix.get_unchecked(end * v + i)
                                {
                                    path.push(i);
                                    end = i;
                                    break;
                                }
                            }
                        } else {
                            for i in 0..v {
                                if *visited.get_unchecked(i) == turn {
                                    path.push(i);
                                    end = i;
                                    break;
                                }
                            }
                        }
                        m_flag ^= true;
                    }
                    Some(path)
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Graph {
        incidence_matrix: Vec<bool>,
        vertics: usize,
        edges: usize,
    }

    impl Graph {
        #[inline]
        pub fn new(vertics: usize) -> Self {
            Self {
                incidence_matrix: vec![false; vertics * vertics],
                vertics,
                edges: 0,
            }
        }

        #[inline]
        pub fn add_edge(&mut self, a: usize, b: usize) {
            let v = self.vertics();
            if a >= v || b >= v {
                panic!("error: vertix is over |V|")
            }
            unsafe {
                if *self.incidence_matrix.get_unchecked(a * v + b) {
                    panic!("error: edge is already existed.")
                }
                *self.incidence_matrix.get_unchecked_mut(a * v + b) = true;
                *self.incidence_matrix.get_unchecked_mut(b * v + a) = true;
            }
            self.edges += 1;
        }

        #[inline]
        pub fn remove_edge(&mut self, a: usize, b: usize) {
            let v = self.vertics();
            if a >= v || b >= v {
                panic!("error: vertix is over |V|")
            }
            unsafe {
                if !*self.incidence_matrix.get_unchecked(a * v + b) {
                    panic!("error: edge is already removed.")
                }
                *self.incidence_matrix.get_unchecked_mut(a * v + b) = false;
                *self.incidence_matrix.get_unchecked_mut(b * v + a) = false;
            }
            self.edges -= 1;
        }

        #[inline]
        pub fn vertics(&self) -> usize {
            self.vertics
        }

        #[inline]
        pub fn edges(&self) -> usize {
            self.edges
        }
    }

    #[cfg(test)]
    mod tests_hungarian {
        use super::*;

        #[test]
        fn for_graph() {
            let mut graph = Graph::new(3);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 2);
            assert_eq!(graph.edges(), 3);
        }

        #[test]
        fn for_hungarian() {
            let mut graph = Graph::new(3);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 2);
            let hungarian = Hungarian::new(graph);
            assert_eq!(hungarian.maximum_matching_with_graph::<i32>().0, 1);

            let mut graph = Graph::new(4);
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            let hungarian = Hungarian::new(graph);
            assert_eq!(hungarian.maximum_matching_with_graph::<i32>().0, 2);

            // this graph is a quize of cce in 2019
            let mut graph = Graph::new(18);
            let given = [
                (0, 1),
                (0, 6),
                (0, 9),
                (1, 3),
                (1, 7),
                (2, 3),
                (2, 5),
                (2, 12),
                (3, 4),
                (3, 5),
                (4, 8),
                (4, 17),
                (5, 13),
                (5, 14),
                (6, 7),
                (6, 8),
                (7, 8),
                (9, 10),
                (9, 13),
                (10, 11),
                (10, 12),
                (11, 12),
                (11, 13),
                (14, 15),
                (14, 16),
                (15, 16),
                (15, 17),
                (16, 17),
            ];
            for &(i, j) in given.iter() {
                graph.add_edge(i, j)
            }
            let hungarian = Hungarian::new(graph);
            assert_eq!(hungarian.maximum_matching_with_graph::<i32>().0, 9);
        }
    }
}

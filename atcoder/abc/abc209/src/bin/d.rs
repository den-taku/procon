use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        queries: [(usize, usize); q]
    }
    let mut vec = vec![vec![]; n];
    for &(a, b) in &ab {
        vec[a - 1].push(b);
        vec[b - 1].push(a);
    }
    let dis = bfs(n, &vec);
    for &(c, d) in &queries {
        println!(
            "{}",
            if (dis[c - 1] + dis[d - 1]) % 2 == 0 {
                "Town"
            } else {
                "Road"
            }
        )
    }
}

#[inline]
fn bfs(n: usize, graph: &Vec<Vec<usize>>) -> Vec<i32> {
    let mut que = std::collections::VecDeque::<usize>::with_capacity(n);
    let mut v = vec![0; n];
    que.push_back(1);
    while !que.is_empty() {
        let next = que.pop_front().unwrap();
        for &e in &graph[next - 1] {
            if v[e - 1] == 0 {
                v[e - 1] = v[next - 1] + 1;
                que.push_back(e);
            }
        }
    }
    v
}

/* use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        queries: [(usize, usize); q]
    }
    // let mut set1 =
    let mut vec = vec![std::usize::MAX;n * n];
    for &(a, b) in &ab {
        //
    }
    // for k in 0..n {
    //     for i in 0..n {
    //         for j in 0..n {

    //         }
    //     }
    // }
    let mut set = vec![vec![]; n];
    for &(a, b) in &ab {
        set[a - 1].push(b - 1);
        set[b - 1].push(a - 1);
        // set.insert(a, b);
        // set.insert(b, a);
    }
    let graph = convert(&ab, n);
    let mut past = graph.clone();
    let mut next = graph;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                unsafe {
                    let a = *next.get_unchecked(i * n + j);
                    *next.get_unchecked_mut(i * n + j) = min(
                        a,
                        *past.get_unchecked(i * n + k),
                        *past.get_unchecked(k * n + j),
                    );
                }
            }
        }
        // ans += next
        //     .iter()
        //     .filter(|e| e.is_some())
        //     .map(|e| e.unwrap())
        //     .sum::<usize>() as usize;
        past = next.clone();
    }
    for &(c, d) in &queries {
        // let dis = bfs(c, d, n, &set);
        let dis = next[(c-1) * n + (d-1)].unwrap();
        println!("{}", if dis % 2 == 0 { "Town" } else { "Road" });
    }
}

#[inline]
fn bfs(c: usize, d: usize, n: usize, graph: &Vec<Vec<usize>>) -> usize {
    let mut que1 = std::collections::VecDeque::<usize>::with_capacity(n);
    let mut que2 = std::collections::VecDeque::<usize>::with_capacity(n);
    let mut v = vec![false; n];
    let mut dis = 1;
    v[c - 1] = true;
    // let mut now = c;
    que1.push_back(c);
    loop {
        // let dir = graph.get(&now).unwrap();
        // if *dir == d {
        //     return dis;
        // } else {
        //     now = *dir;
        //     dis += 1;
        // }
        if que2.is_empty() {
            for e in que1 {
                // for i in 0..n {
                //     if let Some(_) = graph[(e - 1) * n + i] {
                //         if !v[i] {
                //             v[i] = true;
                //             if i + 1 == d {
                //                 return dis;
                //             }
                //             que2.push_back(i + 1);
                //         }
                //     }
                // }
                for &ele in &graph[e - 1] {
                    if !v[ele] {
                        v[ele - 1] = true;
                        if ele == d {
                            return dis;
                        }
                        que2.push_back(ele);
                    }
                }
                // let dir = graph.get(&e).unwrap();
                // if !v[dir - 1] {
                //     v[dir - 1] = true;
                //     if *dir == d {
                //         return dis;
                //     }
                //     que2.push_back(*dir);
                // }
            }
            que1 = std::collections::VecDeque::<usize>::with_capacity(n);
            dis += 1;
        } else {
            for e in que2 {
                // for i in 0..n {
                //     if let Some(_) = graph[(e - 1) * n + i] {
                //         if !v[i] {
                //             v[i] = true;
                //             if i + 1 == d {
                //                 return dis;
                //             }
                //             que1.push_back(i + 1);
                //         }
                //     }
                // }
                for &ele in &graph[e - 1] {
                    if !v[ele] {
                        v[ele - 1] = true;
                        if ele == d {
                            return dis;
                        }
                        que1.push_back(ele);
                    }
                }
                // let dir = graph.get(&e).unwrap();
                // if !v[dir - 1] {
                //     v[dir - 1] = true;
                //     if *dir == d {
                //         return dis;
                //     }
                //     que1.push_back(*dir);
                // }
            }
            que2 = std::collections::VecDeque::<usize>::with_capacity(n);
            dis += 1;
        }
    }
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: {
                let mut v = Vec::with_capacity(n);
                for i in 0..n {
                    v.push(i);
                }
                v
            },
            rank: vec![0;n]
        }
    }

    fn find(&self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.find(self.par[x])
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x_par = self.find(x);
        let y_par = self.find(y);
        if x_par == y_par {
            return ();
        }
        if self.rank[x_par] < self.rank[y_par] {
            self.par[x_par] = y_par;
        } else {
            self.par[y_par] = x_par;
            if self.rank[x_par] == self.rank[y_par] {
                self.rank[x_par] += 1;
            }
        }
    }

    fn same(&self, x:usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[inline]
fn convert(abc: &[(usize, usize)], n: usize) -> Vec<Option<usize>> {
    let mut v = vec![None; n * n];
    for &(a, b) in abc {
        v[(a - 1) * n + (b - 1)] = Some(1);
        v[(b - 1) * n + (a - 1)] = Some(1);
    }
    // for i in 0..n {
    //     v[i * n + i] = ;
    // }
    v
}

#[inline]
fn min(a: Option<usize>, b: Option<usize>, c: Option<usize>) -> Option<usize> {
    if let Some(e1) = a {
        if let Some(e2) = b {
            if let Some(e3) = c {
                Some(std::cmp::min(e1, e2 + e3))
            } else {
                Some(e1)
            }
        } else {
            Some(e1)
        }
    } else if let Some(e2) = b {
        if let Some(e3) = c {
            Some(e2 + e3)
        } else {
            None
        }
    } else {
        None
    }
}
*/

// TSP(https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_2_A&lang=ja)
fn main() {
    let (v, _e, edges) = input();
    let mut dp = vec![vec![None; v]; 2 << (v - 1)];
    dp[(2 << (v - 1)) - 1][0] = Some(0);
    for s in (0..(2 << (v - 1)) - 1).rev() {
        for i in 0..v {
            let mut min = std::usize::MAX;
            for u in 0..v {
                if s >> u & 1 == 0 {
                    if let Some(e) = dp[s + (1 << u)][u] {
                        if let Some(cost) = edges[i * v + u] {
                            min = std::cmp::min(min, e + cost);
                        }
                    }
                }
            }
            if min != std::usize::MAX {
                dp[s][i] = Some(min)
            }
        }
    }
    if let Some(e) = dp[0][0] {
        println!("{}", e)
    } else {
        println!("-1")
    }
}

fn input() -> (usize, usize, Vec<Option<usize>>) {
    let (v, e) = {
        let input = read_line::<usize>();
        (input[0], input[1])
    };
    let mut edges = vec![None; v * v];
    (0..e).fold((), |_, _| {
        let input = read_line::<usize>();
        edges[input[0] * v + input[1]] = Some(input[2]);
    });
    (v, e, edges)
}

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

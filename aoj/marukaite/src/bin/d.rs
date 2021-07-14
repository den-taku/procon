// Matrix Chain Multiplication(https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_B&lang=ja)

fn main() {
    let n = read_line::<usize>()[0];
    let mut ms = Vec::with_capacity(n);
    let _ = (0..n).fold((), |_, _| {
        let elem = read_line::<i64>();
        ms.push((elem[0], elem[1]));
    });
    let ms = ms;
    if n == 1 {
        println!("0");
        return;
    }
    let mut dp = std::collections::HashMap::new();
    for i in 0..n - 1 {
        dp.insert(
            (i, i),
            (ms[i].0, ms[i + 1].1, ms[i].0 * ms[i].1 * ms[i + 1].1),
        );
        assert_eq!(ms[i].1, ms[i + 1].0);
    }
    println!("{}", rec(0, n - 2, &mut dp, &ms).2);
}

fn rec(
    start: usize,
    end: usize,
    dp: &mut std::collections::HashMap<(usize, usize), (i64, i64, i64)>,
    ms: &[(i64, i64)],
) -> (i64, i64, i64) {
    if let Some(e) = dp.get(&(start, end)) {
        *e
    } else {
        let mut est = (0, 0, std::i64::MAX);
        {
            let (r1, c1, cost1) = (ms[start].0, ms[start].1, 0);
            let (r2, c2, cost2) = rec(start + 1, end, dp, ms);
            assert_eq!(c1, r2);
            let r = r1;
            let c = c2;
            let cost = cost1 + r1 * c1 * c2 + cost2;
            if est.2 > cost {
                est = (r, c, cost);
            }
        }
        for i in start + 1..end {
            let (r1, c1, cost1) = rec(start, i - 1, dp, ms);
            let (r2, c2, cost2) = rec(i + 1, end, dp, ms);
            assert_eq!(c1, r2);
            let r = r1;
            let c = c2;
            let cost = cost1 + r1 * c1 * c2 + cost2;
            if est.2 > cost {
                est = (r, c, cost);
            }
        }
        {
            let (r1, c1, cost1) = rec(start, end - 1, dp, ms);
            let (r2, c2, cost2) = (ms[end + 1].0, ms[end + 1].1, 0);
            assert_eq!(c1, r2);
            let r = r1;
            let c = c2;
            let cost = cost1 + r1 * c1 * c2 + cost2;
            if est.2 > cost {
                est = (r, c, cost);
            }
        }
        dp.insert((start, end), est);
        est
    }
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

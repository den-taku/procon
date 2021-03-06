// Longest Common Subsequence (https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_C) 00.03
#![allow(dead_code)]
#![allow(unreachable_code)]
use std::cmp::max;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock(), 4096);
    input! {
        sc = sc,
        n: usize,
        xys: [(String, String); n]
    }
    let mut ans = Vec::with_capacity(n);
    for (x, y) in xys {
        let mut memo = vec![0; x.len() + 1];
        let mut next = vec![0; x.len() + 1];
        for y in y.chars() {
            for (i, x) in x.chars().enumerate() {
                if y == x {
                    next[i + 1] = max(max(next[i], memo[i + 1]), memo[i] + 1)
                } else {
                    next[i + 1] = max(next[i], memo[i + 1])
                }
            }
            let tmp = memo;
            memo = next;
            next = tmp;
        }
        ans.push(memo.pop().unwrap().to_string())
    }
    println!("{}", ans.join("\n"))
}

#[macro_export]
macro_rules! input{
    (sc=$sc:expr,$($r:tt)*)=>{
        input_inner!{$sc,$($r)*}
    };
    ($($r:tt)*)=>{
        let mut sc=fast_input::Scanner::new(std::io::stdin().lock(),4096);
        input_inner!{sc,$($r)*}
    };
}

#[macro_export]
macro_rules! input_inner{
    ($sc:expr)=>{};
    ($sc:expr,)=>{};
    ($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{
        let $var=read_value!($sc,$t);
        input_inner!{$sc $($r)*}
    };
}

#[macro_export]
macro_rules! read_value{
    ($sc:expr,($($t:tt),*))=>{
        ($(read_value!($sc,$t)),*)
    };
    ($sc:expr,[$t:tt;$len:expr])=>{
        (0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()
    };
    ($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};
    ($sc:expr,Usize1)=>{read_value!($sc,usize)-1};
    ($sc:expr,$t:ty)=>{$sc.next::<$t>()};
}

pub struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}
impl Scanner {
    pub fn new<R: std::io::Read>(mut reader: R, estimated: usize) -> Self {
        let mut buf = Vec::with_capacity(estimated);
        let _ = std::io::copy(&mut reader, &mut buf).unwrap();
        if buf.last() != Some(&b'\n') {
            panic!("{}", 0);
        }
        Scanner { buf, pos: 0 }
    }
    #[inline]
    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        let mut start = None;
        loop {
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) | (b'\n', false) | (b'\r', false) => self.pos += 1,
                (_, false) => start = Some(self.pos),
            }
        }
        let target = &self.buf[start.unwrap()..self.pos];
        unsafe { std::str::from_utf8_unchecked(target) }
            .parse()
            .unwrap()
    }
}

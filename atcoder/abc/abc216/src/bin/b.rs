#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut st: [(String, String); n]
    }
    st.sort();
    st.dedup();
    if st.len() < n {
        println!("Yes")
    } else {
        println!("No")
    }
}

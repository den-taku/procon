use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    a: [i64; n * m]
  }
  let mut max = 0;
  let mut est = 0;
  for i in 0.. m {
    for j in i..m {
      for k in 0..n {
        est += std::cmp::max(a[k * m + i], a[k * m + j]); 
      }
      max = std::cmp::max(max, est);
      est = 0;
    }
  }
  println!("{}", max);
}
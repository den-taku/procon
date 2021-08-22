#[proconio::fastout]
fn main() {
    proconio::input! {
        a: u64,
        b: u64,
        c: u64
    }
    let m = 998244353;
    let a_sum = (a * (a + 1) >> 1) % m;
    let b_sum = (b * (b + 1) >> 1) % m;
    let c_sum = (c * (c + 1) >> 1) % m;
    println!("{}", a_sum * b_sum % m * c_sum % m)
}

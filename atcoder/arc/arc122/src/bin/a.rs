use proconio::{fastout, input};

const MOD: i128 = 1_000_000_007;
const N_MAX: usize = 100_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_: [i128; n]
    }
    let mut a = [0i128; N_MAX];
    (0..n).fold((), |(), i| {
        a[i] = a_[i] % MOD;
    });
    // let a: Vec<i128> = a.iter().map(|e| e % MOD).collect();
    let a = a;
    let mut sub_ans = [0i128; N_MAX];
    let mut len = [0i128; N_MAX];

    (0..n).fold((), |_, i| {
        if i == 0 {
            sub_ans[0] = a[0] % MOD;
            len[0] = 1;
        } else if i == 1 {
            sub_ans[1] = 2 * sub_ans[0] % MOD;
            len[1] = 2;
        } else {
            sub_ans[i] = (((((sub_ans[i - 1] % MOD) + (sub_ans[i - 2] % MOD)) % MOD) % MOD
                + (((a[i] % MOD) * (((len[i - 1] as i128 - len[i - 2] as i128) % MOD as i128) as i128)) % MOD))
                % MOD
                + ((a[i - 1] % MOD) * len[i - 2] % MOD) % MOD)
                % MOD;
            sub_ans[i] %= MOD;
            len[i] = (len[i - 1] % MOD + len[i - 2] % MOD) % MOD;
            len[i] %= MOD;
        }
    });
    println!("{}", sub_ans[n - 1] % MOD);
}

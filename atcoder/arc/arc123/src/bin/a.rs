use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a1: i128,
        a2: i128,
        a3: i128
    }
    let res1 = a2 - a1;
    let res2 = a3 - a2;
    if res1 == res2 {
        println!("0");
        return;
    } else if res1 < 0 && res2 > 0 {
        if res2 - (res1).abs() > 0 {
            let res = res2 - (res1).abs();
            if res % 2 == 0 {
                println!("{}", (res1).abs() + res / 2);
                return;
            } else {
                println!("{}", (res1).abs() + res / 2 + 2);
                return;
            }
        } else if res1.abs() - (res2).abs() > 0 {
            let res = res1.abs() - (res2).abs();
            if res % 2 == 0 {
                println!("{}", (res2).abs() + res / 2);
                return;
            } else {
                println!("{}", (res2).abs() + res / 2 + 2);
                return;
            }
        } else {
            println!("{}", res1.abs());
            return;
        }
    } else if res1 > 0 && res2 < 0 {
        println!("{}", (res2).abs() + res1);
        return;
    } else if res1 < 0 && res2 < 0 {
        if (res1).abs() > (res2).abs() {
            let res = (res1).abs() - (res2).abs();
            if res % 2 == 0 {
                println!("{}", res / 2);
                return;
            } else {
                println!("{}", res / 2 + 2);
                return;
            }
        } else {
            println!("{}", (res2).abs() - (res1).abs());
            return;
        }
    } else if res1 > 0 && res2 > 0 {
        if (res1).abs() < (res2).abs() {
            let res = (res2).abs() - (res1).abs();
            if res % 2 == 0 {
                println!("{}", res / 2);
                return;
            } else {
                println!("{}", res / 2 + 2);
                return;
            }
        } else {
            println!("{}", (res1).abs() - (res2).abs());
            return;
        }
    } else if res1 == 0 {
        if res2 < 0 {
            println!("{}", res2.abs());
            return;
        } else {
            if res2 % 2 == 0 {
                println!("{}", res2 / 2);
                return;
            } else {
                println!("{}", res2 / 2 + 2);
                return;
            }
        }
    } else if res2 == 0 {
        if res1 > 0 {
            println!("{}", res1.abs());
            return;
        } else {
            if (res1).abs() % 2 == 0 {
                println!("{}", res1.abs() / 2);
                return;
            } else {
                println!("{}", res1.abs() / 2 + 2);
                return;
            }
        }
    }
}

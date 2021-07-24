use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        ss: [String; 4]
    }
    let mut flag = [false; 4];
    for s in ss {
        if s.starts_with("HR") {
            flag[0] = true
        } else if s.starts_with("2B") {
            flag[1] = true
        } else if s.starts_with("3B") {
            flag[2] = true
        } else {
            flag[3] = true;
        }
    }
    if flag[0] && flag[1] && flag[2] && flag[3] {
        println!("Yes")
    } else {
        println!("No")
    }
}

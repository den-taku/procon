#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let kanban = "WBWBWWBWBWBW";
    let onkai = [
        "Do", "Do", "Re", "Re", "Mi", "Fa", "Fa", "So", "So", "La", "La", "Si",
    ];
    'out: for (i, m) in onkai.iter().enumerate() {
        for (a, c) in kanban.chars().cycle().skip(i).zip(s.chars()) {
            if a != c {
                continue 'out;
            }
        }
        println!("{}", m);
        break;
    }
}

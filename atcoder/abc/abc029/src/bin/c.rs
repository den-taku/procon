use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut stack = Vec::new();
    stack.push("".to_string());
    while !stack.is_empty() {
        let string = stack.pop().unwrap();
        if string.len() == n {
            println!("{}", string);
        } else {
            stack.push(format!("{}c", string.clone()));
            stack.push(format!("{}b", string.clone()));
            stack.push(format!("{}a", string));
        }
    }
}

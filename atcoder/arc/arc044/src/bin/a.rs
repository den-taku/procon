use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!(
        "{}",
        match n {
            1 => "Not Prime",
            2 => "Prime",
            3 => "Prime",
            5 => "Prime",
            n =>
                if n % 2 != 0 && n % 5 != 0 && n % 3 != 0 {
                    "Prime"
                } else {
                    "Not Prime"
                },
        }
    );
}


// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//     }
//     println!(
//         "{}",
//         if n == 1 {
//             "Not Prime"
//         } else if n % 2 != 0 && n % 5 != 0 && n % 3 != 0 {
//             "Prime"
//         } else if n.is_prime() {
//             "Prime"
//         } else {
//             "Not Prime"
//         }
//     );
// }

// trait Prime {
//     fn is_prime(self) -> bool;
// }

// impl Prime for usize {
//     fn is_prime(self) -> bool {
//         if self == 1 {
//             false
//         } else {
//             for i in 2..self {
//                 if i * i > self {
//                     break;
//                 } else if self % i == 0 {
//                     return false;
//                 }
//             }
//             true
//         }
//     }
// }

use library::enumeration::enumeration_library::path_enumeration_algorithm;
use library::primes::primes_library::Seive;

fn main() {
    let mut buffer = Vec::new();
    for e in Seive::<usize>::new().take(100) {
        buffer.push(e.to_string())
    }
    println!("{}", buffer.join("\n"));
    path_enumeration_algorithm(4, |set| println!("{:?}", set.to_vec().as_slice()));
}

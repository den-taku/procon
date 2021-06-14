use library::enumeration::enumeration_library::path_enumeration_algorithm;

fn main() {
    // println!("Hello, world!");
    path_enumeration_algorithm(4, |set| println!("{:?}", set.to_vec().as_slice()));
}

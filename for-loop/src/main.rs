fn main() {
    let numbers = 0..11;

    for i in numbers {
        println!("{}", i);
    }

    let vector = vec!["a", "b", "c"];

    for (i, c) in vector.iter().enumerate() {
        println!("{}: {}", i, c);
    }
}

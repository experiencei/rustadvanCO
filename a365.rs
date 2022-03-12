// activity on array & slices

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

fn process_chunk(data: &[u64]) {
    match data {
        [lhs, rhs] => println!("{}+{}={}", lhs, rhs, lhs + rhs),
        [single] => println!("Unpaired value: single"),
        [] => println!("Data stream complete"),
        [..] => unreachable!("chunk size should be at most 2"),
    }
}

fn main() {
    // `stream` is an iterator of Option<&[u64]>
    let mut stream = data().chunks(2);
    for chunk in stream {
        process_chunk(chunk);
    }
}

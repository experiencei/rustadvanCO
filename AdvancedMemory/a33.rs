fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}

fn main() {
    let short = "hello";
    let long = "this is a long message";
    println!("{}", longest(short, long))
}
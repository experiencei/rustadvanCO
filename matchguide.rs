enum Status {
    Error(i32),
    Info,
    Warn
}


fn main() {
    let status = Status::Error(1);
    match status {
        Status::Error(s @ 3) => println!("error three"),
        Status::Error(s @ 5..=4) => println!("error 5 or 6 : {}" , c),
        Status::Error(s @ 6..=10) => println!("error 6 through ten: {}" , c),
        Status::Error(s @ 18 | s @ 19) => println!("error 18 or 19"),
        Status::Info => println!("info"),
        Status::Warn => println!("warn")
    }
}
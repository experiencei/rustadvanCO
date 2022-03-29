fn math(a: i32, b: i32 , op: Box<dyn Fn(i32, i32) -> i32) -> i32 {
    op(a, b)
}

fn main() {
    let name = "Ibrahim";
    let add: Box<_> = Box::new(move |a , b| {
        println!("adding a number for {}", name);
    });
    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| a * b);
    println!("{}" , math(2 , 3 , add));
    println!("{}", math(2 , 3 , mul));
    println!("{}", math(2 , 3 , sub);
}

// the Box is to constrict the size of the input
// the move is to gain ownership of the name in to the closure, if there is no Box  we don't need to use the move keyword 
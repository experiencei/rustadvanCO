#[derive(Debug ,Copy , Clone)]
struct NeverZero(i32);

impl NeverZero {
    pub fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("Cannot be Zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn divides(a: i32, b: NeverZero) -> i32 {
    let b = b.0;
    a/b
}

fn main() {
    match NeverZero::new(0) {
        OK(nz) => println!("{:?}" , divides(10 , nz)),
        Err(err) => println!("{:?}", err)
    }
}
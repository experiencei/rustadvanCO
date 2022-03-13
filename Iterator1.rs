trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Demo 1

struct Odd {
    number: isize, 
    max : isize,
}


impl Iterator for Odd {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.number += 2;
        if self.number <= self.max {
            Some(self.number)
        } else {
            None
        }
    }
}

impl Odd {
    fn new(max: isize) -> Self {
        Self {number: -1, max}
    }
}

let mut odds = Odd::new(7);
println!("{:?}", odds.next());  Some(1)
println!("{:?}", odds.next());  Some(3)
println!("{:?}", odds.next());  Some(5)
println!("{:?}", odds.next());  Some(7)
println!("{:?}", odds.next());  None


// Demo 2 for ... in

let mut odds = Odd::new(7);
for o in odds {
    println!("odd: {}" , o)
}

// Demo 3 Adapter
let mut evens = Odd::new(8);
for e in evens.map(|odd| odd + 1) {
    println!("even: {:?}", e)
}
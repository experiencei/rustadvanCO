// Immutability Cell & RefCell

use std::cell::Cell;


///*** NOte : Cell return a owned data.

#[derive(Debug)]
struct Book {
    signed : Cell<bool>,
}

impl Book {
    fn sign(&self) {
        self.signed.set(true);
    }
    fn signed(&self) -> bool {
        self.signed.get()
    }
}

let my_book = Book { 
    signed: Cell::new(false)
};

println!("signed: {}", my_book.signed());
my_book.sign();
println!("signed: {}", my_book.signed()); 


// &&&Demo 2
use std::cell::Cell; 

///Note : refCell return a borrowed Data
use std::cell::RefCell;

struct Person {
    name : RefCell<String>,
}


let name = "Amy".to_owned();

let person = Person {
    name : RefCell(name)
}

let name = Person.name.borrow();  // comment 1

let mut name = person.name.borrow_mut(); // comment2 we will get compiler error here borrrowing twice
*name = "Tim".to_owned();



{
    let mut name = person.name.borrow_mut(); //same as above but scoping 
     person.name.replace("Tim".to_owned());

}

{
   
person.name.replace("Tim".to_owned());
}

// if we re to use without scoping then use """try_borrow""";

let name: Result<_ , _> = person.name.try_borrow();
let name: Result<_ , _> = person.name.try_borrow_mut();
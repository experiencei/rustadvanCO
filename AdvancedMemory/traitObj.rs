trait Clicky {
    fn click(&self);
}

struct Keyboard;

impl Clicky for Keyboard {
    fn click(&self) {
        println!("click clack")
    }
}

let keeb = Keyboard;
let keeb_obj: &dyn Clicky = &keeb;

let keeb: &dyn Clicky = &Keyboard;

let keeb: Box<dyn Clicky> = Box::new(Keyboard);  
//instead of borrowing it, we can just use this.


// if you want to **borrow trait obj
fn borrow_clicky(obj: &dyn Clicky) [
    obj.click();
]

let keeb = Keyboard;
borrow_clicky(&keep);


// if you want to **move trait obj

fn move_clicky(obj: Box<dyn Clicky>)  {
    obj.click();
}

let keeb = Box::new(Keyboard);
move_clicky(keep);


//Heterogeneous Vector//

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) {
        println!("click");
    }
}

let mouse: Box<dyn Clicky> = Box::new(Mouse);
let keeb: Box<dyn Clicky> = Box::new(Keyboard);
let clickers = vec![keeb, mouse];


let keeb = Box::new(Keyboard);
let mouse = Box::new(Mouse);
let clickers: Vec<Box<dyn Clicky>> = vec![keeb , mouse]


fn make_clicks(clickeys: Vec<Box<dyn Clicky>>) {
    for clicker in clickeys {
        clicker.click();
    }
}

let keeb = Box::new(Keyboard);
let mouse = Box::new(Mouse);
let clickers: Vec<Box<dyn Clicky>> = vec![keeb , mouse]

make_clicks(clickers);

//Note: always store in a box when working with data;


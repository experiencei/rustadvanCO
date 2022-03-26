use derive_more::Display;

#[derive(Display)]
#[display(fmt = "Item: {} , Quantity: {}" , item , qty)]
struct Order {
    item: String,
    qty: usize,
}

// display struct 
let order = Order {
    item: String::from("Apple"),
    qty: 3,
};

println!("{}", order);

==** Item: Apple , Quantity: 3 **==

// Enum Example
use derive_more::Display;

#[derive(Display)]
enum GroceryItem {
    #[display(fmt = "Bread slices: {}", _0)]
    Bread(usize),
    #[display(fmt = "Fruit")]
    Fruit, 
    #[display(fmt = "Ounces of meat: {}", _0)]
    Meat(usize),
}


let bread = GroceryItem::Bread(10);
let fruit = GroceryItem::Fruit;
let meat = GroceryItem::Meat(6)
println!("{}" bread);  //Bread slices: 10
println!("{}", fruit);  //Fruit
println!("{}", meat)  // Ounces of meat: 6



// Enum Example @222
use derive_more::Display;

#[derive(Display)]
#[display(fmt = "Grocery item : {}")]
enum GroceryItem {
    #[display(fmt = "{} Bread slices", _0)]
    Bread(usize),
    #[display(fmt = "fruit")]
    Fruit, 
    #[display(fmt = "{} ounces of meat", _0)]
    Meat(usize),
}


let bread = GroceryItem::Bread(10);
let fruit = GroceryItem::Fruit;
let meat = GroceryItem::Meat(6)
println!("{}" bread);  //Grocery item: Bread slices10
println!("{}", fruit);  //Grocery item: Fruit
println!("{}", meat)  // Grocery item: ounces of meat


// from --> Tuple Struct
use derive_more::From;

#[derive(From)]
struct UserId(i32);

let user_id: UserId = 15.into();
let user_id = UserId::from(15);

// From - Struct
use derive_more::From;

#[derive(From)]
struct Coordinate {
    x: i32,
    y: i32,
}

let coord: Coordinate = (3, 2 ).into();
let coord = Coordinate::from((3, 2));

//From --> Enum
use derive_more::From;

#[derive(From)]
enum Material {
    Flooring(usize , usize),
    Wood(usize)
}

let floor: Material = (5 , 5).into();
let floor = Material::from((5 , 5));
let wood: Material = 10.into();
let wood = Material::from(10);

// INTOITERATOR

use derive_more::IntoIterator;

#[derive(IntoIterator)]
struct Passenger {
    #[into_iterator(owned, ref, ref_mut)]
    names: Vec<String>,
}

let passengers = Passenger {
    names: vec![]
}
for passenger in &Passenger{}

//Arithemetic
use derive_more::{Add , From , Sub};

#[derive(Add , Debug , From , Sub)]
struct Point {
    x: i32,
    y: i32,
}

let a: Point = (1 , 1).into();
let b: Point = (2 , 3).into();
let c = a + b;
println!("{:?}", c) // point { x: 3, y: 4}

// Constructor
use derive_more::Constructor;

#[derive(Debug , Constructor)]
struct Hits(u64);

impl Hits {
    fn new(data: u64) -> Self {
        Self(data)
    }
}


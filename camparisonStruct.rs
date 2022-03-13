#[derive(PartialEq)]
struct User {
    id: i32,
    name: String,
}

let a = User {id : 1, name : "a".to_owned()};
let b = User {id: 2, name : "b".to_owned()};


if a == b { 
    //. code will never execute because it not equal
    // no PartialEquality
}

#[derive(PartialEq , PartialOrd)]
struct User {
    id: i32,
    name: String,
}

if a < b { 
   // it will only match on **id''
}

// PartiaLOrd - Manual Implementation;
use std::cmp::Ordering;

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.name < other.name {
            Somee(Ordering::Less)
        } else if self.name > other.name { 
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(PartialEq , PartialOrd)] 
// #[derive(Ord)]   .cmp() is made  available through #[derive(Ord)] 
struct User {
    id: i32,
    name: String,
}

// Manual Implementation for primitive types
use std::cmp::Ordering;

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}


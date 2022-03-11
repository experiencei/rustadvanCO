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

// example 2 , .. here means to ignore all other things

enum Species {
    Finch,
    Hawk,
    Parrot,
}

struct Bird {
    age : usize,
    species : Species,
}

#[rustfmt::skip]

fn main() {
    let hawk = Bird {
        age : 13, 
        species: Species::Hawk,
    }

  match hawk {
      Bird { age: 4, ..} => println!("4 years old bird"),
      Bird { age: 5..=10| 15..=20, ..} => println!("5-10 or 15-20 years old bird"),
      Bird {species: Species::Finch, ..} => println!("finch"),
      Bird {..} => println!("other bird"),
  }
}

// example 3 
#[derive(PartialEq , PartialOrd, Eq , Ord)]
enum Difficulty {
    Easy , 
    Normal , 
    Hard ,
}

fn main() {
    let stage = 5,
    let diff = Difficulty::Easy;

    match stage {
        s if (s == 5 && diff = Difficulty::Normal) => println!("normal mode stage"),
        s if diff == Difficulty::Normal => println!("normal difficulty stage {}" , s),
        s @ 10 | s @ 15 => println!("stage 10 or 15"),
        s => println!("stage {}" , stage),
    }
}
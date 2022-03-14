 Cargo.toml
 [dependencies]
 strum = {version = "0.20", features = ["derive"]}

 use strum::EnumCounters;

 #[derive(Debug , EnumCount)]
 enum Color {
     Red,
     Green,
     Blue,
 }

 println!("Variant count : {:?}", Color::COUNT);


 // Demo 2
 use strum::{EnumIter , IntoEnumIterator};
 #[derive(Debug , EnumIter)]
 enum Color {
    Red,
    Green,
    Blue,
}

for variant in Color::iter() {
    println!("{:?}", variant);
}

//Demo 3
use strum::EnumMessage;

#[derive(Debug , EnumMessage)]

enum Status {
    #[strum(
        message = "Idle",
        detailed_message = "waiting for jobs"
    )]
    Idle,
    Processing,
}


let idle = Status::Idle;
println!("{:?}", idle.get_message());   //Some("Idle")
println!("{:?}", idle.get_detailed_message());  //Some("Wait for jobs")
let processing = Status::Processing;
println!("{:?}", processing.get_message());  // None


// Demo 4 String --> Enum
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug , EnumString)]
enum Status {
    #[strum(serialize = "i" , serialize = "Idle")]
    Idle,
    #[strum(serialize = "p")]
    Processing
}


let idle = Status::from_str("i"); //Ok("Idle");
let idle = Status::from_str("Idle"); //Ok("Idle")
let processing = Status::from_str("p"); //Ok("P");
let processing = Status::from_str("processing"); //Err(NotFound)
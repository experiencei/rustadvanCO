trait CheckIn {
    fn check_In(self);
    fn process(&self);

}

struct Pilot;

impl CheckIn for Pilot {
    fn check_In(self){
        println!("checked in as pilot");
    }
    fn process(&self) {
        println!("pilot enters the cockpit")
    }
}

struct Passenger;
impl CheckIn for Passenger {
    fn check_In(self){
        println!("checked in as passenger");
    }
    fn process(&self) {
        println!("passenger take a seat");
    }
}

struct Cargo;
impl CheckIn for Cargo {
    fn check_In(self){
        println!("cargo checked in ");
    }
    fn process(&self) {
        println!("cargo moved to storage");
    }
}

// fn func(param: impl Trait) {}
// fn func<T: Trait> (param: T) {} 

fn process_item<T: CheckIn> (item: T) {
    item.check_in();
    item.process();
}

fn main() {
let paul = Passenger;
let sade = Pilot;
let cargo = Cargo;

process_item(paul);
process_item(sade);
process_item(cargo);
}
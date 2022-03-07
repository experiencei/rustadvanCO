fn function (param1: impl Trait1, param2: impl Trait2) {

}

fn function<T: Trait1 , U:Trait2>(param1: T, param2: U) {

}

fn function<T , U>(param1: T , param2: U) {
    where 
    T: Trait1 + Trait2,
    U: Trait1 + Trait2 + Trait3 + Trait4 
}

// 3 synthax of generic function
fn func(param: impl Trait) {}
fn func<T: Trait> (param: T) {} 
fn func<T>(param : T) where T: Trait {}
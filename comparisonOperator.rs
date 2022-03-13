#[derive(PartialEq)]
enum Floor {
    ClientServices,
    Marketing,
    OPs,
}

let first = Floor::ClientServices;
let second = Floor::Marketing;

if first == second { 
    //. code will never execute because it not equal
    // no PartialEquality
}

#[derive(PartialEq , PartialOrd)]
enum Floor {
    ClientServices,
    Marketing,
    OPs,
}

fn is_below(this: &Floor, other: &Floor) -> bool {
    this < other
}

//example partialOrd w/varient Data
#[derive(PartialEq , PartialOrd)]
enum Tax {
    Flat(f64),
    None,
    Percentage(f64),
}

fn smallest_amount(tax: Tax, other: Tax) -> Tax {
     if tax < other {
         tax
     } else {
         other
     }
}


// here Flat will always less than none
let no_tax = Tax::None;
let flat_tax = Tax::Flat(5.5);

//percentage will always greater than Flat irrespective of number
let flat_tax = Tax::Flat(0.3);
let percent = Tax::Percentage(1.0);


//Flat with higher number is greater.
let low = Tax::Flat(5.5);
let high = Tax::Flat(8.5);
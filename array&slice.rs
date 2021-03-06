//Array
let numbers = [1 ,2 ,4];
let numbers: [u8; 4] = [1 , 2 , 3 , 98];


fn func(arr: [u8; 4]) {}
fn func(arr: &[u8]) {}
fn func(arr: &mut [u8]) {} 


//.as_slice is used to slice a part of num
// slices & vecotrs
fn func(slice: &[u8]) {}

let numbers = vec![1 , 2 , 3];

func(&numbers);
let numbers: &[u8] = numbers.as_slice();

// slicing the ranges to
let chars = vec!['A' , 'B' , 'C' , 'D'];
let bc = &char[1..=2];
let ab = &chars[0..2];

 // examples of the match guard && slice **Note always match on ""[] => ()"" the empty slices
 let chars = vec![1 , 2 , 3];
 match chars.as_slice() {
     [first, .. , last] => (),
     [single] => (),
     [] => ()
 }

 let chars = vec!['A' , 'B' , 'C' , 'D'];
 match chars.as_slice() {
     [one , two, ..] => (),
     [.. , last] => (),
     [] => ()
 }

 // pattern easily overlap
 match slice {
     [first , ..] => (),
     [.., last] => (), // this will be ignored
     [] => ()
 }

 // always match the largest pattern first, followed by smaller pattern.

 match slice {
     [] => (),
     [a, ..] => (),
     [a, b, ..] => (),
     [a, b, c, ..] => (),

 }
 // only the first two will be matched


 match slice {
    [a, b, c, ..] => (),
    [a, b, ..] => (),
    [a, ..] => (),
    [] => (),
    
}
// alll arms can be matched

//Guards
let nums = vec![7 , 9 , 8];
match nums.as_slice() {
    [first @ 1..=3 , rest @ ..] => {

    },
    [single] if single == &5 || single == &6 => (),
    [a, b] => (),
    [..] => (),
    [] => (),
}
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

 // always match
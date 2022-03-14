// instead of type annotations unknown.

let numbers: Vec<u32> = vec![1, 2, 3, 4, 5];
let numbers: Vec<_> = vec![1, 2, 3, 4, 5];
let odds: Vec<_> = numbers
      .iter()
      .filter(|n| **n % 2 == 1)
      .collect();

// DEMO    ----> ident::<type>
//                   ::<>

let numbers: Vec<_> = vec![1, 2, 3, 4, 5];
let odds: Vec<_> = numbers
      .iter()
      .filter(|n| **n % 2 == 1)
      .collect::<Vec<_>>();
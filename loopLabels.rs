'ident:  loop {}
'ident: for x in y {}
'ident: while true {}


// example with break

let matrix = [
    [1, 5, 7 , 4],
    [9, 4, 5 , 5],
    [21, 45 , 13, 5],
];

'row: for row in matrix.iter() {
    'col: for col in row {
        if col % 2 == 1 {
            println!("odds: {}", col);
            break 'row;
        }
    println!("{}" , col)
    }
}

//example with continue

type UserInput<'a> = Result<&' a str, String>;
'menu: loop {
   println!("menu");
   'input: loop {
       let user_input: UserInput = Ok("next");
       match user_input {
           OK(input) => break 'menu,
           Err(_) => {
               println!("try again");
               continue 'input;
           }
       }
   }

}
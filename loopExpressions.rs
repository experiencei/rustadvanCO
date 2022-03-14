// Can only be used with a loops; NOT for while or for::

let value = 5;
let result: usize = 'ident: loop {
    break value;
    break 'ident value;
}
// result = 5


let value: usize = loop {
    if let Ok(input) = get_input() {
        match input.parse::<usize>() {
            Ok(value) => break value,
            Err(e) => continue
        }
    }
};

// loop labels 
    let nums = vec![1 , 4 , 5 ,];
    let div_by_three: Option<usize> = 'outer: loop {
        for n in nums {
            if n % 3 == 0 {
                break 'outer Some(n);

            }
        }
        break None;
    }


    // breaks loop with a value of zero
    let data = loop {
        break 'label value;
    }
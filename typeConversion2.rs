//TryFrom and TryInto
// they are used when there is posibility of failure
// also call fallible type conversions.

use std::convert::TryFrom;
enum NonZeroError {
    IsZero,
}

struct NonZero(i32);

impl TryFrom<i32> for NonZero {
    type Error = NonZeroError;


    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(NonZeroError::IsZero)
        } else {
            Ok(NonZero(value))
        }
    }
    
}
//USAGE:

struct Nonzero(i32);

use std::convert::{TryFrom , TryInto};

match Nonzero::try_from(9) {
    Ok(nonzero) => println!("not zero"),
    Err(e) => println!("is zero!")
}

let whoops: Result<NonZero, _> = 0_i32::try_into();
match whoops {
    Ok(nonzero) => println!("not zero"),
    Err(e) => println!("is zero!"),
}
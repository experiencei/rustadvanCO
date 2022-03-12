//Literal Numeric Annotaitons

15u8;
-12i16;
999_usize;
13_456_019u32;
17.7f32;

// cast synthax with **as keyword

// always converrt to the same numberic before the conversions

lat a = 15u8 as u16;
let b = a as u8 + 20u8 as u16 as u8;


///Chceked Casting  
u8::try_from(300u16)
// | in the case of the upper one we will get an error.
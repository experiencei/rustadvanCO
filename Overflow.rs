// CHECKED
checked_* Option<i32>;
// None 
let n: Option<u32> = 0u32.checked_sub(1);

// None 
let n: Option<u32> = u32::MAX.checked_add(1);

//Some(10)
let n: Option<u32> = 9_u32.checked_add(1);

overflowing_* (i32 , bool);


// (4232456 , true)
let n: (u32, bool) = 0u32.overflowing_sub(1);

//(6 , false)
let n: (u32, bool) = 5u32.overflowing_add(1);

saturating_* 

//0
let n: u32 = 0_u32.saturating_sub(9001);

// 4236909
let n: u32 = u32::MAX.saturating_add(u32::MAX);

wrappping_* 

//423456789
let n: u32 = 1_u32.wrappping_sub(2);

//0
let n: u32 = u32::MAX.wrapping_add(1);
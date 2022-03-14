let a = 1;
let b = 2;
// assert
assert!(a == b, "{} ne {}", a, b);
assert_eq!(a, b,"values should be equal");
assert_ne!(a, b, "values should not be equal");

debug_bassert!(a == b, "{} ne {}", a, b);
debug_assert_eq!(a, b,"values should be equal");
debug_assert_ne!(a, b, "values should not be equal");

//dbg
#[derive(Debug)]
enum RoomType {
    Bedroom,
    Kitchen,
}

#[derive(Debug)]
struct Room {
    dimensions: (usize , usize),
    Kind: RoomType,
}

let Kitchen = Room {
    dimensions: (20, 20),
    Kind: RoomType::Kitchen,
}

dbg!(&Kitchen);

//format
let h = "Hello";
let w = "World";
let greet: String = format!("{}, {}!", h, w);
println!("{}", greet);

//include_str
msg.txt: this is a message

let msg = include_str!("msg.txt");
println!("{}", msg);

//include_bytes
let bytes  = include_bytes!("image.png");

//env
let config_1 = env!("CONFIG_1");


// TODO
todo!("taking a vacation");

// UNIMPLEMENTED
unimplemented!("nobody want this");

// UNREACHABLE
let number = 12;
let mas_5 = {
    if number > 5 {
        5
    }  else {
        number
    }
};

match max_5 {
    n @ 0..=5 => println!("n = {}", n);
    _ => unreachable!("n > 5. this is a bug")
}
use std::ops::Add;

struct Speed(u32);

impl Add<Self> for Speed {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Speed(self.0 + rhs.0)
    } 
}

let fast = Speed(5) + Speed(3);



//Demo 2

use std::ops::Add;

struct Speed(u32);

impl Add<Self> for Speed {
    type Output = Self;

    fn add(self, rhs: rhs: u32) -> Self::Output {
        Speed(self.0 + rhs.0)
    } 
}

let fast = Speed(5) + 3;

// Demo 3
use std::ops::Add;

struct Letter(char);

impl Add<Self> for Letter {
    type Output = String;

    fn add(self, rhs: Self) -> Self::Output {
        format!("{}", self.0 , rhs.0)
    }
}

println!("{}" , Letter('h') + Letter('i'));


**ops::Add + ;
**ops::Sub - ;
**ops::Mul * ;
**ops::Div / ;
**ops::Rem % ;
**ops::Not ! ;
**ops::Neg - ;


// Demo 3
use::std::ops::Index;

enum Temp {
    Current,
    Max,
    Min,
}

struct Hvac {
    current_temp : i16,
    max_temp : i16,
    min_temp : i16,
}

impl Index<Temp> for Hvac {
    type Output = i16;

    fn index(&self , temp: Temp) -> &Self::Output {
        match temp {
            Temp::Current => &self.current_temp,
            Temp::Max => &self.max_temp,
            Temp::Min => &self.min,
        }
    }
}

let env = Hvac {
    current_temp : 30,
    max_temp : 60,
    min_temp : 0
}

let current = env[Temp::Current];
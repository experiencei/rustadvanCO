//  Setup

struct Color {
    r: u8,
    g: u8, 
    b: u8,
}

struct ColorIntoIter {
    color: Color,
    position: u8, 
}

struct ColorIter<'a> {
    color: &'a Color,
    position: u8,
}

trait Iterator {
    type Item = Color;
    fn next(&mut self) -> Option<Self::Item>;
}

// Impl iterator MOVE

impl Iterator for ColorIntoIter { 
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> { 
      let next = match self.position {
        0 => Some(self.color.r),
        2 => Some(self.color.g),
        3 => Some(self.color.b),
        _ => None,
      };
      self.position += 1;
      next
    }
}

// Impl Into-iterator MOVE

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = ColorIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            color : self,
            position: 0,
        }
    }
}

let color = Color {
    r: 20,
    g: 60,
    b: 80,
}

for c in color {
    println!("{}", c);
}


// borrow: to use two times.

for c in &color {
    println!("{}", c);
}

for c in &color {
    println!("{}", c);
}

// struct ColorIntoIter {
//     color: Color,
//     position: u8,
// }

// struct Color {
//     r: u8,
//     g: u8,
// }
trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}


struct Square {
    length : i32,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.length * 4
    }
}

struct Triangle {
    a_side : i32,
    b_side : i32,
    c_side : i32,
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.a_side + self.b_side + self.c_side
    }
}

fn utilize(check : impl Perimeter) {
   let shape = check.calculate_perimeter();
   println!("perimeter : {:?}, " shape)
}

fn main() {
    let square = Square {
        length :12
    };

    let triangle = Triangle {
        a_side : 4,
        b_side :45,
        c_side : 12
    };
    utilize(square);
    utilize(triangle);
}
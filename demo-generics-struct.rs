struct Dimensions {
    weight: f64,
    height: f64,
    length: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T : Convey> {
    pub items: Vec<T>,
}

impl<T : Convey> ConveyorBelt<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}



struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}


impl Default for CarPart {
    fn default() -> Self {
        Self { width: 5.0,
               height: 1.0,
               depth: 2.0,
               weight: 3.0,
               part_number: "abc".to_owned(),
            }
    }
}

impl Convey for CarPart {
    fn weight(&self) -> f64 { 
        self.weight
    }
    fn dimensions(&self) -> Dimensions {
        Dimensions { 
             width: self.width,
             height: self.height,
             length: self.length,
            }
    }
}

fn main() {
    let mut belt: ConveyorBelt<CarPart> = ConveyorBelt { items: vec![]};
    belt.add(CarPart::default());
}

struct Vehicle<B: Body , C: Color> {
    body: B,
    color: C
}

impl <B: Body , C: Color> Vehicle<B, C> { 
    
}
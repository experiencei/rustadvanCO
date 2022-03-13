// INTOITERATOR TRAIT

trait IntoIterator {
    type Item;
    type IntoIter;
    fn into_iter(self) -> Self::IntoIter;
}

// MOVE
struct Friends {
    names : Vec<String>,
}

impl IntoIterator for Friends {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

for f in friends {
    println!("{:?}", f);
}

// BORROW
struct Friends {
    names : Vec<String>,
}

impl<'a> IntoIterator for &'a Friends {
    type Item =&'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter()
    }
}

for f in &friends {
    println!("{:?}", f);
}

// Mutable BORROW

struct Friends {
    names : Vec<String>,
}

impl<'a> IntoIterator for &'a mut Friends {
    type Item =&'a mut String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}

// for f in &friends {
//     println!("{:?}", f);
// }

let names = vec![
    "albert".to_owned(),
    "sara".to_owned(),
]

let mut friends = Friends{names};

for f in &mut friends {
    *f = "Frank".to_string();
    println!("{:?}", f);
}

// Example implementation.

impl Friends {
    fn iter(&self) -> std::slice::Iter<'_, String> {
        self.into_iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, String> {
        self.into_ier()
    }
}

let total = friends.iter().count();
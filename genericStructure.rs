struct Name<T: Trait1 + Trait2 , U: Trait3> {
    field1: T, 
    field2: U,
}

struct Name<T , U> 
where
    T: Trait1 + Trait2,
    U: Trait3,
{
    field1: T,
    field2: U,
}
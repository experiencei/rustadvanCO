struct Entry {
    id : i32,
}

fn main() {
    let data = Entry { id : 5};
    let data_ptr: Box<Entry> = Box::new(data);
    let data_stack = *data_ptr;
}
// Box is used to reference the heap structure & the * is use to refrences the stack;

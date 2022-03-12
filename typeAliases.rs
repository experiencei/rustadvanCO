type ContactName = String;
type Miles = u64;

type callbacks = HashMap<String, Box<Fn(i32 , i32) -> i32>>;

// usage of type aliases;

struct Contact {
    name : String,
    phone : String,
}

type ContactName = String;
type ContactIndex = Hashmap<ContactName , Contact>;

fn add_contact(index: &mut ContactIndex, contact: Contact) {
    index.insert(contact.name.to_owned(), contact);
}

// can also be use with generic/Lifetimes;

type BorrowedItems<'a> = Vec<&'a str>;
type GenericThings<T> = Vec<GenericThings<T>>;
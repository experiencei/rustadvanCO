use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

struct Counter(usize);

let counter Counter(0);

let shared_counter = Arc::new(Mutex::new(Counter));

let thread_1_counter = Arc::clone(&shared_counter); // we can clone like this 
let thread_2_counter = shared_counter.clone();   // we can clone like this too


let thread_1 = thread::spawn(move || {
    let mut counter = thread_1_counter.lock();
    counter.0 += 1;
});

let thread_2 = thread::spawn(move || {
    let mut counter = thread_2_counter.lock();
    counter.0 += 1;
});

thread_join().and_then(|_| thread_2_counter.join());
println!("{}" , shared_counter.lock().0);

//the result return type is **Arc<Mutex<Counter>>**
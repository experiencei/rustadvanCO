use crossbeam_channel::unbounded;

let (sender , reciever) = unbounded();

sender.send("Hello, Channel!");

match reciever.recv() {
    Ok(msg) => println!("{}", msg),
    Err(e) => println!("{}", e)
}


// demo no2 
use crossbeam_channel::unbounded;
use std::thread;

let (s , r) = unbounded();

let handle = thread::spawn(move || match r.recv() {
    Ok(msg) => println!("Thread {}", msg),
    Err(e) => println!("{:?}", e),
})

s.send("Hello from main!")?;
handle.join();
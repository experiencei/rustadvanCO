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


//multi threading example;
let (s , r1) = unbounded();
let r2 = r1.cloned();

let handle1 = thread::spawn(move || match r1.recv() {
    Ok(msg) => println!("Thread1 {}", msg),
    Err(e) => println!("{:?}", e),
});

let handle2 = thread::spawn(move || match r2.recv() {
    Ok(msg) => println!("Thread1 {}", msg),
    Err(e) => println!("{:?}", e),
});

s.send("Hello from main")?;
s.send("Hello from main")?;
handle1.join();
handle2.join();



// demo 1
use std::thread;

fn main() {
    let iterations = 10;
    let a = thread::spawn(move || {
        for i in 1..=iterations {
            println!("A:{}", i);
        }
    });

    let b = thread::spawn(move || {
        for i in 1..=iterations {
            println!("   B:{}", i);
        }
    })

    a.join();
    b.join();
}

//  Demo 2
use std::thread;
use std::time::Duration;

fn main() {
   let value: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });
    
    println!("waiting on thread");

    match value.join() {
        Ok(n) => println!("value: {:?}", n),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}

// always return JoinHandle<usize> and join spawning before termination from the main

// Demo 3
use std::thread;

fn main() {
    let data = vec!["a" , "b" , "c"];
    let caps = thread::spawn(move || {
        let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
    });

    println!("waiting on value..");

    match caps.join() {
        Ok(n) => println!("value: {:?}", n),
        Err(e) => println!("error: {:?}", e),
    }
}
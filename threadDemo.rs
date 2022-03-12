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
use crossbeam_channel::unbounded;

let (sender , reciever) = unbounded();

sender.send("Hello, Channel!");

match reciever.recv() {
    Ok(msg) => println!("{}", msg),
    Err(e) => println!("{}", e)
}

use crossbeam_channel::unbounded;
use std::thread;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg {
    SumResult(i64),
    WorkerQuit
}

fn main() {
    let (worker_tx , worker_rx) = unbounded();
    let (main_tx , main_rx) = unbounded();

    let worker = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("Worker: {}", d),
                WorkerMsg::Sum(lhs , rhs) => {
                    println!("Worker:  summing...");
                    main_tx.send(MainMsg::SumResult(lhs, rhs));
                    ()
                }
                WorkerMsg::Quit => {
                    println!("thread terminating") ;
                    break ;
                }
            }

            Err(e) => {
                println!("disconnected");
                break ;
            }
        }
    });
 

    s.send(ThreadMsg::PrintData("hello  from main".to_owned()));
    s.send(ThreadMsg::Sum(10, 30));
    s.send(ThreadMsg::Quit);

    drop(s);
    handle.join();
}

// **drop used in order to causes Err as it will terminate the sender.

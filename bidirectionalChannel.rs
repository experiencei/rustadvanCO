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
                    main_tx.send(MainMsg::SumResult(lhs + rhs));
                    ()
                }
                Quit => {
                    println!("thread terminating") ;
                    break ;
                }
            }

            Err(e) => {
                println!("worker : disconnected");
                main_tx.try_send(MainMsg::WorkerQuit)
                break ;
            }
        }
    });
 

    worker_tx.send(ThreadMsg::PrintData("hello  from main".to_owned()));
    worker_tx.send(ThreadMsg::Sum(10, 30));
    worker_tx.send(ThreadMsg::Quit);

  while let Ok(msg) = main_rx.recv() {
      match msg {
          MainMsg::SumResult(answer) => println!("Main: answer = {}", answer),
          MainMsg::WorkerQuit => println!("Main worker terminated")
      }
  }

    // drop(s);
    worker.join();
}

// try_form** is used to avoid error like BLOCKAGE
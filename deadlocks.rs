// deadlock are situations where locks are waiting on one another.

// Recursive deadlock example 

use parking_lot::Mutex;

fn recurse(
    data: Rc<Mutex<u32>>,
    remaining: usize,
) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => 0,
        rem => recurse(Rc::clone(&data) , rem - 1)
    }                          |
}                              |
      //  **here were locking twice            


use parking_lot::ReentrantMutex;

fn recurse(
    data: Rc<ReentrantMutex<u32>>,
    remaining: usize,
) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => 0,
        rem => recurse(Rc::clone(&data) , rem - 1)
    }                          |
}                              |

           //**here it solved with ReentrantMutex allows multiple locks from same thread.

// Threaded Deadlock Example
//before the to.lock() the from lock might not finish loading finish.

type ArcAccount = Arc<Mutex<Account>>;

struct Account {
    balance : i64,
}

fn transfer(from: ArcAccount , to: ArcAccount , amount: i64) {
    let mut from = from.lock();
    let mut to = to.lock();
    from.balance -= amount;
    to.balance += amount;

}

let t1 = thread::spawn(move || {
    transfer(a, b, 500)
})

let t2 = thread::spawn(move || {
    transfer(a, b, 800)
});

// we can resolve this by fix deadlock by **retry on failure** try_lock prevent all locks used in fn and try_again.

fn transfer(from: ArcAccount, to: ArcAccount, amount: i64) {
    loop {
        if let Some(mut from) = from.try_lock() {
            if let Some(mut to) = to.try_lock() {
                from.balance -= amount;
                to.balance += amount;
                return;
            }
        }

        thread::sleep(Duration::from_millis(2))
    }
}

// or resolve with Thread Contention / Backoff
use backoff::ExponentialBackoff;
fn transfer(from: ArcAccount, to: ArcAccount , amount: i64) {
    let op = || {
        if let Some(mut from) = from.try_lock() {
          if let Some(mut to) = to.try_lock() {
              from.balance += amount;
              to.balance += amount;
              return Ok(());
          }
        }
        Err(0)?
    };

    let backoff = ExponentialBackoff::default();
    backoff::retry(backoff , op);
}
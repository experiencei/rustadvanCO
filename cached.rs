use cached::proc_macro::cached;

#[cached] 
fn expensive(n: usize) -> usize {
    thread::sleep(Duration::from_millis(500));
    match n {
        1 => 1,
        2 => 2,
        _ => n
    }
}

// setting cached limit
#[cached(size = 10)] 
fn expensive(n: usize) -> usize {
    thread::sleep(Duration::from_millis(500));
    match n {
        1 => 1,
        2 => 2,
        _ => n
    }
}

// setting Time limit {TTL}
#[cached(size = 10 , time = 30)] 
fn expensive(n: usize) -> usize {
    thread::sleep(Duration::from_millis(500));
    match n {
        1 => 1,
        2 => 2,
        _ => n
    }
}

// only cached success;
#[cached(option = true)]
fn expensive_1(n: usize) -> Option<usize> {

}
#[cached(result = true)]
fn expensive_2(n: usize) -> Result<usize , String> {

}

// custom type fixed Enum must be hashable since it store cached file on the hashmap
#[derive(Clone, Eq, PartialEq, Hash)]
enum Choice {
    A, 
    B,
    C,
}

#[cached]
fn expensive(choice: Choice) -> usize {

}

// ???????????????
// we cant cAched borrowed data so the below won't workers
#[cached]
fn expensive(s: &str) -> &str {
    s
}
// ???????????????????????
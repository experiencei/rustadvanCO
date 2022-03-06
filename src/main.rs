use humantime::format_duration;
use std::time::Duration;

fn main() {
    let d = Duration::from_secs(67890);
    println!("{:?}", format_duration(d)););
}
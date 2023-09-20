use std::time::Duration;
use std::{io, time::SystemTime};
use std::thread::sleep;

fn main() {

let mut input = String::new();
let mut sleep: String = String::new();

println!("How many digits of fib() would you like to calculate?");

io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line!");

let x: u32 = input.trim().parse().expect("Not an int!");

println!("How many ms would you like to sleep between each calculation? (0 for none)");

io::stdin()
    .read_line(&mut sleep)
    .expect("Failed to read line!");

let y: u32 = sleep.trim().parse().expect("Not an int!");

let now = SystemTime::now();
fib(x, y);
let delta = now.elapsed();
println!("{:?}", delta);


}

fn fib(_num: u32, _sleep: u32) {
    let mut sum: u128;
    let mut last = 0;
    let mut curr = 1;

    let milliseconds = 1_000_000;

    if _num == 0 {
        return;
    }

    for _i in 1.._num {
        sum = last + curr;
        last = curr;
        curr = sum;
        print!("{}) {}\n", _i, &sum);
        sleep(Duration::new(0, &_sleep * milliseconds))
    }
}

use std::time::Duration;
use std::time::SystemTime;
use std::thread::sleep;
use std::env;
use thousands::Separable;

fn main() {

let args: Vec<String> = env::args().collect();

let x: u32 = args[1].trim().parse().expect("Not an int!");

let y: u32 = args[2].trim().parse().expect("Not an int!");

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
        print!("{}) {}\n", _i, &sum.separate_with_commas());
        sleep(Duration::new(0, &_sleep * milliseconds))
    }
}

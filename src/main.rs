use std::io;

fn main() {

let mut input = String::new();

println!("How many digits of fib() would you like to calculate?");

io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line!");

let x: u32 = input.trim().parse().expect("Not an int!");

fib(x);


}

fn fib(_num: u32) {
    let mut sum: u128;
    let mut last = 0;
    let mut curr = 1;


    if _num == 0 {
        return;
    }

    for _i in 1.._num {
        sum = last + curr;
        last = curr;
        curr = sum;
        print!("{}\n", &sum);
    }

}

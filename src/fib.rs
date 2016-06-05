extern crate time;

use rayon::*;

fn sfib(n: i32) -> i32 {
    if n < 2 { n }
    else { fib(n - 1) + fib(n - 2) }
}

fn fib(n: i32) -> i32 {
    if n < 2 { n }
    else { 
        let (n1, n2) = join(|| fib(n - 1), || fib(n - 2));
        n1 + n2 
    }
}

pub fn run() {
    for i in 1..45 {
        time!("sfib", { print!("fib({}) = {}, ", i, sfib(i)) });
        time!("fib", { print!("fib({}) = {}, ", i, fib(i)) })
    }
}
extern crate rayon;

#[macro_use] mod macros;
mod fib;

fn main() {
    fib::run()
}

// adapted from https://www.softax.pl/blog/rust-lang-in-a-nutshell-3-traits-and-generics/

use std::env;

struct Fibonacci(u128, u128);

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        let new_next: u128 = self.0 + self.1;
        self.0 = self.1;
        self.1 = new_next;
        Some(self.0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please supply one argument, the Fibonacci sequence number to go to");
    }
    let num = args[1].parse().unwrap();
    let seq = Fibonacci(0, 1);
    let mut i: u32 = 0;
    for n in seq.take(num) {
        i += 1;
        print!("{}: {}\n", i, n);
    }
}

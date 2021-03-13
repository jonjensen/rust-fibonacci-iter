// adapted from https://www.softax.pl/blog/rust-lang-in-a-nutshell-3-traits-and-generics/

use std::env;
use std::process;

extern crate benchmarking;

struct Fibonacci(u32, u128, u128);

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci(0, 0, 1)
    }
}

trait Limit {
    // u128 can't hold Fibonacci sequence numbers higher than this position
    const LIMIT: u32 = 185;
}

impl Limit for Fibonacci {}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        self.0 += 1;
        if self.0 > Self::LIMIT {
            return None;
        }

        let new_next: u128 = self.1 + self.2;
        self.1 = self.2;
        self.2 = new_next;
        Some(self.1)
    }
}

/*
trait Next {
    fn next(&mut self) -> Option<u128>;
}

impl Next for Fibonacci {
    fn next(&mut self) -> Option<u128> { Some(0) }
}
*/

const BENCHMARK_ITERATIONS: u64 = 1_000_000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please supply one argument, the Fibonacci sequence number to go to");
        process::exit(1);
    }
    let num = args[1].parse().unwrap();

    benchmarking::warm_up();

    let bench_result =
        benchmarking::measure_function_with_times(BENCHMARK_ITERATIONS, move |measurer| {
            measurer.measure(|| {
                let seq = Fibonacci::new();
                // Skip printing for the benchmark, to just measure non-I/O time
                seq.take(num).last();
                /*
                let mut i: u32 = 0;
                for i in seq.take(num) {
                    i += 1;
                    println!("{}: {}", i, n);
                }
                */
            });
        })
        .unwrap();

    println!(
        "Grabbing those Fibonacci numbers takes {:?}",
        bench_result.elapsed()
    );
}

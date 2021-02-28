// from https://www.softax.pl/blog/rust-lang-in-a-nutshell-3-traits-and-generics/

struct Fibonacci (u32,u32);

impl Iterator for Fibonacci {
	type Item = u32; //associated Iterator type defined as u32

	fn next(&mut self) -> Option<u32> {
		let new_next = self.0 + self.1;
		self.0 = self.1;
		self.1 = new_next;
		Some(self.0)
	}
}

fn main(){
    let seq = Fibonacci(0,1);
    for i in seq.take(10) {
        print!("{}, ", i);
    }
}

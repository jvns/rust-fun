extern mod extra;
// use std::rt::io::timer::sleep;
use std::str;
use extra::comm::DuplexStream;

fn sqr(x:int) -> int {
	return x*x;
}


fn pmap(fun: extern "Rust" fn(~str) -> uint) {
	let myvect = ["Alex", "Julia", "Yaron"];
	let (child1, parent1) = DuplexStream();
	let (child2, parent2) = DuplexStream();

	do spawn {
		loop {
			let s:~str = child1.recv();
			// child1.send(get_length(s));
			let result = fun(s);
			child1.send(result);
		}
	}

	let fun2 = fun.clone();
	do spawn {
		loop {
			let s:~str = child2.recv();
			// child1.send(get_length(s));
			let result = fun2(s);
			child2.send(result);
		}
	}

	for iptr in myvect.iter() {
		// let s = *iptr;
		let s = (*iptr).to_owned();
		let t = s.clone();
		parent1.send(s);
		parent2.send(t);
		println(fmt!("%u", parent1.recv()));
		println(fmt!("%u", parent2.recv()));
	}
}

fn get_length(x:~str) -> uint {
	return x.len();
}

fn main() {
	// let (child2, parent2) = DuplexStream<int, int>();
	let g = get_length.clone();
	// println(g);
	pmap(get_length);
	// pmap({|s| s.len()})
}


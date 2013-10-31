extern mod extra;
use extra::comm::DuplexStream;

fn sqr(x:int) -> int {
	return x*x;
}


fn pmap(fun: extern fn(~str) -> uint, myvect:&[~str]) {
	let (child1, parent1) = DuplexStream();
	let (child2, parent2) = DuplexStream();

	do spawn {
		loop {
			let s:~str = child1.recv();
			let result = fun(s);
			child1.send(result);
		}
	}

	let fun2 = fun.clone();
	do spawn {
		loop {
			let s:~str = child2.recv();
			let result = fun2(s);
			child2.send(result);
		}
	}

	for iptr in myvect.iter() {
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
	let myvect: &[~str] = &[~"Alex", ~"Julia", ~"Yaron"];
	pmap(get_length, myvect);
}


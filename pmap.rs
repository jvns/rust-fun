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
                        match child1.try_recv() {
			    Some(s) => child1.send(fun(s)),
                            None => break
                        }
		}
	}

	do spawn {
		loop {
                        match child2.try_recv() {
			    Some(s) => child2.send(fun(s)),
                            None => break
                        }
		}
	}

	for iptr in myvect.iter() {
		let s = (*iptr).to_owned();
		let t = s.clone();
		parent1.send(s);
		parent2.send(t);
		println(format!("{:u}", parent1.recv()));
		println(format!("{:u}", parent2.recv()));
	}
}

fn get_length(x:~str) -> uint {
	return x.len();
}

fn main() {
	let myvect: &[~str] = &[~"Alex", ~"Julia", ~"Yaron"];
	pmap(get_length, myvect);
}


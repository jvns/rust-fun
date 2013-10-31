extern mod extra;
extern mod std;
use std::rand;

fn sqr(x:int) -> int {
	return x*x;
}


fn pmap(fun: extern fn(~str) -> uint, myvect:~[~str]) {
        let (parent_port1, child_chan1) = stream();
        let (child_port1, parent_chan1) = stream();
        let (parent_port2, child_chan2) = stream();
        let (child_port2, parent_chan2) = stream();
        
	do spawn {
		loop {
                        match child_port1.try_recv() {
			    Some(s) => child_chan1.send(fun(s)),
                            None => break
                        }
		}
	}

	do spawn {
		loop {
                        match child_port2.try_recv() {
			    Some(s) => child_chan2.send(fun(s)),
                            None => break
                        }
		}
	}

        do spawn {
	for iptr in myvect.iter() {
		let s = (*iptr).to_owned();
		let t = s.clone();
                let i = rand::random::<uint>();
		
                match i % 2 {
                  0 => parent_chan1.send(s),
		  1 => parent_chan2.send(t),
                  j => fail!("{:u} % 2 = {:u}",i,j)
                }
	}
        }

        println("working on parent1");
        loop {
          match parent_port1.try_recv() {
           Some(v) => println(format!("{:u}",v)),
           None => break
          }
        }
        println("working on parent2");
        loop {
          match parent_port2.try_recv() {
           Some(v) => println(format!("{:u}",v)),
           None => break
          }
        }
}

fn get_length(x:~str) -> uint {
	return x.len();
}

fn main() {
	let myvect: ~[~str] = ~[~"Alex", ~"Julia", ~"Yaron"];
	pmap(get_length, myvect);
}


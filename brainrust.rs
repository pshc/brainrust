use std::io;

enum Op {
	Next,
	Prev,
	Incr,
	Decr,
	Dump,
	Read,
	Loop(uint),
	Back(uint),
}

struct State {
	i: uint,
	p: uint,
	input: ~Reader,
	output: ~Writer,
	prog: ~[Op],
	mem: [u8, ..30_000]
}

fn step(st: &mut State) {
	match st.prog[st.i] {
		Next => st.p += 1,
		Prev => st.p -= 1,
		Incr => st.mem[st.p] += 1,
		Decr => st.mem[st.p] -= 1,
		Dump => {
			match st.output.write_u8(st.mem[st.p]) {
				Ok(_) => {},
				Err(e) => fail!(e),
			}
		},
		Read => {
			match st.input.read_u8() {
				Ok(b) => st.mem[st.p] = b,
				Err(e) => fail!(e),
			}
		},
		Loop(i) => {
			if st.mem[st.p] == 0 {
				st.i = i; // jump to end of loop
			}
		},
		Back(i) => {
			if st.mem[st.p] != 0 {
				st.i = i; // jump back to loop start
			}
		},
	}

	// next op
	st.i += 1;
}

fn run(prog: ~[Op]) {
	let n = prog.len();
	let mut state = State {
		i: 0,
		p: 0,
		input: ~io::stdin(),
		output: ~io::stdout(),
		prog: prog,
		mem: [0u8, ..30_000]
	};
	while state.i < n {
		step(&mut state);
	}
}

fn parse(stream: &mut Reader) -> ~[Op] {
	let mut ops: ~[Op] = ~[];
	let mut loopStack: ~[uint] = ~[];
	loop {
		let b = match stream.read_byte() {
			Ok(c) => c,
			Err(e) => {
				match e.kind {
					io::EndOfFile() => break,
					_ => fail!("{}", e)
				}
			}
		};
		let op = match b {
			62 => Next,
			60 => Prev,
			43 => Incr,
			45 => Decr,
			46 => Dump,
			44 => Read,
			91 => {
				loopStack.push(ops.len());
				Loop(0)
			},
			93 => {
				let j = loopStack.pop().expect("unmatched ]");
				ops[j] = Loop(ops.len());
				Back(j)
			},
			_  => continue
		};
		ops.push(op);
	}
	assert!(loopStack.is_empty(), "unmatched [");
	ops
}

fn main() {
	let args = std::os::args();
	if args.len() != 2 {
		println!("Usage: {} <script>", args[0]);
		return;
	}
	let prog = {
		let file = io::File::open(&Path::new(args[1]));
		parse(&mut io::BufferedReader::new(file))
	};
	run(prog);
}

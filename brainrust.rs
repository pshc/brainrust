use std::io;

enum Op {
	Next,
	Prev,
	Incr,
	Decr,
	Dump,
	Read,
	Jump,
	Land,
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
		Jump => {
			if st.mem[st.p] == 0 {
				// advance past loop end (slowly)
				st.i += 1;
				let mut lev = 1;
				while lev > 0 {
					match st.prog[st.i] {
						Jump => lev += 1,
						Land => lev -= 1,
						_    => {}
					}
					st.i += 1;
				}
				return;
			}
		},
		Land => {
			if st.mem[st.p] != 0 {
				// return to loop beginning (slowly)
				st.i -= 1;
				let mut lev = 1;
				while lev > 0 {
					match st.prog[st.i] {
						Jump => lev -= 1,
						Land => lev += 1,
						_    => {}
					}
					st.i -= 1;
				}
				st.i += 2;
				return;
			}
		}
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
			91 => Jump,
			93 => Land,
			_  => continue
		};
		ops.push(op);
	}
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

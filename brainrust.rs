#![allow(unstable)]

use std::old_io as io;
use self::Op::{Next, Prev, Incr, Decr, Dump, Read, Loop, Back};

enum Op {
    Next,
    Prev,
    Incr,
    Decr,
    Dump,
    Read,
    Loop(usize),
    Back(usize),
}

struct State<'a> {
    i: usize,
    p: usize,
    input: &'a mut (Reader + 'a),
    output: &'a mut (Writer + 'a),
    prog: &'a [Op],
    mem: [u8; 30_000],
}

fn step(st: &mut State) {
    match st.prog[st.i] {
        Next => st.p += 1,
        Prev => st.p -= 1,
        Incr => st.mem[st.p] += 1,
        Decr => st.mem[st.p] -= 1,
        Dump => {
            if let Err(e) = st.output.write_u8(st.mem[st.p]) {
                panic!(e);
            }
        }
        Read => {
            match st.input.read_u8() {
                Ok(b) => st.mem[st.p] = b,
                Err(e) => panic!(e),
            }
        }
        Loop(i) => {
            if st.mem[st.p] == 0 {
                st.i = i; // to end of loop
            }
        },
        Back(i) => {
            if st.mem[st.p] != 0 {
                st.i = i; // back to loop start
            }
        }
    }

    // next op
    st.i += 1;
}

fn run(prog: &[Op]) {
    let n = prog.len();
    let mut state = State {
        i: 0,
        p: 0,
        input: &mut io::stdin(),
        output: &mut io::stdout(),
        prog: prog,
        mem: [0u8; 30_000]
    };
    while state.i < n {
        step(&mut state);
    }
}

fn parse(stream: &mut Reader) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    loop {
        let b = match stream.read_byte() {
            Ok(c) => c,
            Err(e) => {
                match e.kind {
                    io::EndOfFile => break,
                    _ => panic!("{}", e)
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
                loop_stack.push(ops.len());
                Loop(0)
            }
            93 => {
                let j = loop_stack.pop().expect("unmatched ]");
                ops[j] = Loop(ops.len());
                Back(j)
            }
            _  => continue
        };
        ops.push(op);
    }
    assert!(loop_stack.is_empty(), "unmatched [");
    ops
}

fn main() {
    let args = std::os::args();
    if args.len() != 2 {
        drop(writeln!(&mut io::stderr(), "Usage: {} <script>", args[0]));
        std::os::set_exit_status(1);
        return;
    }
    let prog = {
        let path = Path::new(args[1].as_bytes());
        let file = io::File::open(&path);
        parse(&mut io::BufferedReader::new(file))
    };
    run(&prog[]);
}

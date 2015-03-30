#![feature(exit_status)]

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use self::Op::{Next, Prev, Incr, Decr, Dump, Eat, Loop, Back};

enum Op {
    Next,
    Prev,
    Incr,
    Decr,
    Dump,
    Eat,
    Loop(usize),
    Back(usize),
}

struct State<'a, R: 'a + Read> {
    i: usize,
    p: usize,
    input: &'a mut io::Bytes<R>,
    output: &'a mut (Write + 'a),
    prog: &'a [Op],
    mem: [u8; 30_000],
}

fn step<R: Read>(st: &mut State<R>) {
    match st.prog[st.i] {
        Next => st.p += 1,
        Prev => st.p -= 1,
        Incr => st.mem[st.p] += 1,
        Decr => st.mem[st.p] -= 1,
        Dump => {
            if let Err(e) = st.output.write_all(&[st.mem[st.p]]) {
                panic!(e);
            }
        }
        Eat => {
            match st.input.next().expect("Unexpected EOF") {
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
        input: &mut io::stdin().bytes(),
        output: &mut io::stdout(),
        prog: prog,
        mem: [0u8; 30_000]
    };
    while state.i < n {
        step(&mut state);
    }
}

fn parse<R: Read>(stream: &mut io::Bytes<R>) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    loop {
        let b = match stream.next() {
            None => break,
            Some(Ok(c)) => c,
            Some(Err(e)) => panic!("{}", e)
        };
        let op = match b {
            62 => Next,
            60 => Prev,
            43 => Incr,
            45 => Decr,
            46 => Dump,
            44 => Eat,
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
    let mut args: Vec<String> = env::args().collect();
    let prog_name = args.remove(0);
    if args.len() != 1 {
        drop(writeln!(&mut io::stderr(), "Usage: {} <script>", prog_name));
        env::set_exit_status(1);
        return;
    }
    let filename = args.remove(0);
    let prog = {
        let file = fs::File::open(&filename).unwrap();
        parse(&mut io::BufReader::new(file).bytes())
    };
    run(&prog);
}

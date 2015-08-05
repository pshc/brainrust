use std::env;
use std::fs;
use std::io::{self, Read, Write};
use self::Op::*;

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

fn step<R: Read>(st: &mut State<R>) -> io::Result<()> {
    match st.prog[st.i] {
        Next => st.p += 1,
        Prev => st.p -= 1,
        Incr => st.mem[st.p] += 1,
        Decr => st.mem[st.p] -= 1,
        Dump => {
            try!(st.output.write_all(&[st.mem[st.p]]))
        }
        Eat => {
            let b = try!(st.input.next().expect("Unexpected EOF"));
            st.mem[st.p] = b;
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

    Ok(())
}

fn run(prog: &[Op]) -> io::Result<()> {
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
        try!(step(&mut state));
    }
    Ok(())
}

fn parse<R: Read>(stream: &mut io::Bytes<R>) -> io::Result<Vec<Op>> {
    let mut ops = vec![];
    let mut loop_stack = vec![];
    loop {
        let b = match stream.next() {
            None => break,
            Some(r) => try!(r),
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
    Ok(ops)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let prog_name = args.remove(0);
    if args.len() != 1 {
        writeln!(&mut io::stderr(), "Usage: {} <script>", prog_name).unwrap();
        panic!("only one argument please");
    }
    let filename = args.remove(0);
    let prog = {
        let file = fs::File::open(&filename).unwrap();
        parse(&mut io::BufReader::new(file).bytes()).unwrap()
    };
    run(&prog).unwrap()
}

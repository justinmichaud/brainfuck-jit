#[derive(Debug)]
enum BFInstr {
    PtrIncr(usize),
    PtrDecr(usize),
    DataIncr(usize),
    DataDecr(usize),
    Output,
    Input,
    BranchZero(usize),
    BranchNZero(usize)
}
use mode_1::BFInstr::*;

fn generate_ir(s: &[u8]) -> Vec<BFInstr> {
    let mut program = Vec::new();

    for cell in s.iter() {
        match *cell as char {
            '>' => match program.last() {
                Some(&PtrIncr(n)) => *program.last_mut().unwrap() = PtrIncr(n+1),
                _ => program.push(PtrIncr(1))
            },
            '<' => match program.last() {
                Some(&PtrDecr(n)) => *program.last_mut().unwrap() = PtrDecr(n+1),
                _ => program.push(PtrDecr(1))
            },
            '+' => match program.last() {
                Some(&DataIncr(n)) => *program.last_mut().unwrap() = DataIncr(n+1),
                _ => program.push(DataIncr(1))
            },
            '-' => match program.last() {
                Some(&DataDecr(n)) => *program.last_mut().unwrap() = DataDecr(n+1),
                _ => program.push(DataDecr(1))
            },
            '.' => program.push(Output),
            ',' => program.push(Input),
            '[' => program.push(BranchZero(0)),
            ']' => program.push(BranchNZero(0)),
            _ => {}
        };
    }

    let mut open_brackets = Vec::new();

    for idx in 0..program.len() {
        match program[idx] {
            BranchZero(_) => open_brackets.push(idx),
            BranchNZero(_) => {
                let matching = open_brackets.pop().unwrap_or_else(|| panic!("Unmatched ] at ir idx {}", idx));
                program[idx] = BranchNZero(matching);
                program[matching] = BranchZero(idx);
            },
            _ => { }
        }
    }

    program
}

pub fn execute<F>(s: &[u8], n: usize, mut get_input: F) -> String
    where F: FnMut() -> u8 {
    let mut cells = Vec::<u8>::with_capacity(n);
    for _ in 0..n { cells.push(0) };
    let mut output = String::new();
    let program = generate_ir(s);

    let mut ptr: usize = 0;
    let mut ip: usize = 0;

    while ip < program.len() {
        match program[ip] {
            PtrIncr(n) => ptr += n,
            PtrDecr(n) => ptr -= n,
            DataIncr(n) => cells[ptr] = cells[ptr].wrapping_add(n as u8),
            DataDecr(n) => cells[ptr] = cells[ptr].wrapping_sub(n as u8),
            Output => output.push((cells[ptr] as u8) as char),
            Input => cells[ptr] = get_input(),
            BranchZero(n) => if cells[ptr] == 0 { ip = n },
            BranchNZero(n) => if cells[ptr] != 0 { ip = n },
        };
        ip += 1
    }

    output
}
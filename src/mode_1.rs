#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum BFInstr {
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

pub fn generate_ir(s: &[u8]) -> Vec<BFInstr> {
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

pub fn execute_with_callback<F,T>(program: Vec<T>, n: usize, mut get_input: F,
                                    instr_callback: fn(&mut usize, &mut usize, &mut Vec<u8>, T,
                                                       &mut F, &mut String) -> ()) -> String
    where F: FnMut() -> u8,
          T: Copy {
    let mut cells = Vec::<u8>::with_capacity(n);
    for _ in 0..n { cells.push(0) };
    let mut output = String::new();

    let mut ptr: usize = 0;
    let mut ip: usize = 0;

    while ip < program.len() {
        let instr = program[ip];
        instr_callback(&mut ptr, &mut ip, &mut cells, instr, &mut get_input, &mut output);
        ip += 1
    }

    output
}

#[inline(always)]
pub fn execute_callback<F>(ptr: &mut usize, ip: &mut usize, cells: &mut Vec<u8>,
                           instr: BFInstr, get_input: &mut F, output: &mut String)
    where F: FnMut() -> u8 {
    match instr {
        PtrIncr(n) => *ptr += n,
        PtrDecr(n) => *ptr -= n,
        DataIncr(n) => cells[*ptr] = cells[*ptr].wrapping_add(n as u8),
        DataDecr(n) => cells[*ptr] = cells[*ptr].wrapping_sub(n as u8),
        Output => output.push((cells[*ptr] as u8) as char),
        Input => cells[*ptr] = get_input(),
        BranchZero(n) => if cells[*ptr] == 0 { *ip = n },
        BranchNZero(n) => if cells[*ptr] != 0 { *ip = n },
    };
}

pub fn execute<F>(s: &[u8], n: usize, get_input: F) -> String
    where F: FnMut() -> u8 {
    execute_with_callback(generate_ir(s), n, get_input,
                          execute_callback)
}
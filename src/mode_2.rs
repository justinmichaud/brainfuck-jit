use mode_1;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum BFInstr2 {
    BFInstr(mode_1::BFInstr),
    MoveDataLoop(i32)
}
use mode_2::BFInstr2::*;

#[inline(always)]
fn mapper(a: BFInstr2) -> Option<mode_1::BFInstr> {
    match a {
        BFInstr(i) => Some(i),
        _ => None
    }
}

#[inline(always)]
fn unmapper(a: mode_1::BFInstr) -> BFInstr2 {
    BFInstr(a)
}

pub fn generate_ir(s: &[u8]) -> Vec<BFInstr2> {
    use mode_1::BFInstr::*;
    let program = mode_1::generate_ir(s);
    let mut program2 = Vec::with_capacity(s.len());

    for i in 0..program.len() {
        match (program.get(i),program.get(i+1),program.get(i+2),
               program.get(i+3),program.get(i+4),program.get(i+5)) {
//            (Some(&BranchZero(n)), Some(&DataDecr(1)), Some(&PtrDecr(p)),
//                Some(&DataIncr(1)), Some(&PtrIncr(q)), Some(&BranchNZero(m)))
//                if n == i + 5 && m == i && p == q => program2.push(MoveDataLoop(-(p as i32))),
            (Some(&i),_,_,_,_,_) => {program2.push(BFInstr(i)); program2.push(BFInstr(DataIncr(0)))},
            _ => panic!()
        }
    }

    mode_1::fix_brackets(program2, mapper, unmapper)
}

#[inline(always)]
pub fn execute_callback<F>(ptr: &mut usize, ip: &mut usize, cells: &mut Vec<u8>,
                           instr: BFInstr2, get_input: &mut F, output: &mut String)
    where F: FnMut() -> u8 {
    match instr {
        BFInstr(i) => mode_1::execute_callback(ptr, ip, cells, i, get_input, output),
        MoveDataLoop(amount) => {
            let new_ip = if amount > 0 {
                (*ip).wrapping_add(amount as usize)
            } else {
                (*ip).wrapping_sub((-amount) as usize)
            };
            cells[new_ip] = cells[*ip];
            *ip = new_ip
        }
    };
}


pub fn execute<F>(s: &[u8], n: usize, get_input: F) -> String
    where F: FnMut() -> u8 {
    mode_1::execute_with_callback(generate_ir(s), n, get_input,
                                  execute_callback)
}
use mode_1;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum BFInstr2 {
    BFInstr(mode_1::BFInstr)
}
use mode_2::BFInstr2::*;

pub fn generate_ir(s: &[u8]) -> Vec<BFInstr2> {
    let program = mode_1::generate_ir(s).iter().map(|i| BFInstr(*i)).collect();

    program
}

#[inline(always)]
pub fn execute_callback<F>(ptr: &mut usize, ip: &mut usize, cells: &mut Vec<u8>,
                           instr: BFInstr2, get_input: &mut F, output: &mut String)
    where F: FnMut() -> u8 {
    match instr {
        BFInstr(i) => mode_1::execute_callback(ptr, ip, cells, i, get_input, output)
    };
}


pub fn execute<F>(s: &[u8], n: usize, get_input: F) -> String
    where F: FnMut() -> u8 {
    mode_1::execute_with_callback(generate_ir(s), n, get_input,
                                  execute_callback)
}
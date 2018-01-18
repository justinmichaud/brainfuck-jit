use mode_0::build_matching_brackets_map;

pub fn execute<F>(s: &[u8], n: usize, mut get_input: F) -> String
    where F: FnMut() -> u8 {
    let mut cells = Vec::<u8>::with_capacity(n);
    for _ in 0..n { cells.push(0) };
    let mut output = String::new();
    let idx_to_matching = build_matching_brackets_map(s);

    output
}
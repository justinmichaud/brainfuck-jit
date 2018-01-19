use std::collections::HashMap;

pub fn build_matching_brackets_map(s: &[u8]) -> HashMap<usize, usize> {
    let mut ip: usize = 0;
    let mut open_brackets = Vec::new();
    let mut idx_to_matching = HashMap::new();

    while ip < s.len() {
        match s[ip] as char {
            '[' => open_brackets.push(ip),
            ']' => {
                let idx = open_brackets.pop().unwrap_or_else(|| panic!("Unmatched ] at {}", ip));
                idx_to_matching.insert(ip, idx);
                idx_to_matching.insert(idx, ip);
            },
            _ => {}
        };
        ip += 1
    }

    idx_to_matching
}

pub fn execute<F>(s: &[u8], n: usize, mut get_input: F) -> String
        where F: FnMut() -> u8 {
    let mut cells = Vec::<u8>::with_capacity(n);
    for _ in 0..n { cells.push(0) };
    let mut ptr: usize = 0;
    let mut ip: usize = 0;
    let mut output = String::new();
    let idx_to_matching = build_matching_brackets_map(s);

    while ip < s.len() {
        match s[ip] as char {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => cells[ptr] = cells[ptr].wrapping_add(1),
            '-' => cells[ptr] = cells[ptr].wrapping_sub(1),
            '.' => output.push((cells[ptr] as u8) as char),
            ',' => cells[ptr] = get_input(),
            '[' => if cells[ptr] == 0 { ip = idx_to_matching[&ip] },
            ']' => if cells[ptr] != 0 { ip = idx_to_matching[&ip] },
            _ => {}
        };
        ip += 1
    }

    output
}
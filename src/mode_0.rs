use std::io::stdin;
use std::io::Read;

pub fn execute(s: &[u8], n: usize) -> String {
    let mut cells = Vec::<usize>::with_capacity(n);
    for _ in 0..n { cells.push(0) };
    let mut ptr: usize = 0;
    let mut ip: usize = 0;
    let mut input: [u8; 1] = [0];
    let mut output = String::new();

    while ip < s.len() {
        match s[ip] as char {
            '>' => ptr = (ptr + 1)%n,
            '<' => if ptr > 0 { ptr -= 1 } else { ptr = n-1 }
            '+' => cells[ptr] = cells[ptr].wrapping_add(1),
            '-' => cells[ptr] = cells[ptr].wrapping_sub(1),
            '.' => output.push((cells[ptr] as u8) as char),
            ',' => { stdin().read_exact(&mut input).unwrap(); cells[ptr] = input[0] as usize },
            '[' => if cells[ptr] == 0 {
                let mut nesting = 0;
                for i in ip..s.len() {
                    if s[i] == ']' as u8 { nesting -= 1 }
                    else if s[i] == '[' as u8 { nesting += 1 };

                    if nesting == 0 { ip = i; break }
                }

                if nesting != 0 { panic!("Unmatched [ at {}, nesting: {}", ip, nesting) };
            },
            ']' => if cells[ptr] != 0 {
                let mut nesting = 0;
                for i in (0..ip+1).rev() {
                    if s[i] == '[' as u8 {
                        nesting -= 1
                    } else if s[i] == ']' as u8 {
                        nesting += 1
                    };

                    if nesting == 0 { ip = i; break }
                }

                if nesting != 0 { panic!("Unmatched ] at {}, nesting: {}", ip, nesting) };
            },
            _ => {}
        };
        ip += 1
    }

    output
}
extern crate clap;

mod mode_0;
mod mode_1;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use std::io::stdin;

fn main() {
    let matches = App::new("Brainfuck JIT example")
        .version("1.0")
        .about("Following https://eli.thegreenplace.net/2017/adventures-in-jit-compilation-part-1-an-interpreter/")
        .arg(Arg::with_name("cells")
            .long("cells")
            .value_name("NUMBER")
            .help("Sets # of memory cells")
            .takes_value(true))
        .arg(Arg::with_name("mode")
            .long("mode")
            .value_name("MODE")
            .help("Sets mode: 0 = Interpreter; 1 = Simple JIT")
            .takes_value(true))
        .arg(Arg::with_name("file")
            .long("file")
            .value_name("FILE")
            .help("Input file")
            .takes_value(true))
        .get_matches();

    let cells = matches.value_of("cells").unwrap_or("30000").parse::<usize>().unwrap();
    let mode = matches.value_of("mode").unwrap_or("0").parse::<usize>().unwrap();
    let file = matches.value_of("file").unwrap();

    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", execute(mode, contents.as_bytes(), cells, get_input));
}

fn get_input() -> u8 {
    let mut input: [u8; 1] = [0];
    stdin().read_exact(&mut input).unwrap();
    input[0] as u8
}

fn execute<F>(mode: usize, contents: &[u8], cells: usize, get_input: F) -> String
        where F: FnMut() -> u8 {
    match mode {
        0 => mode_0::execute(contents, cells, get_input),
        1 => mode_1::execute(contents, cells, get_input),
        _ => panic!("Invalid mode: {}", mode)
    }
}

#[cfg(test)]
mod tests {
    use execute;

    fn progtest(prog: &str, expected: &str, cells: usize, input: &str) {
        for i in 1..2 {
            let mut input_index = 0;
            assert_eq!(execute(i, prog.as_bytes(), cells,
                               || {
                                   input_index += 1;
                                   input.as_bytes()[input_index-1]
                               }), expected)
        }
    }

    fn progtest_default(prog: &str, expected: &str) {
        progtest(prog, expected, 1000, "")
    }

    fn progtest_input(prog: &str, input: &str, expected: &str) {
        progtest(prog, expected, 30000, input)
    }

    #[test]
    fn bf1to5() {
        progtest_default("++++++++ ++++++++ ++++++++ ++++++++ ++++++++ ++++++++     >+++++      [<+.>-]",
            "12345")
    }

    #[test]
    fn reverse() {
        progtest_input("+[->,----------]<[+++++++++++.<]", "54321\n","12345")
    }

    #[test]
    fn reverse2() {
        progtest_input("+[->,----------]<[+++++++++++.<]", "hello world\n","dlrow olleh")
    }

    #[test]
    fn helloworld() {
        progtest_default("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.",
                         "Hello World!\n")
    }

    #[test]
    fn fib() {
        progtest_default("+++++++++++
>+>>>>++++++++++++++++++++++++++++++++++++++++++++
>++++++++++++++++++++++++++++++++<<<<<<[>[>>>>>>+>
+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<[>++++++++++[-
<-[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]>[<<[>>>+<<<
-]>>[-]]<<]>>>[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]
>[<<+>>[-]]<<<<<<<]>>>>>[+++++++++++++++++++++++++
+++++++++++++++++++++++.[-]]++++++++++<[->-<]>++++
++++++++++++++++++++++++++++++++++++++++++++.[-]<<
<<<<<<<<<<[>>>+>+<<<<-]>>>>[<<<<+>>>>-]<-[>>.>.<<<
[-]]<<[>>+>+<<<-]>>>[<<<+>>>-]<<[<+>-]>[<+>-]<<<-]",
                         "1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89")
    }

    #[test]
    fn helloworldfromhell() {
        progtest_default(r#"
[
    This routine is a demonstration of checking for the three cell sizes
    that are normal for Brainfuck. The demo code also checks for bugs
    that have been noted in various interpreters and compilers.

    It should print one of three slight variations of "Hello world" followed
    by an exclamation point then the maximum cell value (if it's less than a
    few thousand) and a newline.

    If the interpreter is broken in some way it can print a lot of other
    different strings and frequently causes the interpreter to crash.

    It does work correctly with 'bignum' cells.
]
>>

	This code runs at pointer offset two and unknown bit width; don't
	assume you have more that eight bits

	======= DEMO CODE =======
	First just print "Hello"

	Notice that I reset the cells despite knowing that they are zero
	this is a test for proper functioning of the ability to skip over
	a loop that's never executed but isn't actually a comment loop

	Secondly there's a NOP movement between the two 'l' characters

	Also there's some commented out code afterwards

	>[-]<[-]++++++++[->+++++++++<]>.----[--<+++>]<-.+++++++.><.+++.
	[-][[-]>[-]+++++++++[<+++++>-]<+...--------------.>++++++++++[<+
	++++>-]<.+++.-------.>+++++++++[<----->-]<.-.>++++++++[<+++++++>
	-]<++.-----------.--.-----------.+++++++.----.++++++++++++++.>++
	++++++++[<----->-]<..[-]++++++++++.[-]+++++++[.,]-]

	===== END DEMO CODE =====
<<

Calculate the value 256 and test if it's zero
If the interpreter errors on overflow this is where it'll happen
++++++++[>++++++++<-]>[<++++>-]
+<[>-<
Multiply by 256 again to get 65536
[>++++<-]>[<++++++++>-]<[>++++++++<-]
+>[>
	Cells should be 32bits at this point

	The pointer is at cell two and you can continue your code confident
	that there are big cells

	======= DEMO CODE =======
	This code rechecks that the test cells are in fact nonzero
	If the compiler notices the above is constant but doesn't
	properly wrap the values this will generate an incorrect
	string

	An optimisation barrier; unbalanced loops aren't easy
	>+[<]>-<

	Print a message
	++>[-]++++++[<+++++++>-]<.------------.[-]
	<[>+<[-]]>
	++++++++>[-]++++++++++[<+++++++++++>-]<.--------.+++.------.
	--------.[-]

	===== END DEMO CODE =====

<[-]<[-]>] <[>>
	Cells should be 16bits at this point

	The pointer is at cell two and you can continue your code confident
	that there are medium sized cells; you can use all the cells on the
	tape but it is recommended that you leave the first two alone

	If you need 32bit cells you'll have to use a BF doubler

	======= DEMO CODE =======
	Space
	++>[-]+++++[<++++++>-]<.[-]

	I'm rechecking that the cells are 16 bits
	this condition should always be true

	+>>++++[-<<[->++++<]>[-<+>]>]< + <[ >>

	    Print a message
	    >[-]++++++++++[<+++++++++++>-]<+++++++++.--------.
	    +++.------.--------.[-]

	<[-]<[-] ] >[> > Dead code here
	    This should never be executed because it's in an 8bit zone hidden
	    within a 16bit zone; a really good compiler should delete this
	    If you see this message you have dead code walking

	    Print a message
	    [-]>[-]+++++++++[<++++++++++>-]<.
	    >++++[<+++++>-]<+.--.-----------.+++++++.----.
	    [-]

	<<[-]]<
	===== END DEMO CODE =====

<<[-]] >[-]< ] >[>
	Cells should be 8bits at this point

	The pointer is at cell two but you only have 8 bits cells
	and it's time to use the really big and slow BF quad encoding

	======= DEMO CODE =======

	A broken wrapping check
	+++++[>++++<-]>[<+++++++++++++>-]<----[[-]>[-]+++++[<++++++>-]<++.
	>+++++[<+++++++>-]<.>++++++[<+++++++>-]<+++++.>++++[<---->-]<-.++.
	++++++++.------.-.[-]]

	Space
	++>[-]+++++[<++++++>-]<.[-]

	An exponent checker for github user btzy
	>++[>++<-]>[<<+>>[-<<[>++++<-]>[<++++>-]>]]<<[>++++[>---<++++]>++.
	[<++>+]<.[>+<------]>.+++.[<--->++]<--.[-]<[-]]

        Another dead code check
        [-]>[-]>[-]<++[>++++++++<-]>[<++++++++>-]<[>++++++++<-]>[<++++++++>-
        ]<[<++++++++>-]<[[-]>[-]+++++++++[<++++++++++>-]<.>++++[<+++++>-]<+.
        --.-----------.+++++++.----.>>[-]<+++++[>++++++<-]>++.<<[-]]

	Print a message
	[-] <[>+<[-]]> +++++>[-]+++++++++[<+++++++++>-]<.
	>++++[<++++++>-]<.+++.------.--------.
	[-]
	===== END DEMO CODE =====

<[-]]<

+[[>]<-]    Check unbalanced loops are ok

>>
	======= DEMO CODE =======
	Back out and print the last two characters

	[<[[<[[<[[<[,]]]<]<]<]<][ Deep nesting non-comment comment loop ]]

	Check that an offset of 128 will work
	+>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>-[+<-]

	And inside a loop
	--[>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>++<<<
	<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+]+>----[++
	++>----]-[+<-]

	This is a simple multiply loop that looks like it goes off the
	start of the tape
	+[>]<- [-
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    ++++
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	]

	[ Check there are enough cells. This takes 18569597 steps. ]
	[
	    >++++++[<+++>-]<+[>+++++++++<-]>+[[->+>+<<]>>
	    [-<<+>>]<[<[->>+<<]+>[->>+<<]+[>]<-]<-]<[-<]
	]

	This loop is a bug check for handling of nested loops; it goes
	round the outer loop twice and the inner loop is skipped on the
	first pass but run on the second

	BTW: It's unlikely that an optimiser will notice how this works

	>
	    +[>[
		Print the exclamation point
		[-]+++>
		[-]+++++ +#-
		[<+++2+++>-]<
		.

	    <[-]>[-]]+<]
	<

	Clean up any debris
	++++++++[[>]+[<]>-]>[>]<[[-]<]

	This is a hard optimisation barrier
	It contains several difficult to 'prove' constructions close together
	and is likely to prevent almost all forms of optimisation
	+[[>]<-[,]+[>]<-]

	This part finds the actual value that the cell wraps at; even
	if it's not one of the standard ones; but it gets bored after
	a few thousand: any higher and we print nothing

	This has a reasonably deep nested loop and a couple of loops
	that have unbalanced pointer movements

	Find maxint (if small)
	[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<++++[->>++++>>++++>>++
	++<<<<<<]++++++++++++++>>>>+>>++<<<<<<[->>[->+>[->+>[->+>+[>
	>>+<<]>>[-<<+>]<-[<<<[-]<<[-]<<[-]<<[-]>>>[-]>>[-]>>[-]>->+]
	<<<]>[-<+>]<<<]>[-<+>]<<<]>[-<+>]<<<]>+>[[-]<->]<[->>>>>>>[-
	<<<<<<<<+>>>>>>>>]<<<<<<<]<

	The number is only printed if we found the actual maxint
	>+<[
	    Space
	    >[-]>[-]+++++[<++++++>-]<++.[-]<

	    Print the number
	    [[->>+<<]>>[-<++>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[<[-]+>
	    ->+<[<-]]]]]]]]]]>]<<[>++++++[<++++++++>-]<-.[-]<]]

	]

	Check if we should have had a value but didn't
	>[
	    >[-]>[-]++++[<++++++++>-]<[<++++++++>-]>+++[<++++++++>-]<+++++++
	    [<-------->-]<------->+<[[-]>-<]>[>[-]<[-]++++[->++++++++<]>.+++
	    +++[-<++>]<.[-->+++<]>++.<++++[>----<-]>.[-]<]<

	    [-]>[-]++++++++[<++++++++>-]<[>++++<-]+>[<->[-]]<[>[-]<[-]++++[-
	    >++++++++<]>.---[-<+++>]<.---.--------------.[-->+<]>--.[-]<]
	]<

	Clean up any debris
	++++++++[[>]+[<]>-]>[>]<[[-]<]

	One last thing: an exclamation point is not a valid BF instruction!

	Print the newline
	[-]++++++++++.[-]
	[
	    Oh, and now that I can use "!" the string you see should be one of:
	    Hello World! 255
	    Hello world! 65535
	    Hello, world!

	    And it should be followed by a newline.
	]

	===== END DEMO CODE =====

<<  Finish at cell zero"#, "Hello World! 255\n")
    }

    #[test]
    fn quine() {
        let program = r">+++++>+++>+++>+++++>+++>+++>+++++>++++++>+>++>+++>++++>++++>+++>+++>+++++>+>+>++++>+++++++>+>+++++>+>+>+++++>++++++>+++>+++>++>+>+>++++>++++++>++++>++++>+++>+++++>+++>+++>++++>++>+>+>+>+>++>++>++>+>+>++>+>+>++++++>++++++>+>+>++++++>++++++>+>+>+>+++++>++++++>+>+++++>+++>+++>++++>++>+>+>++>+>+>++>++>+>+>++>++>+>+>+>+>++>+>+>+>++++>++>++>+>+++++>++++++>+++>+++>+++>+++>+++>+++>++>+>+>+>+>++>+>+>++++>+++>+++>+++>+++++>+>+++++>++++++>+>+>+>++>+++>+++>+++++++>+++>++++>+>++>+>+++++++>++++++>+>+++++>++++++>+++>+++>++>++>++>++>++>++>+>++>++>++>++>++>++>++>++>++>+>++++>++>++>++>++>++>++>++>+++++>++++++>++++>+++>+++++>++++++>++++>+++>+++>++++>+>+>+>+>+++++>+++>+++++>++++++>+++>+++>+++>++>+>+>+>++++>++++[[>>>+<<<-]<]>>>>[<<[-]<[-]+++++++[>+++++++++>++++++<<-]>-.>+>[<.<<+>>>-]>]<<<[>>+>>>>+<<<<<<-]>++[>>>+>>>>++>>++>>+>>+[<<]>-]>>>-->>-->>+>>+++>>>>+[<<]<[[-[>>+<<-]>>]>.[>>]<<[[<+>-]<<]<<]";
        progtest_default(program, program)
    }

    #[test]
    fn mandlelbrot() {
        progtest_default("[
    A mandelbrot set fractal viewer in brainf*** written by Erik Bosman
    Taken from https://github.com/pablojorge/brainfuck
]

+++++++++++++[->++>>>+++++>++>+<<<<<<]>>>>>++++++>--->>>>>>>>>>+++++++++++++++[[
>>>>>>>>>]+[<<<<<<<<<]>>>>>>>>>-]+[>>>>>>>>[-]>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>[-]+
<<<<<<<+++++[-[->>>>>>>>>+<<<<<<<<<]>>>>>>>>>]>>>>>>>+>>>>>>>>>>>>>>>>>>>>>>>>>>
>+<<<<<<<<<<<<<<<<<[<<<<<<<<<]>>>[-]+[>>>>>>[>>>>>>>[-]>>]<<<<<<<<<[<<<<<<<<<]>>
>>>>>[-]+<<<<<<++++[-[->>>>>>>>>+<<<<<<<<<]>>>>>>>>>]>>>>>>+<<<<<<+++++++[-[->>>
>>>>>>+<<<<<<<<<]>>>>>>>>>]>>>>>>+<<<<<<<<<<<<<<<<[<<<<<<<<<]>>>[[-]>>>>>>[>>>>>
>>[-<<<<<<+>>>>>>]<<<<<<[->>>>>>+<<+<<<+<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>
[>>>>>>>>[-<<<<<<<+>>>>>>>]<<<<<<<[->>>>>>>+<<+<<<+<<]>>>>>>>>]<<<<<<<<<[<<<<<<<
<<]>>>>>>>[-<<<<<<<+>>>>>>>]<<<<<<<[->>>>>>>+<<+<<<<<]>>>>>>>>>+++++++++++++++[[
>>>>>>>>>]+>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>-]+[
>+>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>->>>>[-<<<<+>>>>]<<<<[->>>>+<<<<<[->>[
-<<+>>]<<[->>+>>+<<<<]+>>>>>>>>>]<<<<<<<<[<<<<<<<<<]]>>>>>>>>>[>>>>>>>>>]<<<<<<<
<<[>[->>>>>>>>>+<<<<<<<<<]<<<<<<<<<<]>[->>>>>>>>>+<<<<<<<<<]<+>>>>>>>>]<<<<<<<<<
[>[-]<->>>>[-<<<<+>[<->-<<<<<<+>>>>>>]<[->+<]>>>>]<<<[->>>+<<<]<+<<<<<<<<<]>>>>>
>>>>[>+>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>->>>>>[-<<<<<+>>>>>]<<<<<[->>>>>+
<<<<<<[->>>[-<<<+>>>]<<<[->>>+>+<<<<]+>>>>>>>>>]<<<<<<<<[<<<<<<<<<]]>>>>>>>>>[>>
>>>>>>>]<<<<<<<<<[>>[->>>>>>>>>+<<<<<<<<<]<<<<<<<<<<<]>>[->>>>>>>>>+<<<<<<<<<]<<
+>>>>>>>>]<<<<<<<<<[>[-]<->>>>[-<<<<+>[<->-<<<<<<+>>>>>>]<[->+<]>>>>]<<<[->>>+<<
<]<+<<<<<<<<<]>>>>>>>>>[>>>>[-<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>
>>>>>>>>>>>>>>>>>>>>>>>]>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>+++++++++++++++[[>>>>
>>>>>]<<<<<<<<<-<<<<<<<<<[<<<<<<<<<]>>>>>>>>>-]+>>>>>>>>>>>>>>>>>>>>>+<<<[<<<<<<
<<<]>>>>>>>>>[>>>[-<<<->>>]+<<<[->>>->[-<<<<+>>>>]<<<<[->>>>+<<<<<<<<<<<<<[<<<<<
<<<<]>>>>[-]+>>>>>[>>>>>>>>>]>+<]]+>>>>[-<<<<->>>>]+<<<<[->>>>-<[-<<<+>>>]<<<[->
>>+<<<<<<<<<<<<[<<<<<<<<<]>>>[-]+>>>>>>[>>>>>>>>>]>[-]+<]]+>[-<[>>>>>>>>>]<<<<<<
<<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]<<<<<<<[->+>>>-<<<<]>>>>>>>>>+++++++++++++++++++
+++++++>>[-<<<<+>>>>]<<<<[->>>>+<<[-]<<]>>[<<<<<<<+<[-<+>>>>+<<[-]]>[-<<[->+>>>-
<<<<]>>>]>>>>>>>>>>>>>[>>[-]>[-]>[-]>>>>>]<<<<<<<<<[<<<<<<<<<]>>>[-]>>>>>>[>>>>>
[-<<<<+>>>>]<<<<[->>>>+<<<+<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>>[-<<<<<<<<
<+>>>>>>>>>]>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>+++++++++++++++[[>>>>>>>>>]+>[-
]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>-]+[>+>>>>>>>>]<<<
<<<<<<[<<<<<<<<<]>>>>>>>>>[>->>>>>[-<<<<<+>>>>>]<<<<<[->>>>>+<<<<<<[->>[-<<+>>]<
<[->>+>+<<<]+>>>>>>>>>]<<<<<<<<[<<<<<<<<<]]>>>>>>>>>[>>>>>>>>>]<<<<<<<<<[>[->>>>
>>>>>+<<<<<<<<<]<<<<<<<<<<]>[->>>>>>>>>+<<<<<<<<<]<+>>>>>>>>]<<<<<<<<<[>[-]<->>>
[-<<<+>[<->-<<<<<<<+>>>>>>>]<[->+<]>>>]<<[->>+<<]<+<<<<<<<<<]>>>>>>>>>[>>>>>>[-<
<<<<+>>>>>]<<<<<[->>>>>+<<<<+<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>+>>>>>>>>
]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>->>>>>[-<<<<<+>>>>>]<<<<<[->>>>>+<<<<<<[->>[-<<+
>>]<<[->>+>>+<<<<]+>>>>>>>>>]<<<<<<<<[<<<<<<<<<]]>>>>>>>>>[>>>>>>>>>]<<<<<<<<<[>
[->>>>>>>>>+<<<<<<<<<]<<<<<<<<<<]>[->>>>>>>>>+<<<<<<<<<]<+>>>>>>>>]<<<<<<<<<[>[-
]<->>>>[-<<<<+>[<->-<<<<<<+>>>>>>]<[->+<]>>>>]<<<[->>>+<<<]<+<<<<<<<<<]>>>>>>>>>
[>>>>[-<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
]>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>>>[-<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>]>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>++++++++
+++++++[[>>>>>>>>>]<<<<<<<<<-<<<<<<<<<[<<<<<<<<<]>>>>>>>>>-]+[>>>>>>>>[-<<<<<<<+
>>>>>>>]<<<<<<<[->>>>>>>+<<<<<<+<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>>>>>>[
-]>>>]<<<<<<<<<[<<<<<<<<<]>>>>+>[-<-<<<<+>>>>>]>[-<<<<<<[->>>>>+<++<<<<]>>>>>[-<
<<<<+>>>>>]<->+>]<[->+<]<<<<<[->>>>>+<<<<<]>>>>>>[-]<<<<<<+>>>>[-<<<<->>>>]+<<<<
[->>>>->>>>>[>>[-<<->>]+<<[->>->[-<<<+>>>]<<<[->>>+<<<<<<<<<<<<[<<<<<<<<<]>>>[-]
+>>>>>>[>>>>>>>>>]>+<]]+>>>[-<<<->>>]+<<<[->>>-<[-<<+>>]<<[->>+<<<<<<<<<<<[<<<<<
<<<<]>>>>[-]+>>>>>[>>>>>>>>>]>[-]+<]]+>[-<[>>>>>>>>>]<<<<<<<<]>>>>>>>>]<<<<<<<<<
[<<<<<<<<<]>>>>[-<<<<+>>>>]<<<<[->>>>+>>>>>[>+>>[-<<->>]<<[->>+<<]>>>>>>>>]<<<<<
<<<+<[>[->>>>>+<<<<[->>>>-<<<<<<<<<<<<<<+>>>>>>>>>>>[->>>+<<<]<]>[->>>-<<<<<<<<<
<<<<<+>>>>>>>>>>>]<<]>[->>>>+<<<[->>>-<<<<<<<<<<<<<<+>>>>>>>>>>>]<]>[->>>+<<<]<<
<<<<<<<<<<]>>>>[-]<<<<]>>>[-<<<+>>>]<<<[->>>+>>>>>>[>+>[-<->]<[->+<]>>>>>>>>]<<<
<<<<<+<[>[->>>>>+<<<[->>>-<<<<<<<<<<<<<<+>>>>>>>>>>[->>>>+<<<<]>]<[->>>>-<<<<<<<
<<<<<<<+>>>>>>>>>>]<]>>[->>>+<<<<[->>>>-<<<<<<<<<<<<<<+>>>>>>>>>>]>]<[->>>>+<<<<
]<<<<<<<<<<<]>>>>>>+<<<<<<]]>>>>[-<<<<+>>>>]<<<<[->>>>+>>>>>[>>>>>>>>>]<<<<<<<<<
[>[->>>>>+<<<<[->>>>-<<<<<<<<<<<<<<+>>>>>>>>>>>[->>>+<<<]<]>[->>>-<<<<<<<<<<<<<<
+>>>>>>>>>>>]<<]>[->>>>+<<<[->>>-<<<<<<<<<<<<<<+>>>>>>>>>>>]<]>[->>>+<<<]<<<<<<<
<<<<<]]>[-]>>[-]>[-]>>>>>[>>[-]>[-]>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>>>>>[-<
<<<+>>>>]<<<<[->>>>+<<<+<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>+++++++++++++++[
[>>>>>>>>>]+>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>-]+
[>+>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>->>>>[-<<<<+>>>>]<<<<[->>>>+<<<<<[->>
[-<<+>>]<<[->>+>+<<<]+>>>>>>>>>]<<<<<<<<[<<<<<<<<<]]>>>>>>>>>[>>>>>>>>>]<<<<<<<<
<[>[->>>>>>>>>+<<<<<<<<<]<<<<<<<<<<]>[->>>>>>>>>+<<<<<<<<<]<+>>>>>>>>]<<<<<<<<<[
>[-]<->>>[-<<<+>[<->-<<<<<<<+>>>>>>>]<[->+<]>>>]<<[->>+<<]<+<<<<<<<<<]>>>>>>>>>[
>>>[-<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>]>
>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>[-]>>>>+++++++++++++++[[>>>>>>>>>]<<<<<<<<<-<<<<<
<<<<[<<<<<<<<<]>>>>>>>>>-]+[>>>[-<<<->>>]+<<<[->>>->[-<<<<+>>>>]<<<<[->>>>+<<<<<
<<<<<<<<[<<<<<<<<<]>>>>[-]+>>>>>[>>>>>>>>>]>+<]]+>>>>[-<<<<->>>>]+<<<<[->>>>-<[-
<<<+>>>]<<<[->>>+<<<<<<<<<<<<[<<<<<<<<<]>>>[-]+>>>>>>[>>>>>>>>>]>[-]+<]]+>[-<[>>
>>>>>>>]<<<<<<<<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>[-<<<+>>>]<<<[->>>+>>>>>>[>+>>>
[-<<<->>>]<<<[->>>+<<<]>>>>>>>>]<<<<<<<<+<[>[->+>[-<-<<<<<<<<<<+>>>>>>>>>>>>[-<<
+>>]<]>[-<<-<<<<<<<<<<+>>>>>>>>>>>>]<<<]>>[-<+>>[-<<-<<<<<<<<<<+>>>>>>>>>>>>]<]>
[-<<+>>]<<<<<<<<<<<<<]]>>>>[-<<<<+>>>>]<<<<[->>>>+>>>>>[>+>>[-<<->>]<<[->>+<<]>>
>>>>>>]<<<<<<<<+<[>[->+>>[-<<-<<<<<<<<<<+>>>>>>>>>>>[-<+>]>]<[-<-<<<<<<<<<<+>>>>
>>>>>>>]<<]>>>[-<<+>[-<-<<<<<<<<<<+>>>>>>>>>>>]>]<[-<+>]<<<<<<<<<<<<]>>>>>+<<<<<
]>>>>>>>>>[>>>[-]>[-]>[-]>>>>]<<<<<<<<<[<<<<<<<<<]>>>[-]>[-]>>>>>[>>>>>>>[-<<<<<
<+>>>>>>]<<<<<<[->>>>>>+<<<<+<<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>+>[-<-<<<<+>>>>
>]>>[-<<<<<<<[->>>>>+<++<<<<]>>>>>[-<<<<<+>>>>>]<->+>>]<<[->>+<<]<<<<<[->>>>>+<<
<<<]+>>>>[-<<<<->>>>]+<<<<[->>>>->>>>>[>>>[-<<<->>>]+<<<[->>>-<[-<<+>>]<<[->>+<<
<<<<<<<<<[<<<<<<<<<]>>>>[-]+>>>>>[>>>>>>>>>]>+<]]+>>[-<<->>]+<<[->>->[-<<<+>>>]<
<<[->>>+<<<<<<<<<<<<[<<<<<<<<<]>>>[-]+>>>>>>[>>>>>>>>>]>[-]+<]]+>[-<[>>>>>>>>>]<
<<<<<<<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>[-<<<+>>>]<<<[->>>+>>>>>>[>+>[-<->]<[->+
<]>>>>>>>>]<<<<<<<<+<[>[->>>>+<<[->>-<<<<<<<<<<<<<+>>>>>>>>>>[->>>+<<<]>]<[->>>-
<<<<<<<<<<<<<+>>>>>>>>>>]<]>>[->>+<<<[->>>-<<<<<<<<<<<<<+>>>>>>>>>>]>]<[->>>+<<<
]<<<<<<<<<<<]>>>>>[-]>>[-<<<<<<<+>>>>>>>]<<<<<<<[->>>>>>>+<<+<<<<<]]>>>>[-<<<<+>
>>>]<<<<[->>>>+>>>>>[>+>>[-<<->>]<<[->>+<<]>>>>>>>>]<<<<<<<<+<[>[->>>>+<<<[->>>-
<<<<<<<<<<<<<+>>>>>>>>>>>[->>+<<]<]>[->>-<<<<<<<<<<<<<+>>>>>>>>>>>]<<]>[->>>+<<[
->>-<<<<<<<<<<<<<+>>>>>>>>>>>]<]>[->>+<<]<<<<<<<<<<<<]]>>>>[-]<<<<]>>>>[-<<<<+>>
>>]<<<<[->>>>+>[-]>>[-<<<<<<<+>>>>>>>]<<<<<<<[->>>>>>>+<<+<<<<<]>>>>>>>>>[>>>>>>
>>>]<<<<<<<<<[>[->>>>+<<<[->>>-<<<<<<<<<<<<<+>>>>>>>>>>>[->>+<<]<]>[->>-<<<<<<<<
<<<<<+>>>>>>>>>>>]<<]>[->>>+<<[->>-<<<<<<<<<<<<<+>>>>>>>>>>>]<]>[->>+<<]<<<<<<<<
<<<<]]>>>>>>>>>[>>[-]>[-]>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>[-]>[-]>>>>>[>>>>>[-<<<<+
>>>>]<<<<[->>>>+<<<+<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>>>>>>[-<<<<<+>>>>>
]<<<<<[->>>>>+<<<+<<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>+++++++++++++++[[>>>>
>>>>>]+>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>-]+[>+>>
>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>->>>>[-<<<<+>>>>]<<<<[->>>>+<<<<<[->>[-<<+
>>]<<[->>+>>+<<<<]+>>>>>>>>>]<<<<<<<<[<<<<<<<<<]]>>>>>>>>>[>>>>>>>>>]<<<<<<<<<[>
[->>>>>>>>>+<<<<<<<<<]<<<<<<<<<<]>[->>>>>>>>>+<<<<<<<<<]<+>>>>>>>>]<<<<<<<<<[>[-
]<->>>>[-<<<<+>[<->-<<<<<<+>>>>>>]<[->+<]>>>>]<<<[->>>+<<<]<+<<<<<<<<<]>>>>>>>>>
[>+>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>->>>>>[-<<<<<+>>>>>]<<<<<[->>>>>+<<<<
<<[->>>[-<<<+>>>]<<<[->>>+>+<<<<]+>>>>>>>>>]<<<<<<<<[<<<<<<<<<]]>>>>>>>>>[>>>>>>
>>>]<<<<<<<<<[>>[->>>>>>>>>+<<<<<<<<<]<<<<<<<<<<<]>>[->>>>>>>>>+<<<<<<<<<]<<+>>>
>>>>>]<<<<<<<<<[>[-]<->>>>[-<<<<+>[<->-<<<<<<+>>>>>>]<[->+<]>>>>]<<<[->>>+<<<]<+
<<<<<<<<<]>>>>>>>>>[>>>>[-<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>
>>>>>>>>>>>>>>>>>>>]>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>+++++++++++++++[[>>>>>>>>
>]<<<<<<<<<-<<<<<<<<<[<<<<<<<<<]>>>>>>>>>-]+>>>>>>>>>>>>>>>>>>>>>+<<<[<<<<<<<<<]
>>>>>>>>>[>>>[-<<<->>>]+<<<[->>>->[-<<<<+>>>>]<<<<[->>>>+<<<<<<<<<<<<<[<<<<<<<<<
]>>>>[-]+>>>>>[>>>>>>>>>]>+<]]+>>>>[-<<<<->>>>]+<<<<[->>>>-<[-<<<+>>>]<<<[->>>+<
<<<<<<<<<<<[<<<<<<<<<]>>>[-]+>>>>>>[>>>>>>>>>]>[-]+<]]+>[-<[>>>>>>>>>]<<<<<<<<]>
>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>->>[-<<<<+>>>>]<<<<[->>>>+<<[-]<<]>>]<<+>>>>[-<<<<
->>>>]+<<<<[->>>>-<<<<<<.>>]>>>>[-<<<<<<<.>>>>>>>]<<<[-]>[-]>[-]>[-]>[-]>[-]>>>[
>[-]>[-]>[-]>[-]>[-]>[-]>>>]<<<<<<<<<[<<<<<<<<<]>>>>>>>>>[>>>>>[-]>>>>]<<<<<<<<<
[<<<<<<<<<]>+++++++++++[-[->>>>>>>>>+<<<<<<<<<]>>>>>>>>>]>>>>+>>>>>>>>>+<<<<<<<<
<<<<<<[<<<<<<<<<]>>>>>>>[-<<<<<<<+>>>>>>>]<<<<<<<[->>>>>>>+[-]>>[>>>>>>>>>]<<<<<
<<<<[>>>>>>>[-<<<<<<+>>>>>>]<<<<<<[->>>>>>+<<<<<<<[<<<<<<<<<]>>>>>>>[-]+>>>]<<<<
<<<<<<]]>>>>>>>[-<<<<<<<+>>>>>>>]<<<<<<<[->>>>>>>+>>[>+>>>>[-<<<<->>>>]<<<<[->>>
>+<<<<]>>>>>>>>]<<+<<<<<<<[>>>>>[->>+<<]<<<<<<<<<<<<<<]>>>>>>>>>[>>>>>>>>>]<<<<<
<<<<[>[-]<->>>>>>>[-<<<<<<<+>[<->-<<<+>>>]<[->+<]>>>>>>>]<<<<<<[->>>>>>+<<<<<<]<
+<<<<<<<<<]>>>>>>>-<<<<[-]+<<<]+>>>>>>>[-<<<<<<<->>>>>>>]+<<<<<<<[->>>>>>>->>[>>
>>>[->>+<<]>>>>]<<<<<<<<<[>[-]<->>>>>>>[-<<<<<<<+>[<->-<<<+>>>]<[->+<]>>>>>>>]<<
<<<<[->>>>>>+<<<<<<]<+<<<<<<<<<]>+++++[-[->>>>>>>>>+<<<<<<<<<]>>>>>>>>>]>>>>+<<<
<<[<<<<<<<<<]>>>>>>>>>[>>>>>[-<<<<<->>>>>]+<<<<<[->>>>>->>[-<<<<<<<+>>>>>>>]<<<<
<<<[->>>>>>>+<<<<<<<<<<<<<<<<[<<<<<<<<<]>>>>[-]+>>>>>[>>>>>>>>>]>+<]]+>>>>>>>[-<
<<<<<<->>>>>>>]+<<<<<<<[->>>>>>>-<<[-<<<<<+>>>>>]<<<<<[->>>>>+<<<<<<<<<<<<<<[<<<
<<<<<<]>>>[-]+>>>>>>[>>>>>>>>>]>[-]+<]]+>[-<[>>>>>>>>>]<<<<<<<<]>>>>>>>>]<<<<<<<
<<[<<<<<<<<<]>>>>[-]<<<+++++[-[->>>>>>>>>+<<<<<<<<<]>>>>>>>>>]>>>>-<<<<<[<<<<<<<
<<]]>>>]<<<<.>>>>>>>>>>[>>>>>>[-]>>>]<<<<<<<<<[<<<<<<<<<]>++++++++++[-[->>>>>>>>
>+<<<<<<<<<]>>>>>>>>>]>>>>>+>>>>>>>>>+<<<<<<<<<<<<<<<[<<<<<<<<<]>>>>>>>>[-<<<<<<
<<+>>>>>>>>]<<<<<<<<[->>>>>>>>+[-]>[>>>>>>>>>]<<<<<<<<<[>>>>>>>>[-<<<<<<<+>>>>>>
>]<<<<<<<[->>>>>>>+<<<<<<<<[<<<<<<<<<]>>>>>>>>[-]+>>]<<<<<<<<<<]]>>>>>>>>[-<<<<<
<<<+>>>>>>>>]<<<<<<<<[->>>>>>>>+>[>+>>>>>[-<<<<<->>>>>]<<<<<[->>>>>+<<<<<]>>>>>>
>>]<+<<<<<<<<[>>>>>>[->>+<<]<<<<<<<<<<<<<<<]>>>>>>>>>[>>>>>>>>>]<<<<<<<<<[>[-]<-
>>>>>>>>[-<<<<<<<<+>[<->-<<+>>]<[->+<]>>>>>>>>]<<<<<<<[->>>>>>>+<<<<<<<]<+<<<<<<
<<<]>>>>>>>>-<<<<<[-]+<<<]+>>>>>>>>[-<<<<<<<<->>>>>>>>]+<<<<<<<<[->>>>>>>>->[>>>
>>>[->>+<<]>>>]<<<<<<<<<[>[-]<->>>>>>>>[-<<<<<<<<+>[<->-<<+>>]<[->+<]>>>>>>>>]<<
<<<<<[->>>>>>>+<<<<<<<]<+<<<<<<<<<]>+++++[-[->>>>>>>>>+<<<<<<<<<]>>>>>>>>>]>>>>>
+>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<[<<<<<<<<<]>>>>>>>>>[>>>>>>[-<<<<<<->>>>>>]+<
<<<<<[->>>>>>->>[-<<<<<<<<+>>>>>>>>]<<<<<<<<[->>>>>>>>+<<<<<<<<<<<<<<<<<[<<<<<<<
<<]>>>>[-]+>>>>>[>>>>>>>>>]>+<]]+>>>>>>>>[-<<<<<<<<->>>>>>>>]+<<<<<<<<[->>>>>>>>
-<<[-<<<<<<+>>>>>>]<<<<<<[->>>>>>+<<<<<<<<<<<<<<<[<<<<<<<<<]>>>[-]+>>>>>>[>>>>>>
>>>]>[-]+<]]+>[-<[>>>>>>>>>]<<<<<<<<]>>>>>>>>]<<<<<<<<<[<<<<<<<<<]>>>>[-]<<<++++
+[-[->>>>>>>>>+<<<<<<<<<]>>>>>>>>>]>>>>>->>>>>>>>>>>>>>>>>>>>>>>>>>>-<<<<<<[<<<<
<<<<<]]>>>]", "AAAAAAAAAAAAAAAABBBBBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDEGFFEEEEDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAAAAABBBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDEEEFGIIGFFEEEDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAAABBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDEEEEFFFI KHGGGHGEDDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEEFFGHIMTKLZOGFEEDDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAABBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEEEFGGHHIKPPKIHGFFEEEDDDDDDDDDCCCCCCCCCCBBBBBBBBBBBBBBBBBB
AAAAAAAAAABBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEEFFGHIJKS  X KHHGFEEEEEDDDDDDDDDCCCCCCCCCCBBBBBBBBBBBBBBBB
AAAAAAAAABBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEEFFGQPUVOTY   ZQL[MHFEEEEEEEDDDDDDDCCCCCCCCCCCBBBBBBBBBBBBBB
AAAAAAAABBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEFFFFFGGHJLZ         UKHGFFEEEEEEEEDDDDDCCCCCCCCCCCCBBBBBBBBBBBB
AAAAAAABBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEFFFFFFGGGGHIKP           KHHGGFFFFEEEEEEDDDDDCCCCCCCCCCCBBBBBBBBBBB
AAAAAAABBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDEEEEEFGGHIIHHHHHIIIJKMR        VMKJIHHHGFFFFFFGSGEDDDDCCCCCCCCCCCCBBBBBBBBB
AAAAAABBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDEEEEEEFFGHK   MKJIJO  N R  X      YUSR PLV LHHHGGHIOJGFEDDDCCCCCCCCCCCCBBBBBBBB
AAAAABBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDEEEEEEEEEFFFFGH O    TN S                       NKJKR LLQMNHEEDDDCCCCCCCCCCCCBBBBBBB
AAAAABBCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDEEEEEEEEEEEEFFFFFGHHIN                                 Q     UMWGEEEDDDCCCCCCCCCCCCBBBBBB
AAAABBCCCCCCCCCCCCCCCCCCCCCCCCCDDDDEEEEEEEEEEEEEEEFFFFFFGHIJKLOT                                     [JGFFEEEDDCCCCCCCCCCCCCBBBBB
AAAABCCCCCCCCCCCCCCCCCCCCCCDDDDEEEEEEEEEEEEEEEEFFFFFFGGHYV RQU                                     QMJHGGFEEEDDDCCCCCCCCCCCCCBBBB
AAABCCCCCCCCCCCCCCCCCDDDDDDDEEFJIHFFFFFFFFFFFFFFGGGGGGHIJN                                            JHHGFEEDDDDCCCCCCCCCCCCCBBB
AAABCCCCCCCCCCCDDDDDDDDDDEEEEFFHLKHHGGGGHHMJHGGGGGGHHHIKRR                                           UQ L HFEDDDDCCCCCCCCCCCCCCBB
AABCCCCCCCCDDDDDDDDDDDEEEEEEFFFHKQMRKNJIJLVS JJKIIIIIIJLR                                               YNHFEDDDDDCCCCCCCCCCCCCBB
AABCCCCCDDDDDDDDDDDDEEEEEEEFFGGHIJKOU  O O   PR LLJJJKL                                                OIHFFEDDDDDCCCCCCCCCCCCCCB
AACCCDDDDDDDDDDDDDEEEEEEEEEFGGGHIJMR              RMLMN                                                 NTFEEDDDDDDCCCCCCCCCCCCCB
AACCDDDDDDDDDDDDEEEEEEEEEFGGGHHKONSZ                QPR                                                NJGFEEDDDDDDCCCCCCCCCCCCCC
ABCDDDDDDDDDDDEEEEEFFFFFGIPJIIJKMQ                   VX                                                 HFFEEDDDDDDCCCCCCCCCCCCCC
ACDDDDDDDDDDEFFFFFFFGGGGHIKZOOPPS                                                                      HGFEEEDDDDDDCCCCCCCCCCCCCC
ADEEEEFFFGHIGGGGGGHHHHIJJLNY                                                                        TJHGFFEEEDDDDDDDCCCCCCCCCCCCC
A                                                                                                 PLJHGGFFEEEDDDDDDDCCCCCCCCCCCCC
ADEEEEFFFGHIGGGGGGHHHHIJJLNY                                                                        TJHGFFEEEDDDDDDDCCCCCCCCCCCCC
ACDDDDDDDDDDEFFFFFFFGGGGHIKZOOPPS                                                                      HGFEEEDDDDDDCCCCCCCCCCCCCC
ABCDDDDDDDDDDDEEEEEFFFFFGIPJIIJKMQ                   VX                                                 HFFEEDDDDDDCCCCCCCCCCCCCC
AACCDDDDDDDDDDDDEEEEEEEEEFGGGHHKONSZ                QPR                                                NJGFEEDDDDDDCCCCCCCCCCCCCC
AACCCDDDDDDDDDDDDDEEEEEEEEEFGGGHIJMR              RMLMN                                                 NTFEEDDDDDDCCCCCCCCCCCCCB
AABCCCCCDDDDDDDDDDDDEEEEEEEFFGGHIJKOU  O O   PR LLJJJKL                                                OIHFFEDDDDDCCCCCCCCCCCCCCB
AABCCCCCCCCDDDDDDDDDDDEEEEEEFFFHKQMRKNJIJLVS JJKIIIIIIJLR                                               YNHFEDDDDDCCCCCCCCCCCCCBB
AAABCCCCCCCCCCCDDDDDDDDDDEEEEFFHLKHHGGGGHHMJHGGGGGGHHHIKRR                                           UQ L HFEDDDDCCCCCCCCCCCCCCBB
AAABCCCCCCCCCCCCCCCCCDDDDDDDEEFJIHFFFFFFFFFFFFFFGGGGGGHIJN                                            JHHGFEEDDDDCCCCCCCCCCCCCBBB
AAAABCCCCCCCCCCCCCCCCCCCCCCDDDDEEEEEEEEEEEEEEEEFFFFFFGGHYV RQU                                     QMJHGGFEEEDDDCCCCCCCCCCCCCBBBB
AAAABBCCCCCCCCCCCCCCCCCCCCCCCCCDDDDEEEEEEEEEEEEEEEFFFFFFGHIJKLOT                                     [JGFFEEEDDCCCCCCCCCCCCCBBBBB
AAAAABBCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDEEEEEEEEEEEEFFFFFGHHIN                                 Q     UMWGEEEDDDCCCCCCCCCCCCBBBBBB
AAAAABBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDEEEEEEEEEFFFFGH O    TN S                       NKJKR LLQMNHEEDDDCCCCCCCCCCCCBBBBBBB
AAAAAABBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDEEEEEEFFGHK   MKJIJO  N R  X      YUSR PLV LHHHGGHIOJGFEDDDCCCCCCCCCCCCBBBBBBBB
AAAAAAABBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDEEEEEFGGHIIHHHHHIIIJKMR        VMKJIHHHGFFFFFFGSGEDDDDCCCCCCCCCCCCBBBBBBBBB
AAAAAAABBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEFFFFFFGGGGHIKP           KHHGGFFFFEEEEEEDDDDDCCCCCCCCCCCBBBBBBBBBBB
AAAAAAAABBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEFFFFFGGHJLZ         UKHGFFEEEEEEEEDDDDDCCCCCCCCCCCCBBBBBBBBBBBB
AAAAAAAAABBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEEFFGQPUVOTY   ZQL[MHFEEEEEEEDDDDDDDCCCCCCCCCCCBBBBBBBBBBBBBB
AAAAAAAAAABBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEEFFGHIJKS  X KHHGFEEEEEDDDDDDDDDCCCCCCCCCCBBBBBBBBBBBBBBBB
AAAAAAAAAAABBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEEEFGGHHIKPPKIHGFFEEEDDDDDDDDDCCCCCCCCCCBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEEFFGHIMTKLZOGFEEDDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAAABBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDEEEEFFFI KHGGGHGEDDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAAAAABBBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDEEEFGIIGFFEEEDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBBBBB
")

    }

    const FACTOR_PROG: &str = {"[
   Takes an integer from stdin and emits its factors to stdout

   Factor an arbitrarily large positive integer

   Copyright (C) 1999 by Brian Raiter
   under the GNU General Public License
]

>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>-

*
* read in the number
*

<<<<<<<<<+
[-[>>>>>>>>>>][-]<<<<<<<<<<[[->>>>>>>>>>+<<<<<<<<<<]<<<<<<<<<<]
  >>>>>>>>>>,----------]
>>>>>>>>>>[------------------------------------->>>>>>>>>->]
<[+>[>>>>>>>>>+>]<-<<<<<<<<<<]-

*
* display the number and initialize the loop variable to two
*

[>++++++++++++++++++++++++++++++++++++++++++++++++.
  ------------------------------------------------<<<<<<<<<<<]
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
--------------------------.[-]
>>>>>>>>>>>>++<<<<+

*
* the main loop
*

[ [-]>>

  *
  * make copies of the number and the loop variable
  *

  [>>>>[-]>[-]>[-]>[-]
    >[-]>[-]
    <<<<<<<[->>>+>+<<<<]>>>>>>>>]
  <<<<<<<<<<[>>>>>>[-<<<<+>>>>]<<<<<<<<<<<<<<<<]>>>>>>>>>>
  [>[->>>+>>+<<<<<]>>>>>>>>>]
  <<<<<<<<<<[>>>>>>[-<<<<<+>>>>>]<<<<<<<<<<<<<<<<]>>>>>>>>>>

  *
  * divide the number by the loop variable
  *

  [>>>[-]>>>[-]>[-]>>>]                                  initialize
  <<<<<<<<<<[<<<<<<<<<<]
  >>>>>>>>>[-]>>>>>>>+<<<<<<<<[+]+
  [ ->>                               double divisor until above dividend
    [>>>>>>[->++<]>>>>]<<<<<<<<<<
    [>>>>>>>>[-]>[-]
       <<<<[->>>++<<<]<<<<<<<<<<<<<<<]>>>>>>>>>>
    [>>>>>>>>[->+<[->+<[->+<[->+<[->+<[->+<[->+<[->+<[->+<
            [->--------->>>>>>>>>+<<<<<<<<<<[->+<]]]]]]]]]]]>>]
    <<<<<<<<<<[>>>>>>>>>[-<+<<<+>>>>]<<<<<<<<<<<<<<<<<<<]>>>>>>>>>>
    [>>>>>>>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>
            [-<--------->>>>>>>>>>>+<<<<<<<<<<[-<+>]]]]]]]]]]]>>>]
    <<<<<<<<<<
    [>>>>[->>>+>>+<<<<<]<<<<<<<<<<<<<<]
    >>>>>>>>>>[>>>>>>>[-<<<+>>>]>>>]<<<<<<<<<<
    [>>>>>>>>[->-<]>
      [<<<<<<<<<[<[-]>>>>>>>>>>[-<<<<<<<<<<+>>>>>>>>>>]<<<<<<<<<<<<<<<<<<<]
        >>>>>>>>>>>>>>>>>>>]
      <<<<<<<<<<<<<<<<<<<]
    >>>>>>>>>[+[+[+[+[+[+[+[+[+[+[[-]<+>]]]]]]]]]]]<
  ]
  >>>>>>>>
  [                                   subtract divisor from dividend
    <<<<<<
    [>>>>>>>>[-]>[-]<<<<<[->>>+>+<<<<]>>>>>>]<<<<<<<<<<
    [>>>>>>>>[-<<<<+>>>>]<<<[->>>+>+<<<<]<<<<<<<<<<<<<<<]>>>>>>>>>>
    [>>>>>>>>>[-<<<<+>>>>]>]<<<<<<<<<<
    [>>>>>>>>[-<->]<<<<<<<<<<<<<<<<<<]>>>>>>>>>>
    [>>>>>>>[->+<[->+<[->+<[->+<[->+<[->+<[->+<[->+<[->+<[->+<
            [++++++++++[+>-<]>>>>>>>>>>-<<<<<<<<<<]]]]]]]]]]]>>>]
    >>>>>>>+
    [                                 if difference is nonnegative then
      [-]<<<<<<<<<<<<<<<<<            replace dividend and increment quotient
      [>>>>[-]>>>>[-<<<<+>>>>]<<[->>+<<]<<<<<<<<<<<<<<<<]>>>>>>>>>>
      [>>>>>>>>[->+<<<+>>]>>]<<<<<<<<<<
      [>>>[->>>>>>+<<<<<<]<<<<<<<<<<<<<]>>>>>>>>>>
      [>>>>>>>>>[-<<<<<<+>>>>>>[-<<<<<<+>>>>>>
                [-<<<<<<+>>>>>>[-<<<<<<+>>>>>>
                [-<<<<<<+>>>>>>[-<<<<<<+>>>>>>
                [-<<<<<<+>>>>>>[-<<<<<<+>>>>>>
                [-<<<<<<+>>>>>>[-<<<<<<--------->>>>>>>>>>>>>>>>+<<<<<<<<<<
                [-<<<<<<+>>>>>>]]]]]]]]]]]>]
      >>>>>>>
    ]                                 halve divisor and loop until zero
    <<<<<<<<<<<<<<<<<[<<<<<<<<<<]>>>>>>>>>>
    [>>>>>>>>[-]<<[->+<]<[->>>+<<<]>>>>>]<<<<<<<<<<
    [+>>>>>>>[-<<<<<<<+>>>>>>>[-<<<<<<<->>>>>>+>
             [-<<<<<<<+>>>>>>>[-<<<<<<<->>>>>>+>
             [-<<<<<<<+>>>>>>>[-<<<<<<<->>>>>>+>
             [-<<<<<<<+>>>>>>>[-<<<<<<<->>>>>>+>
             [-<<<<<<<+>>>>>>>]]]]]]]]]<<<<<<<
             [->>>>>>>+<<<<<<<]-<<<<<<<<<<]
    >>>>>>>
    [-<<<<<<<<<<<+>>>>>>>>>>>]
      >>>[>>>>>>>[-<<<<<<<<<<<+++++>>>>>>>>>>>]>>>]<<<<<<<<<<
    [+>>>>>>>>[-<<<<<<<<+>>>>>>>>[-<<<<<<<<->>>>>+>>>
              [-<<<<<<<<+>>>>>>>>[-<<<<<<<<->>>>>+>>>
              [-<<<<<<<<+>>>>>>>>[-<<<<<<<<->>>>>+>>>
              [-<<<<<<<<+>>>>>>>>[-<<<<<<<<->>>>>+>>>
              [-<<<<<<<<+>>>>>>>>]]]]]]]]]<<<<<<<<
              [->>>>>>>>+<<<<<<<<]-<<<<<<<<<<]
    >>>>>>>>[-<<<<<<<<<<<<<+>>>>>>>>>>>>>]>>
    [>>>>>>>>[-<<<<<<<<<<<<<+++++>>>>>>>>>>>>>]>>]<<<<<<<<<<
    [<<<<<<<<<<]>>>>>>>>>>
    >>>>>>
  ]
  <<<<<<

  *
  * make copies of the loop variable and the quotient
  *

  [>>>[->>>>+>+<<<<<]>>>>>>>]
  <<<<<<<<<<
  [>>>>>>>[-<<<<+>>>>]<<<<<[->>>>>+>>+<<<<<<<]<<<<<<<<<<<<]
  >>>>>>>>>>[>>>>>>>[-<<<<<+>>>>>]>>>]<<<<<<<<<<

  *
  * break out of the loop if the quotient is larger than the loop variable
  *

  [>>>>>>>>>[-<->]<
    [<<<<<<<<
      [<<[-]>>>>>>>>>>[-<<<<<<<<<<+>>>>>>>>>>]<<<<<<<<<<<<<<<<<<]
    >>>>>>>>>>>>>>>>>>]<<<<<<<<<<<<<<<<<<]
  >>>>>>>>[>-<[+[+[+[+[+[+[+[+[+[[-]>+<]]]]]]]]]]]>+

  [ [-]

    *
    * partially increment the loop variable
    *

    <[-]+>>>>+>>>>>>>>[>>>>>>>>>>]<<<<<<<<<<

    *
    * examine the remainder for nonzero digits
    *

    [<<<<<<[<<<<[<<<<<<<<<<]>>>>+<<<<<<<<<<]<<<<]
    >>>>>>>>>>>>>>>>>>>>[>>>>>>>>>>]<<<<<<<<<<[<<<<<<<<<<]
    >>>>-

    [ [+]

      *
      * decrement the loop variable and replace the number with the quotient
      *

      >>>>>>>>-<<[>[-]>>[-<<+>>]>>>>>>>]<<<<<<<<<<

      *
      * display the loop variable
      *

      [+>>[>>>>>>>>+>>]<<-<<<<<<<<<<]-
      [>>++++++++++++++++++++++++++++++++++++++++++++++++.
         ------------------------------------------------<<<<<<<<<<<<]
      ++++++++++++++++++++++++++++++++.[-]>>>>

    ]

    *
    * normalize the loop variable
    *

    >>>>>>
    [>>[->>>>>+<<<<<[->>>>>+<<<<<
       [->>>>>+<<<<<[->>>>>+<<<<<
       [->>>>>+<<<<<[->>>>>+<<<<<
       [->>>>>+<<<<<[->>>>>+<<<<<
       [->>>>>+<<<<<[->>>>>--------->>>>>+<<<<<<<<<<
       [->>>>>+<<<<<]]]]]]]]]]]>>>>>>>>]
    <<<<<<<<<<[>>>>>>>[-<<<<<+>>>>>]<<<<<<<<<<<<<<<<<]
    >>>>>>>>>

  ]<

]>>

*
* display the number and end
*

[>>>>>>>>>>]<<<<<<<<<<[+>[>>>>>>>>>+>]<-<<<<<<<<<<]-
[>++++++++++++++++++++++++++++++++++++++++++++++++.<<<<<<<<<<<]
++++++++++.
"};

    #[test]
    fn factor6() {
        progtest_input(FACTOR_PROG, "6\n", "6: 2 3\n")
    }

    #[test]
    fn factor179424691() {
        progtest_input(FACTOR_PROG, "179424691\n", "179424691: 179424691\n")
    }
}
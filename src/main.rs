extern crate clap;

mod mode_0;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;

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
            .help("Sets mode: 0 = Interpreter")
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

    println!("{}", execute(mode, contents.as_bytes(), cells));
}

fn execute(mode: usize, contents: &[u8], cells: usize) -> String {
    match mode {
        0 => mode_0::execute(contents, cells),
        _ => panic!("Invalid mode: {}", mode)
    }
}

#[cfg(test)]
mod tests {
    use execute;

    fn progtest(prog: &str, expected: &str, cells: usize) {
        for i in 0..1 {
            assert_eq!(execute(i, prog.as_bytes(), cells), expected)
        }
    }

    fn progtest_default(prog: &str, expected: &str) {
        progtest(prog, expected, 30000)
    }

    #[test]
    fn bf1to5() {
        progtest_default("++++++++ ++++++++ ++++++++ ++++++++ ++++++++ ++++++++     >+++++      [<+.>-]",
            "12345")
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

<<  Finish at cell zero"#, "Hello, world!\n")
    }
}
use std::process;
use std::io;

pub fn nfa(state: i32, strlang: String) {
    // Rejects the non-matching string
    if strlang.len() == 0 && (state != 3 && state != 4 && state != 6) {
        println!("Last State {}", state);
        println!("String REJECTED!");
        process::exit(1);
    }

    // Accepts the matching string
    if strlang.len() == 0 && (state == 3 || state == 4 || state == 6) {
        println!("Last State {}", state);
        println!("String ACCEPTED!");
        process::exit(1);
    }
    // Extracting the next letter
    let alpha = strlang.chars().nth(0);

    // Extracting the remaining string to be matched
    let substr = String::from(strlang.get(1..).unwrap());

    if state == 0 && alpha==Some('a') {
        println!("State {}", state);
        nfa(4, substr);
    } else if state == 4 && alpha == Some('b') {
        println!("State {}", state);
        nfa(5, substr);
    } else if state == 5 && alpha == Some('c') {
        println!("State {}", state);
        nfa(6, substr);
    } else if state == 6 && alpha == Some('b') {
        println!("State {}", state);
        nfa(5, substr);
    } else if state == 0 && alpha == Some('b') {
        println!("State {}", state);
        nfa(1, substr);
    } else if state == 1 && alpha == Some('b') {
        println!("State {}", state);
        nfa(3, substr);
    } else if state == 1 && alpha == Some('a') {
        println!("State {}", state);
        nfa(2, substr);
    } else if state == 2 && alpha == Some('a') {
        println!("State {}", state);
        nfa(2, substr);
    } else if state == 2 && alpha == Some('b') {
        println!("State {}", state);
        nfa(3, substr);
    } else {
        println!("State {}", state);
        println!("String REJECTED!");
    }
}

fn main() {
    println!("RegEx: ba*b + a(bc)*");
    println!("Enter a String:");
    let mut stra = String::new();
	io::stdin()
		.read_line(&mut stra)
		.expect("failed to read input.");

    let a= String::from(stra.trim());
    nfa(0, a);
}
use std::io;
use std::io::prelude::*;
use std::string::String;

fn mal_read(in: &str) -> &str {
    in
}

fn mal_eval(in: &str) -> &str {
    in
}

fn mal_print(in: &str) -> &str {
    in
}

fn mal_rep(in: &str) -> &str {
    let read = mal_read(in);
    let eval = mal_eval(read);
    let print = mal_print(eval);

    print
}

fn main_loop() {
    loop {
        print!("eval> ");
        let mut in = String::new();
        let check = io::stdin().read_line(&mut in);
        if check == Ok(0) { break; }
        println!("{}", in);
    }
}
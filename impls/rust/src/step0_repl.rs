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

fn main() {
    loop {
        let mut in = String::new();
    }
}
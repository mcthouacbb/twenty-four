use std::{io, str::SplitWhitespace};

use number_list::NumberList;

mod number_list;
mod solver;

fn solve_command(tokens: SplitWhitespace<'_>) {
    todo!();
}

fn generate_command(tokens: SplitWhitespace<'_>) {
    todo!();
}

fn main() {
    loop {
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");

        let mut tokens = cmd.trim().split_whitespace();
        match tokens.next() {
            Some("solve") => {
                solve_command(tokens);
            }
            Some("generate") => {
                generate_command(tokens);
            }
            _ => {
                print!("Unknown command: {}", cmd);
            }
        }
    }
}

use std::{io, str::SplitWhitespace};

use number_list::NumberList;
use solver::Solver;

mod graph;
mod number_list;
mod op;
mod solver;

fn solve_command(tokens: SplitWhitespace<'_>) {
    let solver = Solver::new(NumberList::new(&vec![8, 4, 7, 9]));
    // let solver = Solver::new(NumberList::new(&vec![3, 4, 5]));
    // let solver = Solver::new(NumberList::new(&vec![7, 2, 4, 8]));
    solver.solve();
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

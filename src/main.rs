use std::{io, str::SplitWhitespace};

use number_list::NumberList;
use solver::Solver;

mod graph;
mod number_list;
mod op;
mod solver;

fn solve_command(mut tokens: SplitWhitespace<'_>) {
    let mut nums = NumberList::new(&Vec::new());
    let mut target = 24;
    loop {
        let tok = tokens.next();
        if tok.is_none() {
            break;
        }
        let str = tok.unwrap();
        let num = str.parse::<i32>();
        if num.is_err() {
            println!("Invalid solve argument: {}", str);
            continue;
        }
        nums.add(num.unwrap());
    }
    println!("Solving {} for {}", nums, target);
    // TODO: actually use target
    let mut solver = Solver::new(nums.clone());
    // let solver = Solver::new(NumberList::new(&vec![8, 4, 7, 9]));
    // let solver = Solver::new(NumberList::new(&vec![3, 4, 5]));
    // let solver = Solver::new(NumberList::new(&vec![7, 2, 4, 8]));
    solver.solve();
    solver.print_graph();
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

mod sorting;
mod easy;

use crate::easy::variables::run_variables;
use crate::easy::match_expr::run_match_expr;
use crate::easy::ownership::ownership;

fn main() {
    //println!("Variables:");
    //run_variables();
    // println!("Match Expression:");
    // run_match_expr();
    println!("Ownership: ");
    ownership();
}

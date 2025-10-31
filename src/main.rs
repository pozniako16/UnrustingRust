mod sorting;
mod aa_easy;

use crate::aa_easy::variables::run_variables;
use crate::aa_easy::match_expr::run_match_expr;
use crate::aa_easy::ownership::ownership;

fn main() {
    //println!("Variables:");
    //run_variables();
    // println!("Match Expression:");
    // run_match_expr();
    println!("Ownership: ");
    ownership();
}

mod sorting;
mod easy;
mod intermediate;

use crate::easy::variables::run_variables;
use crate::easy::match_expr::run_match_expr;
use crate::easy::ownership::ownership;
use crate::easy::structs::structs;
use crate::intermediate::iterator::iterator;

fn main() {
    //println!("Variables:");
    //run_variables();
    // println!("Match Expression:");
    // run_match_expr();
    // println!("Ownership: ");
    // ownership();
    // println!("Structs: ");
    // structs()
    println!("Iterator: ");
    iterator();
}

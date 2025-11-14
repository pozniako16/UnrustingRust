extern crate core;

mod sorting;
mod easy;
mod intermediate;


use crate::easy::variables::run_variables;
use crate::easy::match_expr::run_match_expr;
use crate::easy::ownership::ownership;
use crate::easy::structs::structs;
use crate::intermediate::iterator::iterator;
use crate::intermediate::error_handling::error_handling;
use crate::intermediate::generic_and_traits::generic_traits;
use crate ::intermediate::cli::cli;

fn main() {
    //println!("Variables:");
    //run_variables();
    // println!("Match Expression:");
    // run_match_expr();
    // println!("Ownership: ");
    // ownership();
    // println!("Structs: ");
    // structs()
    //println!("Iterator: ");
    //iterator();
    //println!("Error handling: ");
    //error_handling()
    //println!("Generics and traits: ");
    //generic_traits()
    println!("CLI: ");
    cli()
    
    
}

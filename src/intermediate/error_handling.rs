/*
Write a function that takes a vector of strings and attempts to parse each one into an integer (i32). The function should return a Result<Vec<i32>, String> where:
On success, it returns a vector of all parsed integers.
On failure (if any string does not parse correctly), it returns an error with a descriptive message.
To solve this, use iterator methods like map, and_then, and collect to perform parsing and error handling in a concise way.
*/

fn parse_ints(ints_str: &Vec<String>) -> Result<Vec<i32>, String> {
    ints_str
        .iter()
        .map(|x| {
            match x.parse::<i32>() {
                Ok(val) => Ok(val),
                Err(err) => Err(format!("Parsing failed - {}, {} is not a number", err, x)),
            }
        })
        .collect()
}

fn parse_positive_ints(ints_str: &Vec<String>) -> Result<Vec<i32>, String> {
    ints_str
        .iter()
        .map(|x| x
            .parse::<i32>()
            .map_err(|err| format!("Parsing failed - {}, {} is not a number", err, x))
        )
        .filter(|res| match res { 
            Ok(val) => *val >= 0,
            Err(_) => true
        })
        .collect()
}

fn display_result(result: Result<Vec<i32>, String>) {
    match result {
        Ok(v) => {
            for val in v {
                print!("{} ", val)
            }
        },
        Err(e) => {
            print!("Error: {}", e)
        }
    }
}

pub fn error_handling() {
    let vec = vec![String::from("1"), String::from("2")];
    println!("Result: ");
    display_result(parse_ints(&vec));
    let wrong_vec =  vec![String::from("1"), String::from("hello"), String::from("13")];
    println!("\n____");
    display_result(parse_ints(&wrong_vec));
    println!("\n____");
    let vec_with_neg = vec![String::from("1"), String::from("-1"), String::from("42")];
    display_result(parse_positive_ints(&vec_with_neg));
    let wrong_vec_with_neg = vec![String::from("1"), String::from("--1"), String::from("42")];
    println!("\n____");
    display_result(parse_positive_ints(&wrong_vec_with_neg));


}

use crate::easy::match_expr::CreatureType::{ANIMAL, LEGENDARY, UNKNOWN};

fn is_an_animal(to_test: &str) -> bool {
    matches!(to_test, "cow" | "wolf" | "cat" | "dog" | "bird")
}

fn compare(lhs: u32, rhs: u32) -> &'static str {
    if lhs > rhs {
        "bigger"
    } else if lhs < rhs{ 
        "smaller"
    } else {
        "equal"
    }
}

fn if_else_stmt() {
    let car_res = is_an_animal("car");
    println!("Car is an animal: {car_res}");
    let cow_res = is_an_animal("cow");
    println!("Cow is an animal: {cow_res}");
    println!("lhs is {}", compare(1, 2));
    println!("lhs is {}", compare(1, 1));
    println!("lhs is {}", compare(2, 1));
}

#[derive(Debug)]
enum CreatureType {
    ANIMAL,
    LEGENDARY,
    UNKNOWN
}

fn categorize_creature(to_categorize: &str) -> CreatureType{
    match to_categorize {
        "wolf" | "cat" | "dog" | "bird" => CreatureType::ANIMAL,
        "dragon" | "ogre" | "unicorn" => CreatureType::LEGENDARY,
        _ => CreatureType::UNKNOWN
    }
}

fn categorize_number(to_categorize: u32) -> &'static str{
    match to_categorize {
        0..10 => "single digit",
        10..100 => "double digits",
        _ => "too big"
    }
}

fn match_stmt() {
    println!("Cow is {:?}", categorize_creature("cow"));
    println!("Unicorn is {:?}", categorize_creature("unicorn"));
    println!("Clakbax is {:?}", categorize_creature("clakbax"));
    println!("1 is {}", categorize_number(1));
    println!("10 is {}", categorize_number(10));
    println!("2000 is {}", categorize_number(2000));
}

pub fn run_match_expr() {
    if_else_stmt();
    match_stmt();
}
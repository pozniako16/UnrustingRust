fn mutable_int_variable_example() {
    let mut x = 1;
    println!("int before change: {x}");
    x += 1;
    println!("int after change: {x}");
}

fn mutable_string_variable_example() {
    let mut str = "Hello";
    let combined = format!("{} {}", str, "world");
    println!("Str manip via format: {combined}");
    //DOES NOT WORK, reference is mutable not value
    //str = str.to_owned() + " Universe";
    let str_obj = String::from("Hello");
    //DOES NOT WORK Cannot borrow immutable
    //str_obj.push_str(" Universe");
    let mut str_mut = String::from("Hello");
    str_mut.push_str(" Universe");
    println!("Mut String manip: {str_mut}");
}

fn boolean_variable_example() {
    let var = true;
    //DOES NOT WORK, immutable
    //var = !var;
    let mut mut_var = true;
    println!("Boolean pre inversion {mut_var}");
    mut_var = !mut_var;
    println!("Boolean post inversion {mut_var}");
}

fn mutability_or_shadowing() {
    let x = 5; //immutable int
    println!("Immutable int {x}");
    let mut x = 55; //shadowed mutable int
    println!("Shadowed mutable int {x}");
    //syntax fail, does not work
    //mut x = 1;
}

pub fn run_variables() {
    mutable_int_variable_example();
    mutable_string_variable_example();
    boolean_variable_example();
    mutability_or_shadowing();
}
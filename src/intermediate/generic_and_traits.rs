/*
Exercise 3: Use Generics and Traits Together
Create a generic struct Pair<T> that holds two values of the same type.
Implement a trait Printable for Pair<T> to print both values.
Write a function that takes any type implementing Printable and calls its print method.
This will help you practice generic types, trait definitions, trait implementations, and trait bounds in function parameters.
*/

struct Pair<'a, T: std::fmt::Display> {
    first: &'a T,
    second: &'a T
}

trait Printable {
    fn print(&self);
}

impl<'a, T: std::fmt::Display> Pair<'a, T> {
    fn new(first: &'a T, second: &'a T) -> Self {
        Pair{first, second}
    }
}

impl<'a, T: std::fmt::Display> Printable for Pair<'a, T> {
    fn print(&self) {
        println!("({}, {})", self.first, self.second);
    }
}


pub fn generic_traits() {
    let pair = Pair::new(&1, &2);
    pair.print();
    let first = "hello";
    let second = "world";

    let pair = Pair::new(&first, &second);
    pair.print()
}

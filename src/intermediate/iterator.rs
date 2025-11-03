/*
Exercise 1: Implement a Custom Iterator
Define a struct Counter that produces a sequence of numbers (e.g., from 1 to 5).
Implement the Iterator trait for Counter with the next() method.
Use the iterator in a loop and also experiment with iterator adapters like map or filter.
*/

#[derive(Copy, Clone)]
struct Counter {
    value: u32,
    max: u32
}


impl Counter {
    fn new(to: u32) -> Self{
        Counter {
            value: 0,
            max: to
        }
    }

    fn reset(&mut self) {
        self.value = 0;
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max > self.value {
            let curr = self.value;
            self.value += 1;
            Some(curr)
        } else {
            None
        }
    }
}

pub fn iterator() {
    let mut counter = Counter::new(10);
    for val in counter {
        print!("{} ", val);
    }
    counter.reset();
    let counter = Counter::new(10);
    let filtered = counter
        .map(|v| v + 1) // add one to all values 
        .skip(2)// skip the first 2
        .filter(|v| v % 2 == 0)// remove odd
        .take(3); // keep only the first 3
    println!("\nMap and filtered");
    for val in filtered {
        print!("{} ", val)
    }

}

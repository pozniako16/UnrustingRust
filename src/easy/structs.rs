use fmt::Display;
use std::fmt;
use std::fmt::Formatter;
/*
Create a struct with multiple fields.
Implement an impl block with various methods:
A constructor (new) function returning an instance
Getter methods using &self
Setter methods or operations modifying the struct using &mut self
Write sample code to create an instance, call methods, and demonstrate state changes.
 */

#[derive(Clone)]
struct  User<'a> {
    name: &'a str, 
    email: &'a str,
    pwd: &'a str
}

impl <'a> User<'a> {
    fn new(name: &'a str, email: &'a str, pwd: &'a str) -> Self {
        User{ name, email, pwd }
    }
    
    fn get_name(&self) -> &str {
        self.name
    }
    
    fn set_name(&mut self, name: &'a str) -> bool {
        if (name.len() == 0) || (name == self.name) {
            return false
        }
        self.name = name;
        true
    }
    
    fn get_email(&self) -> &str {
        self.email
    }
    
    fn set_email(&mut self, email: &'a str){
        self.email = email;
    }
    
    fn equals(&self, rhs: &User) -> bool {
        self.email == rhs.email && self.pwd == rhs.pwd
    }
}

impl<'a> Display for User<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "USER -> [name: {}, email: {}, pwd: {}]", self.name, self.email, "*".repeat(self.pwd.len()))
    }
}


pub fn structs() {
    let user = User::new("John", "john@doe.com", "12345678");
    println!("User: {}", user);
    println!("Username: {}", user.get_name());
    println!("Email: {}", user.get_email());
    //trying to call setter on immutable instance, of course does not work
    //user.set_name("Johnny");
    let mut user2 = user.clone();
    user2.set_name("Johnny");
    println!("Edited User: {}", user2);
    println!("User equal ?: {}", user.equals(&user2));
    user2.set_email("johnnyboy@halliday");
    println!("And now ?: {}", user.equals(&user2))
    
}
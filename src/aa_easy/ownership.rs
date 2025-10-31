// Define a Book struct with title and pages


#[derive(Clone)]
struct Book<'a> {
    title: &'a str,
    pages: Vec<&'a str>,
    is_checked_out: bool
}

fn read_book_once(book: Book) {
    // Write a function that takes ownership of a Book and "reads" it
    // Try to use the book after passing it - document why it fails
    // Implement Clone to allow copying books
    println!("Title: {:?}", book.title);
    for page in book.pages {
        println!("{:?}", page)
    }
}

// Write a function that borrows a Book immutably and prints its title
// Call it multiple times on the same book - it should work!
fn read_book_title(book: &Book) {
    println!("Title: {:?}", book.title);
}

// Create a function that compares two borrowed books
fn compare_books(rhs: &Book, lhs: &Book) -> bool {
    rhs.title == lhs.title
}

// Add a field to track if a book is borrowed (e.g., is_checked_out: bool)
// Write a function that takes &mut Book and marks it as checked out
fn check_out_book_mut(book: &mut Book) {
    book.is_checked_out = true;
}

/*
Does not work as we can't modify immutable borrow. Event if the reference does not change (I think)
It still tracks what's contained in the reference highlighting a strong bond between reference
and value.
fn check_out_book_immut(book: &Book) {
    book.is_checked_out = true;
}
*/

// Write a function that creates a Book and tries to return &Book (should fail)
// Write a function that takes &Book and returns &str (title field) - should work
// Experiment with which references can be returned safely
/*
You can't return a reference to an object that is destroyed coming out of the function's scope.
You could avoid this by specify a 'static lifetime, but it's very much avoiding the core principles
of rust's garbage collection.
fn new_generic_book() -> &Book {
    return &Book {
        title: String::from("One piece"),
        pages: Vec::from([
            String::from("My treasure exists !"),
            String::from("Go find it,"),
            String::from("It's out there")
        ]),
        is_checked_out: false
    };
}
 */

fn new_generic_book<'a>() -> Book<'a> {
    Book {
        title: "One piece",
        pages: Vec::from([
            "My treasure exists !",
            "Go find it,",
            "It's out there"
        ]),
        is_checked_out: false
    }
}

fn get_title<'a>(book: &Book<'a>) -> &'a str {
    book.title
}

// Probably that returning a reference to one page is going to be very stupid as it could hinder the
// integrity of a book
fn get_first_page<'a>(book: &Book<'a>) -> &'a str {
    match book.pages.first() {
        None => { "" }
        Some(val) => {val}
    }
}

// Write a function that borrows the Vec and prints all titles
fn read_all_titles(books: &Vec<Book>) {
    for book in books.iter() {
        read_book_title(&book);
    }
}

// Write a function that takes &mut Vec<Book> to add a new book
fn add_generic_book(books: &mut Vec<Book>) {
    books.push(new_generic_book());
}


// Write a function that takes ownership of the Vec and consumes it
fn consume_books(books: Vec<Book>) {
    for book in books.into_iter() {
        read_book_once(book);
    }
}

pub fn ownership() {
    let book = Book {
        title: "One piece",
        pages: Vec::from([
            "My treasure exists !",
            "Go find it,",
            "It's out there"
        ]),
        is_checked_out: false
    };
    let book_copy = book.clone();

    read_book_once(book);
    // book is owned by 'read_book' and thus the reference is destroyed after the function call
    // read_book(book) does not work as the reference is not 'borrowable' anymore

    // this works as we using a copied version
    read_book_once(book_copy);
    // This still not works as the copied variable is borrowed by `read_book`
    // read_book(book_copy)

    let book = Book {
        title: "One piece",
        pages: Vec::from([
            "My treasure exists !",
            "Go find it,",
            "It's out there"
        ]),
        is_checked_out: false
    };
    read_book_title(&book);
    read_book_title(&book);
    read_book_title(&book);
    println!("Comparing identical book: {}", compare_books(&book, &book));
    let mut book2 = book.clone();
    book2.title = "Two Piece";
    println!("Comparing different book: {}", compare_books(&book, &book2));
    check_out_book_mut(&mut book2);
    check_out_book_mut(&mut book2);
    // Try to have both mutable and immutable borrows - document the error
    // Try to have two mutable borrows - document the error
    let mut book3 = book2.clone();
    let book_borrow = book2;
    // Value has been moved to book borrow and thus can't be transferred to book_borrow_2
    // let book_borrow_2 = book2;
    // This works as the value is owned by book_borrow
    let transitive_book_borrow = book_borrow;

    let mut book_borrow_mut =  book3;
    // still don't work because it's not about mutability but ownership.
    // Adding mutability does not change anything
    // let mut book_borrow_mut =  book3;

    println!("A title returned from book: {}", get_title(&book_borrow_mut));
    println!("A title returned from book: {}", get_title(&book_borrow_mut));
    let mut page = get_first_page(&book_borrow_mut);
    println!("First page: {}", page);
    //don't understand why it does not modify the ref
    page = "My treasure does not exist";
    println!("First page: {}", get_first_page(&book_borrow_mut));

    // Create a Vec<Book> with 3-4 books
    let mut book_vec = Vec::from([
        new_generic_book(),
        new_generic_book(),
        new_generic_book()
    ]);
    println!("Reading all titles: ");
    read_all_titles(&book_vec);
    add_generic_book(&mut book_vec);
    println!("Reading all titles after book addition");
    read_all_titles(&book_vec);
    println!("Consuming books");
    consume_books(book_vec);
    //Does not work since it's been consumed
    //consume_books(book_vec);
    
    
    
}




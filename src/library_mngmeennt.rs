#[allow(dead_code)]
struct Person {
    name: String,
    age: u32,
}

struct Book {
    title: String,
    author: String,
    is_available: bool,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn borrow_book(&mut self, title: &str) -> Option<&mut Book> {
        for book in &mut self.books {
            if book.title == title && book.is_available {
                book.is_available = false;
                return Some(book);
            }
        }
        None
    }

    fn return_book(&mut self, title: &str) -> bool {
        for book in &mut self.books {
            if book.title == title && !book.is_available {
                book.is_available = true;
                return true;
            }
        }
        false
    }
}

/*

let mut library = Library::new();

    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    let book1 = Book {
        title: String::from("Book 1"),
        author: String::from("Author 1"),
        is_available: true,
    };

    let book2 = Book {
        title: String::from("Book 2"),
        author: String::from("Author 2"),
        is_available: true,
    };

    library.add_book(book1);
    library.add_book(book2);

    if let Some(borrowed_book) = library.borrow_book("Book 1") {
        println!("{} borrowed {} by {}", person1.name, borrowed_book.title, borrowed_book.author);
    } else {
        println!("{} couldn't borrow the book.", person1.name);
    }

    if library.return_book("Book 2") {
        println!("{} returned the book.", person2.name);
    } else {
        println!("{} couldn't return the book.", person2.name);
    }

*/

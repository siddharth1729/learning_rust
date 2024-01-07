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

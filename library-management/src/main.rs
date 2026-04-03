#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_available: bool,
}

#[derive(Debug)]
struct Library {
    name: String,
    address: String,
    book: Option<Book>, // Library may or may not have a book
}

#[derive(Debug)]
enum LibraryError {
    BookNotAvailable,
    BookNotFound,
    AlreadyBorrowed,
}

impl Book {
    fn borrow(&mut self) -> Result<&mut Book, LibraryError> {
        if self.is_available == true {
            self.is_available = false;
            Ok(self)
        } else {
            Err(LibraryError::AlreadyBorrowed)
        }
    }
    fn return_book() {}
}

impl Library {
    fn add_book() {}
    fn borrow_book(&mut self) -> Result<&mut Book, LibraryError> {
        if let Some(book) = self.book.as_mut() {
            match book.borrow() {
                Ok(borrowed_book) => Ok(borrowed_book),
                Err(err) => Err(err),
            }
        } else {
            Err(LibraryError::BookNotFound)
        }
    }
    fn return_book() {}
}

fn main() {
    let book = Book {
        title: String::from("The Rust book"),
        author: String::from("Steve Klabnik"),
        is_available: true,
    };

    let mut library = Library {
        name: String::from("City library"),
        address: String::from("123 Library Lane"),
        book: Some(book), // assuming that there is only one book
    };

    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed book: {:#?}", result);
        }
        Err(err) => {
            println!("Error: {:#?}", err);
        }
    }
}

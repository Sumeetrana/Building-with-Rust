struct Book {
    title: String,
    author: String,
    is_available: String
}

struct Library {
    name: String,
    address: String,
    book: String
}

enum LibraryError {
    BookNotAvailable,
    BookNotFound,
    AlreadyBorrowed
}

fn main() {
    
}
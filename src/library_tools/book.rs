/// This is the Book struct, which stores information about a book.
/// It contains the title and author of the book.
pub struct Book {
    pub title: String,
    pub author: String,
}

impl Book {
    /// Constructor for the Book struct
    /// Initializes a new Book with the given title and author,
    /// and sets its status to BOOK_NOT_TAKEN
    pub fn new(book_title: String, book_author: String) -> Self {
        Self {
            title: book_title.to_string(),
            author: book_author.to_string(),
        }
    }
}

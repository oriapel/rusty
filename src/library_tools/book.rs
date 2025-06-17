pub const BOOK_TAKEN: bool = false;
pub const BOOK_NOT_TAKEN: bool = true;

/// This module defines the Book struct and its methods
pub struct Book {
    pub title: String,
    pub author: String,
    pub status: bool,
}

impl Book {
    /// Constructor for the Book struct
    /// Initializes a new Book with the given title and author,
    /// and sets its status to BOOK_NOT_TAKEN
    pub fn new(book_title: String, book_author: String) -> Self {
        Self {
            title: book_title.to_string(),
            author: book_author.to_string(),
            status: BOOK_NOT_TAKEN,
        }
    }

    /// This function prints the book's details
    /// It displays the title, author, and availability status of the book
    pub fn print_book(&self) {
        println!(
            "Title: {}, Author: {}, Available: {}",
            self.title,
            self.author,
            if self.status == BOOK_TAKEN {
                "No"
            } else {
                "Yes"
            }
        );
    }
}

//!
use crate::library_tools::book::Book;
use std::fmt;

#[derive(PartialEq)]
pub enum BookStatus {
    BookTaken,
    BookNotTaken,
}

/// The [LibraryBook] struct represents a book in the library.
/// It contains a [Book] and its status, which can be either [BookStatus::BookTaken] or [BookStatus::BookNotTaken].
pub struct LibraryBook {
    pub book: Book,
    pub status: BookStatus,
}

impl LibraryBook {
    /// Constructor for the [LibraryBook] struct
    /// Initializes a new [LibraryBook] with the given title and author,
    /// and sets its status to [BookStatus::BookNotTaken]
    pub fn new(book_title: String, book_author: String) -> Self {
        Self {
            book: Book::new(book_title, book_author),
            status: BookStatus::BookNotTaken,
        }
    }
}

impl fmt::Display for LibraryBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let book_status = match self.status {
            BookStatus::BookTaken => "Borrowed",
            BookStatus::BookNotTaken => "Available",
            _ => "Unknown",
        };
        write!(
            f,
            "Title: {}, Author: {}, Status: {}",
            self.book.title, self.book.author, book_status
        )
    }
}

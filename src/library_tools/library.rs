use std::fmt::Display;

use crate::library_tools::library_book::{BookStatus, LibraryBook};

/// defined errors for the library module
pub enum LibraryErrors {
    BookNotFound,
    InvalidBookStatus,
}

/// The [Library] struct represents a collection of [LibraryBook]s.
/// It contains a vector of [LibraryBooks] and provides methods to manage them
pub struct Library {
    pub library_books: Vec<LibraryBook>,
}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut all_books_data = String::new();
        for (index, book) in self.library_books.iter().enumerate() {
            all_books_data.push_str(&format!("{}. {}\n", index, book));
        }
        write!(
            f,
            "Library contains the following books:\n{}",
            all_books_data
        )
    }
}

impl Library {
    // Constructor for the [Library] struct
    // Initializes an empty [Library]
    pub fn new() -> Self {
        Self {
            library_books: Vec::new(),
        }
    }

    /// This function adds a new [LibraryBook] to the library
    pub fn add_new_book(&mut self, book: LibraryBook) {
        // I could use the next line if I would to run this code on limited memory machine
        // books.try_reserve(1)?;
        self.library_books.push(book);
    }

    /// This function borrows a book from the [Library], changes the [LibraryBook]'s [BookStatus] to [BookStatus::BookTaken]
    pub fn borrow_book(&mut self, name: &String) -> Result<(), LibraryErrors> {
        self.change_book_availability(name, BookStatus::BookTaken)
    }

    /// This function returns a [LibraryBook] to the [Library], changes the [LibraryBook]'s [BookStatus] to [BookStatus::BookNotTaken]
    pub fn return_book(&mut self, name: &String) -> Result<(), LibraryErrors> {
        self.change_book_availability(name, BookStatus::BookNotTaken)
    }

    /// This function returns a mutable [LibraryBook] by its name, if it exists
    fn get_mut_book_by_name(&mut self, name: &String) -> Option<&mut LibraryBook> {
        self.library_books
            .iter_mut()
            .find(|book| &book.book.title == name)
    }

    /// This function returns a [LibraryBook] by its name, if it exists
    pub fn get_book_by_name(&self, name: &String) -> Option<&LibraryBook> {
        self.library_books
            .iter()
            .find(|book| &book.book.title == name)
    }

    /// This function changes the [LibraryBook]'s [BookStatus],
    /// to be used when borrowing or returning a book.
    fn change_book_availability(
        &mut self,
        name: &String,
        new_book_status: BookStatus,
    ) -> Result<(), LibraryErrors> {
        let library_book = self
            .get_mut_book_by_name(name)
            .ok_or(LibraryErrors::BookNotFound)?;

        if library_book.status != new_book_status {
            library_book.status = new_book_status;
            return Ok(());
        }
        Err(LibraryErrors::InvalidBookStatus)
    }
}

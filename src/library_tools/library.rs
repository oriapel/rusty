use crate::library_tools::library_book::{BookStatus, LibraryBook};

/// defined errors for the library module
pub enum LibraryErrors {
    BookNotFound,
    InvalidBookStatus,
}

/// The library struct represents a collection of books
/// It contains a vector of books and provides methods to manage them
pub struct Library {
    pub library_books: Vec<LibraryBook>,
}

impl Library {
    // Constructor for the library struct
    // Initializes an empty library
    pub fn new() -> Self {
        Self {
            library_books: Vec::new(),
        }
    }

    /// This function print all books' data
    pub fn list_all_books(&self) {
        for (index, book) in self.library_books.iter().enumerate() {
            print!("{}. ", index);
            println!("{book}");
        }
    }

    /// This function adds a new book to the library
    pub fn add_new_book(&mut self, book: LibraryBook) {
        // I could use the next line if I would to run this code on limited memory machine
        // books.try_reserve(1)?;
        self.library_books.push(book);
    }

    /// This function borrows a book from the library, changes the book's status to BOOK_TAKEN
    pub fn borrow_book(&mut self, name: &String) -> Result<(), LibraryErrors> {
        return self.change_book_availability(name, BookStatus::BookTaken);
    }

    /// This function returns a book to the library, changes the book's status to BOOK_NOT_TAKEN
    pub fn return_book(&mut self, name: &String) -> Result<(), LibraryErrors> {
        return self.change_book_availability(name, BookStatus::BookNotTaken);
    }

    /// This function returns a book by its index, if it exists
    pub fn get_book_by_index(&self, index: u32) -> Option<&LibraryBook> {
        self.library_books.get(index as usize)
    }

    /// This function returns a mutable reference to a book by its index, if it exists
    fn get_mut_book_by_index(&mut self, index: u32) -> Option<&mut LibraryBook> {
        self.library_books.get_mut(index as usize)
    }

    fn get_mut_book_by_name(&mut self, name: &String) -> Option<&mut LibraryBook> {
        self.library_books.iter_mut().find(|book| &book.book.title == name)
    }

    /// This function changes the book's availability status,
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
            Ok(())
        } else {
            Err(LibraryErrors::InvalidBookStatus)
        }
    }
}

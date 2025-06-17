use crate::library_tools::book::{BOOK_NOT_TAKEN, BOOK_TAKEN, Book};

/// defined errors for the library module
pub enum LibraryErrors {
    BookNotFound,
    InvalidBookStatus,
}

/// The library struct represents a collection of books
/// It contains a vector of books and provides methods to manage them
pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    // Constructor for the library struct
    // Initializes an empty library
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    /// This function print all books' data
    pub fn list_all_books(&self) {
        for (index, book) in self.books.iter().enumerate() {
            print!("{}. ", index);
            book.print_book();
        }
    }

    /// This function adds a new book to the library
    pub fn add_new_book(&mut self, book: Book) {
        // I could use the next line if I would to run this code on limited memory machine
        // books.try_reserve(1)?;
        self.books.push(book);
    }

    /// This function changes the book's availability status,
    /// to be used when borrowing or returning a book.
    fn change_book_availability(
        &mut self,
        index: u32,
        new_book_status: bool,
    ) -> Result<(), LibraryErrors> {
        if let Some(book) = self.get_mut_book_by_index(index).as_mut() {
            if book.status != new_book_status {
                book.status = new_book_status;
                Ok(())
            } else {
                Err(LibraryErrors::InvalidBookStatus)
            }
        } else {
            Err(LibraryErrors::BookNotFound)
        }
    }

    /// This function borrows a book from the library, changes the book's status to BOOK_TAKEN
    pub fn borrow_book(&mut self, index: u32) -> Result<(), LibraryErrors> {
        return self.change_book_availability(index, BOOK_TAKEN);
    }

    /// This function returns a book to the library, changes the book's status to BOOK_NOT_TAKEN
    pub fn return_book(&mut self, index: u32) -> Result<(), LibraryErrors> {
        return self.change_book_availability(index, BOOK_NOT_TAKEN);
    }

    /// This function returns a book by its index, if it exists
    pub fn get_book_by_index(&self, index: u32) -> Option<&Book> {
        self.books.get(index as usize)
    }

    /// This function returns a mutable reference to a book by its index, if it exists
    fn get_mut_book_by_index(&mut self, index: u32) -> Option<&mut Book> {
        self.books.get_mut(index as usize)
    }
}

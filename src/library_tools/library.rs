use crate::library_tools::book::{BOOK_NOT_TAKEN, BOOK_TAKEN, Book};

pub enum LibraryErrors {
    BookNotFound,
    InvalidBookStatus,
}

pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    // Constructor for the library struct
    // Initializes an empty library
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    /// print all books' data
    pub fn list_all_books(&self) {
        for (index, book) in self.books.iter().enumerate() {
            print!("{}. ", index);
            book.print_book();
        }
    }

    /// Adds a new book to the library
    pub fn add_new_book(&mut self, book: Book) {
        // I could use the next line if I would to run this code on limited memory machine
        // books.try_reserve(1)?;
        self.books.push(book);
    }

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

    pub fn borrow_book(&mut self, index: u32) -> Result<(), LibraryErrors> {
        return self.change_book_availability(index, BOOK_TAKEN);
    }

    pub fn return_book(&mut self, index: u32) -> Result<(), LibraryErrors> {
        return self.change_book_availability(index, BOOK_NOT_TAKEN);
    }

    pub fn get_book_by_index(&self, index: u32) -> Option<&Book> {
        self.books.get(index as usize)
    }

    fn get_mut_book_by_index(&mut self, index: u32) -> Option<&mut Book> {
        self.books.get_mut(index as usize)
    }
}

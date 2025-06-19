use crate::library_tools::library::{Library, LibraryErrors};
use crate::library_tools::library_book::LibraryBook;
use std::io::stdin;

/// Enum representing the options available in the library manager menu
enum LibraryOptions {
    AddNewBook = 1,
    BorrowBook = 2,
    ReturnBook = 3,
    ListAllBooks = 4,
    PrintBook = 5,
    Exit = 6,
    InvalidChoice,
}

/// Converts a u32 number to a LibraryOptions enum variant
fn convert_u32_to_library_option(num: u32) -> LibraryOptions {
    match num {
        1 => LibraryOptions::AddNewBook,
        2 => LibraryOptions::BorrowBook,
        3 => LibraryOptions::ReturnBook,
        4 => LibraryOptions::ListAllBooks,
        5 => LibraryOptions::PrintBook,
        6 => LibraryOptions::Exit,
        _ => LibraryOptions::InvalidChoice,
    }
}

/// LibraryManager struct that manages the library operations
/// It contains a Library instance and methods to interact with it
pub struct LibraryManager {
    pub library: Library,
}

impl LibraryManager {
    /// Constructor for the LibraryManager struct
    /// Initializes a new LibraryManager with an empty Library
    pub fn new() -> Self {
        Self {
            library: Library::new(),
        }
    }

    /// This function prints the menu options for the library manager
    pub fn print_menu(&self) {
        println!("Library Manager Menu:");
        println!("{}. Add a new book", LibraryOptions::AddNewBook as u32);
        println!("{}. Borrow a book", LibraryOptions::BorrowBook as u32);
        println!("{}. Return a book", LibraryOptions::ReturnBook as u32);
        println!("{}. List all books", LibraryOptions::ListAllBooks as u32);
        println!(
            "{}. Print a specific book",
            LibraryOptions::PrintBook as u32
        );
        println!("{}. Exit", LibraryOptions::Exit as u32);
    }

    /// This function manages the library operations in a loop,
    /// allowing the user to choose options from the menu until they decide to exit
    pub fn manage_library(&mut self) {
        loop {
            self.print_menu();

            let user_choice = get_u32_from_user();
            let user_choice = convert_u32_to_library_option(user_choice);

            match user_choice {
                LibraryOptions::AddNewBook => self.add_book_to_library(),
                LibraryOptions::BorrowBook => self.borrow_book_from_library(),
                LibraryOptions::ReturnBook => self.return_book_to_library(),
                LibraryOptions::ListAllBooks => self.list_all_books_in_library(),
                LibraryOptions::PrintBook => self.print_specific_book_from_library(),
                LibraryOptions::Exit => break,
                LibraryOptions::InvalidChoice => println!("Invalid choice, please try again"),
            }
        }
        println!("~Goodbye~");
    }

    /// This funciton adds a new book to the library, with data provided by the user
    fn add_book_to_library(&mut self) {
        println!("Enter book name:");
        let book_name = get_string_from_user();
        println!("Enter author name:");
        let author_name = get_string_from_user();

        self.library
            .add_new_book(LibraryBook::new(book_name, author_name));
    }

    /// This function borrows a book from the library
    fn borrow_book_from_library(&mut self) {
        println!("Enter book name to borrow:");
        let book_name = get_string_from_user();
        let action_status = self.library.borrow_book(&book_name);
        match action_status {
            Ok(_) => println!("Succesfully borrowed book!"),
            Err(LibraryErrors::BookNotFound) => println!("No such book!"),
            Err(LibraryErrors::InvalidBookStatus) => println!("Book is already borrowed!"),
        }
    }

    /// This function returns a book to the library
    fn return_book_to_library(&mut self) {
        println!("Enter book name to return:");
        let book_name = get_string_from_user();
        let action_status = self.library.return_book(&book_name);
        match action_status {
            Ok(_) => println!("Succesfully returned book!"),
            Err(LibraryErrors::BookNotFound) => println!("No such book!"),
            Err(LibraryErrors::InvalidBookStatus) => println!("Book was not borrowed!"),
        }
    }

    /// This function lists all books in the library
    fn list_all_books_in_library(&self) {
        println!("{}", self.library);
    }

    /// This function prints a specific book from the library (by index)
    fn print_specific_book_from_library(&self) {
        println!("Enter book index to print:");
        let book_index = get_u32_from_user();
        if let Some(book) = self.library.get_book_by_index(book_index) {
            print!("{}. ", book_index);
            println!("{book}");
        } else {
            println!("No book found at index {}", book_index);
        }
    }
}

/// might be worth to move these two function to a separate file, but I'm too tired already
/// Reads a string input from the user
fn get_string_from_user() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

/// Reads a u32 input from the user
fn get_u32_from_user() -> u32 {
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(num) => return num,
                Err(_) => println!("please enter a valid number"),
            },
            Err(_) => println!("please enter a valid number"),
        }
    }
}

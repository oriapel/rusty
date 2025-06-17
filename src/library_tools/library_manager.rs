use crate::library_tools::book::{BOOK_NOT_TAKEN, BOOK_TAKEN, Book};
use crate::library_tools::library::{Library, LibraryErrors};
use std::io::stdin;

pub enum LibraryOptions {
    AddNewBook,
    BorrowBook,
    ReturnBook,
    ListAllBooks,
    PrintBook,
    Exit,
    InvalidChoice,
}

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

fn get_string_from_user() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

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

pub struct LibraryManager {
    pub library: Library,
}

impl LibraryManager {
    pub fn new() -> Self {
        Self {
            library: Library::new(),
        }
    }

    fn add_book_to_library(&mut self) {
        println!("Enter book name:");
        let book_name = get_string_from_user();
        println!("Enter author name:");
        let author_name = get_string_from_user();

        self.library.add_new_book(Book::new(book_name, author_name));
    }

    fn borrow_book_from_library(&mut self) {
        println!("Enter book index to borrow:");
        let book_index = get_u32_from_user();
        let action_status = self.library.borrow_book(book_index);
        match action_status {
            Ok(_) => println!("Succesfully borrowed book!"),
            Err(LibraryErrors::BookNotFound) => println!("No such book!"),
            Err(LibraryErrors::InvalidBookStatus) => println!("Book is already borrowed!"),
        }
    }

    fn return_book_to_library(&mut self) {
        println!("Enter book index to return:");
        let book_index = get_u32_from_user();
        let action_status = self.library.return_book(book_index);
        match action_status {
            Ok(_) => println!("Succesfully returned book!"),
            Err(LibraryErrors::BookNotFound) => println!("No such book!"),
            Err(LibraryErrors::InvalidBookStatus) => println!("Book was not borrowed!"),
        }
    }

    fn list_all_books_in_library(&self) {
        self.library.list_all_books()
    }

    fn print_specific_book_from_library(&self) {
        println!("Enter book index to print:");
        let book_index = get_u32_from_user();
        if let Some(book) = self.library.get_book_by_index(book_index) {
            print!("{}. ", book_index);
            book.print_book();
        } else {
            println!("No book found at index {}", book_index);
        }
    }

    pub fn print_menu(&self) {
        println!("Library Manager Menu:");
        println!("1. Add a new book");
        println!("2. Borrow a book");
        println!("3. Return a book");
        println!("4. List all books");
        println!("5. Print a specific book");
        println!("6. Exit");
    }

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

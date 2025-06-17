mod library_tools;
use crate::library_tools::book::{BOOK_NOT_TAKEN, BOOK_TAKEN, Book};
use crate::library_tools::library::{Library, LibraryErrors};
use crate::library_tools::library_manager::{LibraryManager, LibraryOptions};

use std::io::stdin;

fn main() {
    let mut library_manager = LibraryManager::new();
    library_manager.manage_library();
}

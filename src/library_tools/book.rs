pub const BOOK_TAKEN: bool = false;
pub const BOOK_NOT_TAKEN: bool = true;

pub struct Book {
    pub title: String,
    pub author: String,
    pub status: bool,
}

impl Book {
    pub fn new(book_title: String, book_author: String) -> Self {
        Self {
            title: book_title.to_string(),
            author: book_author.to_string(),
            status: BOOK_NOT_TAKEN,
        }
    }

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

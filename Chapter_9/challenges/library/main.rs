#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

pub struct Library {
    books: Vec<Book>,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author && self.year == other.year
    }
}

impl Library {
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn remove_book(&mut self, title: &str) -> Result<(), String> {
        let index = self.books.iter().position(|b| b.title == title);

        match index {
            Some(i) => {
                self.books.remove(i);
                Ok(())
            },
            None => Err(format!("Book with the title '{}' not found.", title)),
        }
    }

    pub fn get_books_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    pub fn get_book_by_title(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.title == title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        // arrange
        // act
        let library = Library::new();

        // assert
        assert_eq!(library.books.len(), 0);
    }

    #[test]
    fn add_book_to_library() {
        // arrange
        let book = Book {
            author: "Stephen King".to_string(),
            title: "It".to_string(),
            year: 1986
        };
        let mut library = Library::new();

        // act
        library.add_book(book);

        // assert
        assert_eq!(library.books.len(), 1);
        assert_eq!(library.books[0].author, "Stephen King".to_string());
        assert_eq!(library.books[0].title, "It".to_string());
        assert_eq!(library.books[0].year, 1986);
    }

    #[test]
    fn remove_book_not_existing() {
        // arrange
        let mut library = Library::new();

        // act
        let result = library.remove_book("It");

        // assert
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn remove_book_existing() {
        // arrange
        let mut library = Library::new();
        library.add_book(Book {
            author: "Stephen King".to_string(),
            title: "It".to_string(),
            year: 1986
        });

        // act
        let result = library.remove_book("It");

        // assert
        assert_eq!(library.books.len(), 0);
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn get_books_by_auther_no_books() {
        // arrange
        let library = Library::new();

        // act
        let result = library.get_books_by_author("Stephen King");

        // assert
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn get_books_by_auther_existing_books() {
        // arrange
        let mut library = Library::new();
        library.add_book(Book {
            author: "Stephen King".to_string(),
            title: "It".to_string(),
            year: 1986
        });

        // act
        let result = library.get_books_by_author("Stephen King");

        // assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].author, "Stephen King".to_string());
        assert_eq!(result[0].title, "It".to_string());
        assert_eq!(result[0].year, 1986);
    }

    #[test]
    fn get_book_not_existing() {
        // arrange
        let library = Library::new();

        // act
        let result = library.get_book_by_title("It");

        // assert
        assert_eq!(result.is_none(), true)
    }

    #[test]
    fn get_book_existing() {
        // arrange
        let mut library = Library::new();
        library.add_book(Book {
            author: "Stephen King".to_string(),
            title: "It".to_string(),
            year: 1986
        });

        // act
        let result = library.get_book_by_title("It");

        // assert
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().author, "Stephen King".to_string());
        assert_eq!(result.unwrap().title, "It".to_string());
        assert_eq!(result.unwrap().year, 1986);
    }

    #[test]
    fn add_book_to_library_twice() {
        // arrange
        let mut library = Library::new();

        // act
        library.add_book(Book {
            author: "Stephen King".to_string(),
            title: "It".to_string(),
            year: 1986
        });
        library.add_book(Book {
            author: "Stephen King".to_string(),
            title: "It".to_string(),
            year: 1986
        });

        // assert
        assert_eq!(library.books.len(), 2);
        assert_eq!(library.books[0].author, "Stephen King".to_string());
        assert_eq!(library.books[0].title, "It".to_string());
        assert_eq!(library.books[0].year, 1986);
        assert_eq!(library.books[1].author, "Stephen King".to_string());
        assert_eq!(library.books[1].title, "It".to_string());
        assert_eq!(library.books[1].year, 1986);
    }
    
    #[test]
    fn add_book_to_library_case_insensitive() {
        // arrange
        let mut library = Library::new();

        // act
        library.add_book(Book {
            author: "Stephen King".to_string(),
            title: "It".to_string(),
            year: 1986
        });
        library.add_book(Book {
            author: "Stephen King".to_string(),
            title: "it".to_string(),
            year: 1986
        });

        // assert
        assert_eq!(library.books.len(), 2);
        assert_eq!(library.books[0].title, "It".to_string());
        assert_eq!(library.books[1].title, "it".to_string());
    }
}

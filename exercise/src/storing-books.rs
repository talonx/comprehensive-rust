struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Notice how the `self` parameter
// changes type to indicate the method's required level of ownership
// over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.len() == 0
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        let iter = self.books.iter();
        for b in iter {
            println!("{} - {}", b.title, b.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let iter = self.books.iter();
        let mut oldest: Option<&Book> = None;
        for b in iter {
            if oldest.is_none() || b.year < oldest.unwrap().year {
                oldest = Some(b);
            }
        }
        oldest
    }
}

fn main() {
    let mut library = Library::new();

    println!(
        "The library is empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!(
        "The library is no longer empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}

#[derive(Debug)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book) => Some(book + " is found"),
            None => None,
        }
    }
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("TRUST");
    my_library.add_book("PEAK");
    my_library.add_book("PC");
    my_library.add_book("SPARK");

    for item in my_library{
        println!("{}", item);
    }
}

// Lifetimes werden auch gebraucht, wenn Strukturen Referenzen enthalten.

struct Book<'a> {
    //     ^^^^ Lifetime-Annotation hinzugefügt
    author: &'a str,
    //       ^^
    title: &'a str,
    //      ^^
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}

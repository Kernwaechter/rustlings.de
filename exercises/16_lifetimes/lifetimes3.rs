// Lifetimes werden auch gebraucht, wenn Strukturen Referenzen enthalten.

// TODO: Korrigiere die Compiler-Fehler bei dieser Struktur.
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}

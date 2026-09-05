fn main() {
    let number = "T-H-R-E-E";
    println!("Buchstabiere eine Zahl: {number}");

    // Mittels Variablen-Shadowing
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let number = 3;
    println!("Zahl plus zwei ist: {}", number + 2);
}

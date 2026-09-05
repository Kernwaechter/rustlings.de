fn main() {
    // Der einfachste Weg, den Compiler-Fehler zu beheben, ist, die Variable
    // `x` zu initialisieren. Wenn du ihren Wert auf eine Ganzzahl setzt,
    // leitet Rust ihren Typ als `i32` ab — das ist der Standardtyp für
    // Ganzzahlen.
    let x = 42;

    // Wir können aber auch einen anderen Typ als den Standard `i32`
    // erzwingen, indem wir eine Typ-Annotation hinzufügen:
    // let x: u8 = 42;

    if x == 10 {
        println!("x ist zehn!");
    } else {
        println!("x ist nicht zehn!");
    }
}

#![allow(clippy::needless_late_init)]

fn main() {
    // Das Lesen von Variablen ohne zugewiesenen Wert (uninitialisiert) ist in Rust nicht erlaubt!
    // Deshalb müssen wir zuerst einen Wert zuweisen.
    let x: i32 = 42;

    println!("Zahl {x}");

    // Es ist möglich, eine Variable anzulegen (zu deklarieren) und ihr erst
    // später einen Wert zu geben (sie zu initialisieren). Bevor sie einen
    // Wert bekommen hat, kann sie aber nicht benutzt werden.
    let y: i32;
    y = 42;
    println!("Zahl {y}");
}

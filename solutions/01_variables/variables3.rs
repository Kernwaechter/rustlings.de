#![allow(clippy::needless_late_init)]

fn main() {
    // Das Lesen uninitialisierter Variablen ist in Rust nicht erlaubt!
    // Deshalb müssen wir zuerst einen Wert zuweisen.
    let x: i32 = 42;

    println!("Zahl {x}");

    // Es ist möglich, eine Variable zu deklarieren und sie erst später zu
    // initialisieren. Sie kann aber vor der Initialisierung nicht benutzt
    // werden.
    let y: i32;
    y = 42;
    println!("Zahl {y}");
}

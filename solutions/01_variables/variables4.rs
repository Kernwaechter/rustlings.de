fn main() {
    // In Rust sind Variablen standardmäßig unveränderlich (immutable).
    // Das Schlüsselwort `mut` nach `let` macht die zuvor angelegte Variable
    // (deklarierte) veränderlich.
    let mut x = 3;
    println!("Zahl {x}");

    x = 5;
    println!("Zahl {x}");
}

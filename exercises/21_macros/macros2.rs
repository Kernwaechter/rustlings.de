fn main() {
    my_macro!();
}

// TODO: Korrigiere den Compiler-Fehler, indem du die gesamte Definition dieses
// Makros verschiebst.
macro_rules! my_macro {
    () => {
        println!("Schau dir mein Makro an!");
    };
}

// TODO: Behebe den Compiler-Fehler, ohne die Makrodefinition aus diesem
// Modul herauszunehmen.
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Schau dir mein Makro an!");
        };
    }
}

fn main() {
    my_macro!();
}

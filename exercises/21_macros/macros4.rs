// TODO: Korrigiere den Compiler-Fehler, indem du ein oder zwei Zeichen
// hinzufügst.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Schau dir mein Makro an!");
    }
    ($val:expr) => {
        println!("Schau dir dieses andere Makro an: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}

// Das Attribut `macro_use` hinzugefügt.
#[macro_use]
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

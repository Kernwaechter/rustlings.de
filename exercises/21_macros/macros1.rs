// Ein Makro ist Code, der zur Kompilierzeit anderen Code erzeugt. Du
// erkennst einen Makroaufruf am Ausrufezeichen dahinter, z. B.
// `println!(...)`.

macro_rules! my_macro {
    () => {
        println!("Schau dir mein Makro an!");
    };
}

fn main() {
    // TODO: Korrigiere den Makroaufruf.
    my_macro();
}

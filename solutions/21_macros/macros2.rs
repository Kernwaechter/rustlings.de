// Die Makrodefinition vor ihren Aufruf verschoben.
macro_rules! my_macro {
    () => {
        println!("Schau dir mein Makro an!");
    };
}

fn main() {
    my_macro!();
}

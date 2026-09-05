// Ändere diese Funktion nicht.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Behebe den Compiler-Fehler, indem du eine Zeile verschiebst.

    let string1 = String::from("langer String ist lang");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("Der längere String ist '{result}'");
}

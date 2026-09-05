// TODO: Korrigiere den Compiler-Fehler in der `main`-Funktion, ohne diese Funktion zu ändern.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "grün" || attempt == "blau" || attempt == "rot"
}

fn main() {
    let word = String::from("grün"); // Diese Zeile nicht ändern.

    if is_a_color_word(word) {
        println!("Das ist ein Farbwort, das ich kenne!");
    } else {
        println!("Das ist kein Farbwort, das ich kenne.");
    }
}

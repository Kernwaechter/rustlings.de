// TODO: Korrigiere den Compiler-Fehler, ohne die Funktionssignatur zu ändern.
fn current_favorite_color() -> String {
    "blau"
}

fn main() {
    let answer = current_favorite_color();
    println!("Meine aktuelle Lieblingsfarbe ist {answer}");
}

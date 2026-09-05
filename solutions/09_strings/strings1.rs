fn current_favorite_color() -> String {
    // Äquivalent zu `String::from("blau")`
    "blau".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("Meine aktuelle Lieblingsfarbe ist {answer}");
}

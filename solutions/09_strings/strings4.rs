fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blau");

    string("rot".to_string());

    string(String::from("hi"));

    string("rust macht Spaß!".to_owned());

    string(format!("Interpolations{}", "station"));

    // ACHTUNG: Das ist Byte-Indizierung, keine Zeichen-Indizierung.
    // Zeichen-Indizierung geht mit `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hallo zusammen ".trim());

    string("Guten Montag!".replace("Mon", "Diens"));

    string("mEiNe UmSchAlTtAsTe KlEmMt".to_lowercase());
}

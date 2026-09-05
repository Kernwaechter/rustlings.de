// Aufrufe dieser Funktion sollen durch Aufrufe von `string_slice` oder `string` ersetzt werden.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Hier ist eine Reihe von Werten — manche sind `String`, manche `&str`.
// Deine Aufgabe ist es, `placeholder(…)` durch entweder `string_slice(…)`
// oder `string(…)` zu ersetzen, je nachdem, was du für jeden Wert für richtig hältst.
fn main() {
    placeholder("blau");

    placeholder("rot".to_string());

    placeholder(String::from("hi"));

    placeholder("rust macht Spaß!".to_owned());

    placeholder(format!("Interpolations{}", "station"));

    // ACHTUNG: Das ist Byte-Indizierung, keine Zeichen-Indizierung.
    // Zeichen-Indizierung geht mit `s.chars().nth(INDEX)`.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hallo zusammen ".trim());

    placeholder("Guten Montag!".replace("Mon", "Diens"));

    placeholder("mEiNe UmSchAlTtAsTe KlEmMt".to_lowercase());
}

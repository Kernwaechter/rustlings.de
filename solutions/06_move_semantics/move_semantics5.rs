#![allow(clippy::ptr_arg)]

// Leiht sich die Daten aus, statt den Besitz zu übernehmen.
// Eigentlich wird hier empfohlen, `&str` statt `&String` zu verwenden. Aber
// das reicht erstmal, weil wir Strings noch nicht behandelt haben.
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Übernimmt Besitz, statt sich die Daten nur auszuleihen.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust ist toll!".to_string();

    get_char(&data);

    string_uppercase(data);
}

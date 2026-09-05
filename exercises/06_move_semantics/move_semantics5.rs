#![allow(clippy::ptr_arg)]

// TODO: Korrigiere die Compiler-Fehler, ohne etwas anderes zu ändern außer dem
// Hinzufügen oder Entfernen von Referenzen (dem Zeichen `&`).

// Sollte keinen Besitz übernehmen
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Sollte Besitz übernehmen
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust ist toll!".to_string();

    get_char(data);

    string_uppercase(&data);
}

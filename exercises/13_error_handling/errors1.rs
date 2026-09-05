// TODO: Diese Funktion verweigert die Erzeugung von Text für ein Namensschild,
// wenn du ihr einen leeren String übergibst. Es wäre schöner, wenn sie
// erklären würde, was das Problem war, statt einfach `None` zurückzugeben.
// Zum Glück hat Rust eine ähnliche Konstruktion wie `Option`, mit der sich
// Fehlerbedingungen ausdrücken lassen. Ändere die Funktionssignatur und den
// Funktionsrumpf so, dass `Result<String, String>` statt `Option<String>`
// zurückgegeben wird.
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Leere Namen sind nicht erlaubt
        None
    } else {
        Some(format!("Hallo! Mein Name ist {name}"))
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hallo! Mein Name ist Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Leere Namen sind nicht erlaubt"),
        );
    }
}

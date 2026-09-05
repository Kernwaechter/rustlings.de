fn generate_nametag_text(name: String) -> Result<String, String> {
    //                                    ^^^^^^         ^^^^^^
    if name.is_empty() {
        // `Err(String)` statt `None`.
        Err("Leere Namen sind nicht erlaubt".to_string())
    } else {
        // `Ok` statt `Some`.
        Ok(format!("Hallo! Mein Name ist {name}"))
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

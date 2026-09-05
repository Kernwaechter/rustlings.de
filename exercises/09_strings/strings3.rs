fn trim_me(input: &str) -> &str {
    // TODO: Entferne Leerzeichen von beiden Enden eines Strings.
}

fn compose_me(input: &str) -> String {
    // TODO: Füge " Welt!" an den String an! Das geht auf mehrere Arten.
}

fn replace_me(input: &str) -> String {
    // TODO: Ersetze "Autos" im String durch "Luftballons".
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hallo!     "), "Hallo!");
        assert_eq!(trim_me("  Was geht?"), "Was geht?");
        assert_eq!(trim_me("   Servus!  "), "Servus!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hallo"), "Hallo Welt!");
        assert_eq!(compose_me("Tschüss"), "Tschüss Welt!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("Ich finde Autos cool"),
            "Ich finde Luftballons cool",
        );
        assert_eq!(
            replace_me("Ich schaue mir gerne Autos an"),
            "Ich schaue mir gerne Luftballons an",
        );
    }
}

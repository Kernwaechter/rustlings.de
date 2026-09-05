// TODO: Behebe den Compiler-Fehler in dieser Funktion.
fn picky_eater(food: &str) -> &str {
    if food == "Erdbeere" {
        "Lecker!"
    } else {
        1
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

// TODO: Lies die Tests, um das gewünschte Verhalten zu verstehen.
// Bring alle Tests zum Bestehen, ohne sie zu verändern.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // Das bedeutet: `picky_eater` mit dem Argument "Erdbeere" aufzurufen, soll "Lecker!" zurückgeben.
        assert_eq!(picky_eater("Erdbeere"), "Lecker!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("Kartoffel"), "Kann ich wohl essen.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("Brokkoli"), "Nein danke!");
        assert_eq!(picky_eater("Gummibärchen"), "Nein danke!");
        assert_eq!(picky_eater("buchstäblich alles"), "Nein danke!");
    }
}

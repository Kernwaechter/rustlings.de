fn picky_eater(food: &str) -> &str {
    if food == "Erdbeere" {
        "Lecker!"
    } else if food == "Kartoffel" {
        "Kann ich wohl essen."
    } else {
        "Nein danke!"
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
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

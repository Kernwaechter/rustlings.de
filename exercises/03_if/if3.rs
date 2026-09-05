fn animal_habitat(animal: &str) -> &str {
    // TODO: Behebe den Compiler-Fehler in der Anweisung unten.
    let identifier = if animal == "Krabbe" {
        1
    } else if animal == "Erdhörnchen" {
        2.0
    } else if animal == "Schlange" {
        3
    } else {
        "Unbekannt"
    };

    // Ändere den Ausdruck unten nicht!
    if identifier == 1 {
        "Strand"
    } else if identifier == 2 {
        "Bau"
    } else if identifier == 3 {
        "Wüste"
    } else {
        "Unbekannt"
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

// Ändere die Tests nicht!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("Erdhörnchen"), "Bau")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("Schlange"), "Wüste")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("Krabbe"), "Strand")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("Dinosaurier"), "Unbekannt")
    }
}

fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "Krabbe" {
        1
    } else if animal == "Erdhörnchen" {
        // Ganzzahl, damit jeder Zweig denselben Typ hat.
        2
    } else if animal == "Schlange" {
        3
    } else {
        // Ein beliebiger, unbenutzter Bezeichner.
        4
    };

    // Statt so eines Bezeichners würde man in Rust ein Enum verwenden.
    // Aber Enums kennen wir noch nicht.
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

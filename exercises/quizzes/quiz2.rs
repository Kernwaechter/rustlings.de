// Das ist ein Quiz zu den folgenden Abschnitten:
// - Strings
// - Vecs
// - Move-Semantik
// - Module
// - Enums
//
// Lass uns eine kleine Maschine in Form einer Funktion bauen. Als Eingabe
// geben wir eine Liste von Strings und Befehlen. Diese Befehle legen fest,
// welche Aktion mit dem String passiert. Das kann eines von diesen sein:
// - Den String in Großbuchstaben umwandeln
// - Den String trimmen
// - "bar" eine festgelegte Anzahl von Malen an den String anhängen
//
// Genauer gesagt:
// - Die Eingabe ist ein Vektor aus Tupeln mit je zwei Elementen. Das erste
//   Element ist der String, das zweite der Befehl.
// - Die Ausgabe ist ein Vektor von Strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Vervollständige die Funktion wie oben beschrieben.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    // TODO: Was müssen wir importieren, um `transformer` in den Scope zu
    // holen?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hallo".to_string(), Command::Uppercase),
            (" alle wege führen nach rom! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HALLO",
                "alle wege führen nach rom!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

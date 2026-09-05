// Lass uns eine kleine Maschine in Form einer Funktion bauen. Als Eingabe
// geben wir eine Liste von Strings und Befehlen. Diese Befehle bestimmen,
// welche Aktion auf den String angewendet wird. Es kann entweder sein:
// - Den String in Großbuchstaben umwandeln
// - Den String trimmen
// - "bar" eine festgelegte Anzahl von Malen an den String anhängen
//
// Die genaue Form davon wird sein:
// - Die Eingabe ist ein Vektor aus 2-elementigen Tupeln, wobei das erste
//   Element der String ist und das zweite der Befehl.
// - Das Ausgabeelement ist ein Vektor von Strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // Die Lösung mit einer Schleife. Schau dir `transformer_iter` für eine
    // Version mit Iteratoren an.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::new();

        for (string, command) in input {
            // Erzeuge den neuen String.
            let new_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            };

            // Schiebe den neuen String in den Ausgabevektor.
            output.push(new_string);
        }

        output
    }

    // Entspricht `transform`, benutzt aber zum Vergleich einen Iterator
    // statt einer Schleife. Keine Sorge, wir üben Iteratoren später ;)
    pub fn transformer_iter(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(string, command)| match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            })
            .collect()
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    // `transformer` importieren.
    use super::my_module::transformer;

    use super::Command;
    use super::my_module::transformer_iter;

    #[test]
    fn it_works() {
        for transformer in [transformer, transformer_iter] {
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
}

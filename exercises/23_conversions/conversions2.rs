// Der Trait `From` wird für Wert-zu-Wert-Umwandlungen benutzt. Wenn `From`
// implementiert ist, wird automatisch auch eine Implementierung von `Into`
// bereitgestellt. Mehr dazu kannst du in der Dokumentation lesen:
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// Maßeinheiten mit eigenen Typen darzustellen, ist gängige Praxis. Es
// verhindert, dass Werte unterschiedlicher Maßeinheiten versehentlich
// vermischt werden.

struct Celsius(f64);

struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    // TODO: Wandle Celsius in Fahrenheit um. Mach dir keine Sorgen um die
    // Fließkomma-Präzision. Die Formel lautet: F = C * 1.8 + 32
}

impl From<Fahrenheit> for Celsius {
    // TODO: Wandle Fahrenheit in Celsius um.
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(f64, f64); 6] = [
        (-50.0, -58.0),
        (0.0, 32.0),
        (20.0, 68.0),
        (100.0, 212.0),
        (400.0, 752.0),
        (1000.0, 1832.0),
    ];

    #[test]
    fn celsius_to_fahrenheit() {
        for (celsius, fahrenheit) in CASES {
            let Fahrenheit(actual) = Celsius(celsius).into();
            assert_eq!(actual.round(), fahrenheit);
        }
    }

    #[test]
    fn fahrenheit_to_celsius() {
        for (celsius, fahrenheit) in CASES {
            let Celsius(actual) = Fahrenheit(fahrenheit).into();
            assert_eq!(actual.round(), celsius);
        }
    }
}

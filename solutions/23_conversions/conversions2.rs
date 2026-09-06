// Der Trait `From` wird für Wert-zu-Wert-Umwandlungen benutzt.
// Implementierst du `From`, bekommst du automatisch auch eine
// Implementierung von `Into` dazu. Mehr dazu kannst du in der
// Dokumentation lesen:
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// Es ist gängige Praxis, für verschiedene Maßeinheiten eigene Typen zu
// benutzen. Das verhindert, dass du aus Versehen Werte unterschiedlicher
// Maßeinheiten miteinander vermischst.

struct Celsius(f64);

struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(Celsius(celsius): Celsius) -> Self {
        Fahrenheit(celsius * 1.8 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(Fahrenheit(fahrenheit): Fahrenheit) -> Self {
        Celsius((fahrenheit - 32.0) / 1.8)
    }
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

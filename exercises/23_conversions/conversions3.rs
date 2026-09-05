// In dieser Übung implementieren wir `FromStr`, um Daten, die als String
// gespeichert sind, in einen strukturierten Typ umzuwandeln. Anders als
// beim Trait `From` kann die von `FromStr` ausgedrückte Umwandlung
// fehlschlagen, daher gibt sie ein `Result` zurück. Außerdem kannst du
// nach der Implementierung von `FromStr` die Methode `parse` auf Strings
// benutzen, um ein Objekt des implementierenden Typs zu erzeugen. Mehr
// dazu kannst du in der Dokumentation lesen:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// Wir benutzen diesen Fehlertyp für die `FromStr`-Implementierung.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Falsche Anzahl an Feldern
    BadLen,
    // Leeres Namensfeld
    NoName,
    // Umschlossener Fehler von parse::<u8>()
    ParseInt(ParseIntError),
}

// TODO: Vervollständige diese `FromStr`-Implementierung, um eine `Person`
// aus einem String der Form "Mark,20" parsen zu können.
// Beachte, dass du den Alters-Bestandteil mit etwas wie
// `"4".parse::<u8>()` in ein `u8` parsen musst.
//
// Schritte:
// 1. Teile den gegebenen String an den darin enthaltenen Kommas.
// 2. Falls die Teilung weniger oder mehr als 2 Elemente liefert, gib den
//    Fehler `ParsePersonError::BadLen` zurück.
// 3. Benutze das erste Element aus der Teilung als Namen.
// 4. Falls der Name leer ist, gib den Fehler `ParsePersonError::NoName`
//    zurück.
// 5. Parse das zweite Element aus der Teilung als Alter in ein `u8`.
// 6. Falls das Parsen des Alters fehlschlägt, gib den Fehler
//    `ParsePersonError::ParseInt` zurück.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}

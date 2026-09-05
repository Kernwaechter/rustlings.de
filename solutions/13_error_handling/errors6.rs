// Fang-alles-Fehlertypen wie `Box<dyn Error>` zu benutzen, wird für
// Bibliothekscode nicht empfohlen, bei dem Aufrufer eventuell anhand des
// Fehlerinhalts Entscheidungen treffen wollen, statt ihn nur auszugeben oder
// weiterzureichen. Hier definieren wir einen eigenen Fehlertyp, damit
// Aufrufer entscheiden können, was als Nächstes zu tun ist, wenn unsere
// Funktion einen Fehler zurückgibt.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Ein eigener Fehlertyp, den wir in `PositiveNonzeroInteger::parse` benutzen.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

// Als alternative Lösung erlaubt die Implementierung des Traits `From` die
// automatische Umwandlung eines `ParseIntError` in einen
// `ParsePosNonzeroError` mithilfe des `?`-Operators, ohne `map_err` aufrufen
// zu müssen.
//
// ```
// let x: i64 = s.parse()?;
// ```
//
// Traits wie `From` werden in späteren Übungen behandelt.
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(err: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // Gib einen passenden Fehler zurück, statt in Panik zu geraten, wenn
        // `parse()` einen Fehler zurückgibt.
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::ParseInt)?;
        //                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        Self::new(x).map_err(ParsePosNonzeroError::Creation)
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("keine Zahl"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}

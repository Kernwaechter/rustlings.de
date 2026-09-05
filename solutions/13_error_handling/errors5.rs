// Diese Übung ist eine abgewandelte Version der Übung `errors4`. Sie
// verwendet ein paar Konzepte, die wir erst später im Kurs behandeln, wie
// `Box` und den `From`-Trait. Es ist jetzt nicht wichtig, sie im Detail zu
// verstehen, aber du kannst gerne vorauslesen. Stell dir den Typ
// `Box<dyn ???>` fürs Erste einfach als „ich will irgendetwas, das ??? kann“
// vor.
//
// Kurz gesagt: Dieser Anwendungsfall für Boxen ist dafür da, wenn du einen
// Wert besitzen möchtest und dir nur wichtig ist, dass es sich um einen Typ
// handelt, der einen bestimmten Trait implementiert. Dazu wird die `Box` als
// Typ `Box<dyn Trait>` deklariert, wobei `Trait` der Trait ist, nach dem der
// Compiler bei jedem in diesem Kontext verwendeten Wert sucht. In dieser
// Übung sind das die möglichen Fehler, die ein `Result` zurückgeben kann.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Das ist nötig, damit `CreationError` den Trait `Error` implementieren kann.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "Zahl ist negativ",
            CreationError::Zero => "Zahl ist null",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("Ausgabe={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

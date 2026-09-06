// Diese Übung ist eine abgewandelte Version von `errors4`. Sie nutzt ein
// paar Konzepte, die erst später im Kurs drankommen: `Box` und den
// `From`-Trait. Du musst sie jetzt noch nicht im Detail verstehen. Wenn du
// magst, kannst du aber schon vorauslesen. Stell dir den Typ `Box<dyn ???>`
// fürs Erste einfach so vor: „Ich will irgendetwas, das ??? kann.“
//
// Kurz gesagt: Manchmal willst du einen Wert besitzen, und es ist dir egal,
// um welchen genauen Typ es sich handelt. Wichtig ist dir nur eins: Der Typ
// muss einen bestimmten Trait implementieren. Genau dafür ist eine `Box` da.
//
// Dafür legst du den Typ der `Box` so fest (deklarierst ihn): `Box<dyn
// Trait>`. `Trait` ist dabei der Trait, den der Compiler bei jedem
// passenden Wert erwartet. In dieser Übung sind das die möglichen Fehler,
// die ein `Result` zurückgeben kann.

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

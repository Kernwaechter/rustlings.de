// Structs enthalten Daten, können aber auch Logik haben. In dieser Übung
// haben wir das Struct `Fireworks` definiert sowie ein paar Funktionen, die
// damit arbeiten. Verwandle diese freistehenden Funktionen in Methoden und
// in Funktionen, die zum Typ gehören (assoziierte Funktionen), um diese
// Beziehung im Code klarer auszudrücken.

#![deny(clippy::use_self)] // üben, den Typ `Self` zu benutzen

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

// TODO: Verwandle diese Funktion in eine Funktion, die zum Typ `Fireworks` gehört (assoziierte Funktion).
fn new_fireworks() -> Fireworks {
    Fireworks { rockets: 0 }
}

// TODO: Verwandle diese Funktion in eine Methode von `Fireworks`.
fn add_rockets(fireworks: &mut Fireworks, rockets: usize) {
    fireworks.rockets += rockets
}

// TODO: Verwandle diese Funktion in eine Methode von `Fireworks`.
fn start(fireworks: Fireworks) -> String {
    "🚀".repeat(fireworks.rockets)
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_some_fireworks() {
        let f = Fireworks::new();
        assert_eq!(f.start(), "");

        let mut f = Fireworks::new();
        f.add_rockets(3);
        assert_eq!(f.start(), "🚀🚀🚀");

        let mut f = Fireworks::new();
        f.add_rockets(7);
        // Im letzten Test nutzen wir keine Methoden-Syntax, um sicherzustellen, dass
        // die Funktion `start` den Besitz von `fireworks` übernimmt.
        assert_eq!(Fireworks::start(f), "🚀🚀🚀🚀🚀🚀🚀");
    }
}

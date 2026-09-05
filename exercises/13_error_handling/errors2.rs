// Angenommen, wir schreiben ein Spiel, in dem man Gegenstände mit Münzen
// kaufen kann. Alle Gegenstände kosten 5 Münzen, und bei jedem Kauf fällt
// eine Bearbeitungsgebühr von 1 Münze an. Ein Spieler tippt ein, wie viele
// Gegenstände er kaufen möchte, und die Funktion `total_cost` berechnet die
// Gesamtkosten der Gegenstände. Da der Spieler die Menge eingetippt hat,
// bekommen wir sie als String. Er könnte alles Mögliche eingetippt haben,
// nicht nur Zahlen!
//
// Im Moment behandelt diese Funktion den Fehlerfall überhaupt nicht. Was wir
// erreichen wollen: Wenn wir die Funktion `total_cost` mit einem String
// aufrufen, der keine Zahl ist, gibt diese Funktion einen `ParseIntError`
// zurück. In diesem Fall wollen wir diesen Fehler sofort aus unserer
// Funktion zurückgeben, statt zu versuchen, zu multiplizieren und zu
// addieren.
//
// Es gibt mindestens zwei Wege, das korrekt umzusetzen. Aber einer davon ist
// deutlich kürzer!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Behandle den Fehlerfall wie oben beschrieben.
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}

// Angenommen, wir schreiben ein Spiel, in dem man Gegenstände mit Münzen
// kaufen kann. Alle Gegenstände kosten 5 Münzen. Bei jedem Kauf kommt noch
// eine Bearbeitungsgebühr von 1 Münze dazu.
//
// Ein Spieler tippt ein, wie viele Gegenstände er kaufen möchte. Die
// Funktion `total_cost` berechnet daraus die Gesamtkosten. Der Spieler
// tippt die Menge aber selbst ein — deshalb bekommen wir sie als String.
// Und ein String kann alles Mögliche enthalten, nicht nur Zahlen!
//
// Im Moment behandelt diese Funktion den Fehlerfall überhaupt nicht. Das
// wollen wir ändern: Rufen wir `total_cost` mit einem String auf, der
// keine Zahl ist, soll die Funktion einen `ParseIntError` zurückgeben.
// Diesen Fehler wollen wir dann sofort weiterreichen, statt trotzdem zu
// multiplizieren und zu addieren.
//
// Es gibt mindestens zwei Wege, das korrekt umzusetzen. Aber einer davon
// ist deutlich kürzer!

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

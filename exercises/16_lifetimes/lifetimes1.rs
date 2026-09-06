// Der Rust-Compiler muss prüfen können, ob eine Referenz gültig ist. Nur so
// kann er dich warnen, wenn eine Referenz zu früh ungültig wird (aus ihrem
// Gültigkeitsbereich fällt, dem Scope). Wie lange eine Referenz gültig
// bleibt, nennt man ihre Lifetime.
//
// Denk daran: Eine Referenz ist nur eine Ausleihe (Borrow). Sie besitzt
// ihre Daten nicht selbst, sondern leiht sie sich nur. Was passiert also,
// wenn der eigentliche Besitzer der Daten seinen Scope verlässt, bevor die
// Referenz benutzt wird?

// TODO: Korrigiere den Compiler-Fehler, indem du die Funktionssignatur
// aktualisierst.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}

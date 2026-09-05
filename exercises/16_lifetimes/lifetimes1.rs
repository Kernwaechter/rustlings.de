// Der Rust-Compiler muss wissen, wie er prüfen kann, ob übergebene
// Referenzen gültig sind, damit er dem Programmierer mitteilen kann, wenn
// eine Referenz Gefahr läuft, aus dem Gültigkeitsbereich (Scope) zu fallen,
// bevor sie benutzt wird. Denk daran: Referenzen sind Ausleihen (Borrows)
// und besitzen ihre Daten nicht selbst. Was, wenn ihr Besitzer den Scope
// verlässt?

// TODO: Behebe den Compiler-Fehler, indem du die Funktionssignatur
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

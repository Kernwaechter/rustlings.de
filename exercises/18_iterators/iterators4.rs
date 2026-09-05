fn factorial(num: u64) -> u64 {
    // TODO: Vervollständige diese Funktion, sodass sie die Fakultät von
    // `num` zurückgibt. Diese ist definiert als `1 * 2 * 3 * … * num`.
    // https://de.wikipedia.org/wiki/Fakultät_(Mathematik)
    //
    // Benutze nicht:
    // - frühzeitige Rückgaben (explizite Verwendung des Schlüsselworts
    //   `return`)
    // Versuche, Folgendes nicht zu benutzen:
    // - imperative Schleifen (for/while)
    // - zusätzliche Variablen
    // Für eine zusätzliche Herausforderung, benutze nicht:
    // - Rekursion
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}

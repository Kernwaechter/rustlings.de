// Mary kauft Äpfel. Der Preis eines Apfels wird wie folgt berechnet:
// - Ein Apfel kostet 2 Rustbucks.
// - Wenn Mary aber mehr als 40 Äpfel kauft, sinkt der Preis jedes Apfels
// in der gesamten Bestellung auf nur noch 1 Rustbuck!

fn calculate_price_of_apples(n_apples: u64) -> u64 {
    if n_apples > 40 {
        n_apples
    } else {
        2 * n_apples
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

// Ändere die Tests nicht!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}

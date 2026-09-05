// Berechnet die Potenz von 2 mithilfe einer Bit-Verschiebung.
// `1 << n` entspricht "2 hoch n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Teste die Funktion `power_of_2` mit ein paar Werten.
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}

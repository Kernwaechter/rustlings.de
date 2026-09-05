// Tests sind wichtig, um sicherzustellen, dass dein Code das tut, was du
// denkst, dass er tun sollte.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    // Beim Schreiben von Unit-Tests importiert man häufig alles aus dem
    // äußeren Modul (`super`) mit einem Wildcard.
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(is_even(0));
        assert!(!is_even(-1));
        //      ^ Du kannst `false` mit dem Negationsoperator `!` prüfen.
    }
}

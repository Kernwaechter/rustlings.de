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
    // TODO: Importiere `is_even`. Du kannst einen Wildcard benutzen, um
    // alles aus dem äußeren Modul zu importieren.

    #[test]
    fn you_can_assert() {
        // TODO: Teste die Funktion `is_even` mit ein paar Werten.
        assert!();
        assert!();
    }
}

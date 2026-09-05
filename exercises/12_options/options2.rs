fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Mach daraus eine if-let-Anweisung, deren Wert `Some` ist.
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Mach daraus eine while-let-Anweisung. Denk daran, dass
        // `Vec::pop()` eine weitere Schicht `Option` hinzufügt. Du kannst
        // verschachteltes Pattern-Matching in if-let- und
        // while-let-Anweisungen benutzen.
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

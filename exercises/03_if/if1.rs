fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Vervollständige diese Funktion, sodass sie die größere Zahl zurückgibt!
    // Sind beide Zahlen gleich, kann eine der beiden zurückgegeben werden.
    // Verwende nicht:
    // - einen weiteren Funktionsaufruf
    // - zusätzliche Variablen
}

fn main() {
    // Hier kannst du optional experimentieren.
}

// Kümmere dich vorerst nicht darum :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

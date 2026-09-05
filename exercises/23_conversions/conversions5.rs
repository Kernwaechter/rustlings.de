// AsRef und AsMut erlauben günstige Referenz-zu-Referenz-Umwandlungen.
// Mehr dazu unter https://doc.rust-lang.org/std/convert/trait.AsRef.html
// und https://doc.rust-lang.org/std/convert/trait.AsMut.html.

// Ermittle die Anzahl der Bytes (nicht Zeichen) im gegebenen Argument
// (`.len()` gibt die Anzahl der Bytes in einem String zurück).
// TODO: Füge den Trait `AsRef` passend als Trait-Bound hinzu.
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// Ermittle die Anzahl der Zeichen (nicht Bytes) im gegebenen Argument.
// TODO: Füge den Trait `AsRef` passend als Trait-Bound hinzu.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Quadriert eine Zahl mit `as_mut()`.
// TODO: Füge den passenden Trait-Bound hinzu.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implementiere den Funktionsrumpf.
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}

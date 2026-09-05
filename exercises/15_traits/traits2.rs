trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implementiere den Trait `AppendBar` für einen Vektor von Strings.
// `append_bar` soll den String "Bar" in den Vektor schieben.

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}

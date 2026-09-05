fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Tupel-Index-Syntax.
        let second = numbers.1;

        assert_eq!(second, 2, "Das ist nicht die 2. Zahl im Tupel!");
    }
}

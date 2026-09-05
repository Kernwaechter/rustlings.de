fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Nutze einen Tupel-Index, um auf das zweite Element von `numbers`
        // zuzugreifen, und weise es einer Variable namens `second` zu.
        // let second = ???;

        assert_eq!(second, 2, "Das ist nicht die 2. Zahl im Tupel!");
    }
}

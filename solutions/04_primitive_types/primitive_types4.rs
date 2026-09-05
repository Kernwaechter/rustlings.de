fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4  <- Indizes
        //          -------
        //             |
        //             +--- Slice

        // Beachte, dass der obere Index 4 ausgeschlossen ist.
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // Der obere Index kann mit der Syntax `..=` (mit `=`-Zeichen) eingeschlossen werden.
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}

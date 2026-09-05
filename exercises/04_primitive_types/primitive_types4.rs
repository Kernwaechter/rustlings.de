fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Hol dir aus dem Array `a` einen Slice namens `nice_slice`, sodass der Test besteht.
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}

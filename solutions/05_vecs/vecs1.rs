// Ein Vektor (`Vec`) ist eine Liste von Werten desselben Typs, die
// wachsen und schrumpfen kann.

fn elems_to_vec(a: i32, b: i32, c: i32) -> Vec<i32> {
    vec![a, b, c]
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elems_to_vec() {
        let (a, b, c) = (2, 7, 12);
        let v = elems_to_vec(a, b, c);
        assert_eq!(v[0], a);
        assert_eq!(v[1], b);
        assert_eq!(v[2], c);
    }
}

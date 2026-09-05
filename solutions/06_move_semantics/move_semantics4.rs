fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        // `y` wird hier benutzt.
        y.push(42);
        // Die veränderliche Referenz `y` wird nicht mehr benutzt,
        // deshalb kann eine neue Referenz erzeugt werden.
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

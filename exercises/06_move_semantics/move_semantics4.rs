fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    // TODO: Behebe die Compiler-Fehler nur, indem du die Zeilen im Test neu anordnest.
    // Füge keine Zeile hinzu, ändere oder entferne keine.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        let z = &mut x;
        y.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

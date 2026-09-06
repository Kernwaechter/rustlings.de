// In Rust hat jeder Wert genau einen Besitzer (Ownership) — die Variable,
// die ihn gerade hält. Gibst du einen Wert an eine andere Variable oder
// Funktion weiter, wandert der Besitz mit. Die ursprüngliche Variable
// kannst du danach nicht mehr benutzen. Diese Übungsreihe erkundet, was
// das für deinen Code bedeutet.

// TODO: Korrigiere den Compiler-Fehler in dieser Funktion.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}

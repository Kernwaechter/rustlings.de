fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    //  ^^^ hinzugefügt

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
        // Auf `vec0` kann nicht mehr zugegriffen werden, weil es nach `fill_vec` verschoben wurde.
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}

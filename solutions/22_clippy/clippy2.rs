fn main() {
    let mut res = 42;
    let option = Some(12);
    // Benutze `if-let` statt Iteration.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}

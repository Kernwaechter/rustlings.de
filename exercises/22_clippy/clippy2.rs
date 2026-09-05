fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Behebe den Clippy-Lint.
    for x in option {
        res += x;
    }

    println!("{res}");
}

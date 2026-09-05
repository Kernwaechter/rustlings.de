fn call_me(num: u8) {
    for i in 0..num {
        println!("Klingel! Anruf Nummer {}", i + 1);
    }
}

fn main() {
    // `call_me` erwartet ein Argument.
    call_me(5);
}

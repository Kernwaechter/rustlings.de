// TODO: Füge den fehlenden Typ des Arguments `num` nach dem Doppelpunkt `:` hinzu.
fn call_me(num:) {
    for i in 0..num {
        println!("Klingel! Anruf Nummer {}", i + 1);
    }
}

fn main() {
    call_me(3);
}

// Der Typ von Funktionsargumenten muss immer angegeben werden.
// Typ-Annotation `u64` hinzugefügt.
fn call_me(num: u64) {
    for i in 0..num {
        println!("Klingel! Anruf Nummer {}", i + 1);
    }
}

fn main() {
    call_me(3);
}

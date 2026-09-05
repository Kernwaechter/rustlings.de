fn main() {
    // Ein Array mit 100 Elementen mit dem Wert 42.
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Wow, das ist ein großes Array!");
    } else {
        println!("Meh, solche Arrays esse ich zum Frühstück.");
        panic!("Array nicht groß genug, mehr Elemente nötig");
    }
}

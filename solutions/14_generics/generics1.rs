// `Vec<T>` ist generisch über den Typ `T`. In den meisten Fällen kann der
// Compiler `T` selbst herleiten, zum Beispiel nachdem ein Wert mit einem
// konkreten Typ in den Vektor geschoben wurde. Aber in dieser Übung braucht
// der Compiler etwas Hilfe durch eine Typ-Annotation.

fn main() {
    // `u8` und `i8` lassen sich beide nach `i16` umwandeln.
    let mut numbers: Vec<i16> = Vec::new();
    //             ^^^^^^^^^^ hinzugefügt

    // Ändere die folgenden Zeilen nicht.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}

fn square(num: i32) -> i32 {
    // Semikolon `;` am Ende der Zeile unten entfernt, um das Ergebnis implizit zurückzugeben.
    num * num
}

fn main() {
    let answer = square(3);
    println!("Das Quadrat von 3 ist {answer}");
}

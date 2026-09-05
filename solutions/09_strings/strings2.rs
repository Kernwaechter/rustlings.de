fn is_a_color_word(attempt: &str) -> bool {
    attempt == "grün" || attempt == "blau" || attempt == "rot"
}

fn main() {
    let word = String::from("grün");

    if is_a_color_word(&word) {
        //             ^ hinzugefügt, um ein `&String` zu erhalten, das vom
        //               Compiler automatisch zu `&str` "gecoercet" wird.
        println!("Das ist ein Farbwort, das ich kenne!");
    } else {
        println!("Das ist kein Farbwort, das ich kenne.");
    }
}

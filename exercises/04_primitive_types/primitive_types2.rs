// Zeichen (`char`)

fn main() {
    // Beachte die _einfachen_ Anführungszeichen — die unterscheiden sich von den
    // doppelten Anführungszeichen, die du bisher gesehen hast.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetisch!");
    } else if my_first_initial.is_numeric() {
        println!("Numerisch!");
    } else {
        println!("Weder alphabetisch noch numerisch!");
    }

    // TODO: Lege unten, analog zum Beispiel oben, eine Variable namens
    // `your_character` mit deinem Lieblingszeichen an (deklariere sie).
    // Probier einen Buchstaben, probier eine Ziffer (in einfachen Anführungszeichen), probier
    // ein Sonderzeichen, probier ein Zeichen aus einer anderen Sprache als deiner eigenen,
    // probier ein Emoji 😉
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Alphabetisch!");
    } else if your_character.is_numeric() {
        println!("Numerisch!");
    } else {
        println!("Weder alphabetisch noch numerisch!");
    }
}

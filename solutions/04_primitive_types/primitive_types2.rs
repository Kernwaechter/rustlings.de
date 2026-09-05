fn main() {
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetisch!");
    } else if my_first_initial.is_numeric() {
        println!("Numerisch!");
    } else {
        println!("Weder alphabetisch noch numerisch!");
    }

    // Beispiel mit einem Emoji.
    let your_character = '🦀';

    if your_character.is_alphabetic() {
        println!("Alphabetisch!");
    } else if your_character.is_numeric() {
        println!("Numerisch!");
    } else {
        println!("Weder alphabetisch noch numerisch!");
    }
}

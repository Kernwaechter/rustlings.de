fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("langer String ist lang");
    // Lösung 1: Du kannst `string2` aus dem inneren Block herausziehen,
    // sodass er nicht vor der print-Anweisung verworfen (dropped) wird.
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("Der längere String ist '{result}'");
    // `string2` wird am Ende der Funktion verworfen.

    // =========================================================================

    let string1 = String::from("langer String ist lang");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // Lösung 2: Du kannst die print-Anweisung in den inneren Block
        // verschieben, sodass sie ausgeführt wird, bevor `string2` verworfen
        // wird.
        println!("Der längere String ist '{result}'");
        // `string2` wird hier verworfen (Ende des inneren Scopes).
    }
}

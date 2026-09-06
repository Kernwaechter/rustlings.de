// Ein Enum (kurz für Enumeration, zu Deutsch „Aufzählung“) ist ein Typ,
// der eine feste Auswahl an möglichen Varianten festlegt. Ein Wert dieses
// Typs ist immer genau eine dieser Varianten.

#[derive(Debug)]
enum Message {
    // TODO: Definiere ein paar Nachrichtentypen, wie sie unten benutzt werden.
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}

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

// Ein Tupel bündelt mehrere Werte mit unterschiedlichen Typen in einem
// einzigen Wert, z. B. `(String, f64)`. Mit einem Muster kannst du diese
// Werte wieder einzeln herausholen — das nennt man Destrukturieren.

fn main() {
    let cat = ("Furry McFurson", 3.5);

    // Das Tupel destrukturieren.
    let (name, age) = cat;

    println!("{name} ist {age} Jahre alt");
}

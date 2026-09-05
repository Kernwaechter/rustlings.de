#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Korrigiere den Compiler-Fehler, indem du etwas zu dieser
    // match-Anweisung hinzufügst.
    match optional_point {
        Some(p) => println!("Koordinaten sind {},{}", p.x, p.y),
        _ => panic!("Kein Match!"),
    }

    println!("{optional_point:?}"); // Diese Zeile nicht ändern.
}

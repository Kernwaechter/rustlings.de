#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // Lösung 1: Über die `Option` (nicht `&Option`) matchen, ohne dabei aus
    // der `Some`-Variante herauszuziehen.
    match optional_point {
        Some(ref p) => println!("Koordinaten sind {},{}", p.x, p.y),
        //   ^^^ hinzugefügt
        _ => panic!("Kein Match!"),
    }

    // Lösung 2: Über eine Referenz (`&Option`) matchen, indem `&` vor
    // `optional_point` hinzugefügt wird.
    match &optional_point {
        //^ hinzugefügt
        Some(p) => println!("Koordinaten sind {},{}", p.x, p.y),
        _ => panic!("Kein Match!"),
    }

    println!("{optional_point:?}");
}

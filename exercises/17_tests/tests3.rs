struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Ändere diese Funktion nicht.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Hier wäre ein `Result` besser. Aber wir wollen lernen, wie man
            // Funktionen testet, die in Panik geraten können.
            panic!("Breite und Höhe des Rechtecks müssen positiv sein");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: Dieser Test soll prüfen, ob das Rechteck die Größe hat, die
        // wir seinem Konstruktor übergeben.
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // Breite prüfen
        assert_eq!(todo!(), 20); // Höhe prüfen
    }

    // TODO: Dieser Test soll prüfen, ob das Programm in Panik gerät, wenn wir
    // versuchen, ein Rechteck mit negativer Breite zu erzeugen.
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: Dieser Test soll prüfen, ob das Programm in Panik gerät, wenn wir
    // versuchen, ein Rechteck mit negativer Höhe zu erzeugen.
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}

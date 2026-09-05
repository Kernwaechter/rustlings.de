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
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Breite prüfen
        assert_eq!(rect.height, 20); // Höhe prüfen
    }

    #[test]
    #[should_panic] // Dieses Attribut hinzugefügt, um zu prüfen, dass der Test in Panik gerät.
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic] // Dieses Attribut hinzugefügt, um zu prüfen, dass der Test in Panik gerät.
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}

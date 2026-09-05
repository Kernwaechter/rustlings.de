// Diese Funktion gibt zurück, wie viel Eis noch im Kühlschrank ist.
// Vor 22:00 Uhr (24-Stunden-Format) sind noch 5 Kugeln übrig. Um 22:00 Uhr
// isst jemand alles auf, sodass kein Eis mehr übrig ist (Wert 0). Gib `None`
// zurück, wenn `hour_of_day` größer als 23 ist.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Vervollständige den Funktionsrumpf.
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Repariere diesen Test. Wie kommst du an den Wert, der in der
        // Option enthalten ist?
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // Diese Zeile nicht ändern.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}

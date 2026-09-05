// Beim Ausführen von Operationen auf Elementen innerhalb einer Sammlung
// (Collection) sind Iteratoren unverzichtbar. Dieses Modul hilft dir, dich
// mit dem Aufbau der Benutzung eines Iterators vertraut zu machen und
// damit, wie man Elemente innerhalb einer iterierbaren Sammlung durchläuft.

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = &["Banane", "Cherimoya", "Avocado", "Pfirsich", "Himbeere"];

        // TODO: Erzeuge einen Iterator über den Slice.
        let mut fav_fruits_iterator = todo!();

        assert_eq!(fav_fruits_iterator.next(), Some(&"Banane"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Ersetze `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"Avocado"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Ersetze `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"Himbeere"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Ersetze `todo!()`
    }
}

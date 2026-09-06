// Willst du mit Elementen in einer Sammlung (Collection) arbeiten, kommst
// du an Iteratoren nicht vorbei. Diese Übung zeigt dir den grundlegenden
// Aufbau eines Iterators und wie du damit Element für Element durch eine
// Sammlung gehst.

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

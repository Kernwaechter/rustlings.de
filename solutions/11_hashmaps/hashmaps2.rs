// Wir sammeln verschiedene Früchte, um einen köstlichen Obstkuchen zu backen.
// Dafür haben wir einen Korb, den wir als Hashmap darstellen. Der Schlüssel
// steht für den Namen jeder gesammelten Frucht, der Wert für die Anzahl
// dieser Frucht, die wir gesammelt haben. Drei Fruchtsorten – Apfel (4),
// Mango (2) und Litschi (5) – befinden sich bereits im Korb. Du musst
// Früchte hinzufügen, sodass mindestens eine von jeder Sorte enthalten ist
// und die Gesamtzahl über 11 liegt – wir haben viele hungrige Mäuler zu
// stopfen. Du darfst dabei keine weiteren Früchte der bereits vorhandenen
// Sorten (Apfel, Mango und Litschi) hinzufügen.

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // Falls die Frucht noch nicht existiert, füge sie mit einem Wert ein.
        basket.entry(fruit).or_insert(5);
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verändere diese Funktion nicht!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Fruchtsorte {fruit_kind:?} wurde nicht im Korb gefunden");
            };
            assert!(*amount > 0);
        }
    }
}

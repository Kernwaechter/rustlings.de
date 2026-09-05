// Ein Korb voller Früchte in Form einer Hashmap muss angelegt werden. Der
// Schlüssel steht für den Namen der Frucht, der Wert für die Anzahl dieser
// Frucht im Korb. Du musst mindestens 3 verschiedene Fruchtsorten (z. B.
// Apfel, Banane, Mango) in den Korb legen, und die Gesamtzahl aller Früchte
// muss mindestens 5 betragen.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // Deklariere die Hashmap.
    let mut basket = HashMap::new();

    // Zwei Bananen sind schon für dich vorgegeben :)
    basket.insert(String::from("Banane"), 2);

    // Lege weitere Früchte in deinen Korb.
    basket.insert(String::from("Apfel"), 3);
    basket.insert(String::from("Mango"), 1);

    basket
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}

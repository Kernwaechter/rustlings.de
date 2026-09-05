// Dieser Laden hat einen Sonderverkauf: Ist der Preis eine gerade Zahl, gibt es
// 10 Rustbucks Rabatt, ist er eine ungerade Zahl, gibt es 3 Rustbucks Rabatt.
// Die Funktionskörper selbst sind hier nicht wichtig, uns interessieren erstmal
// nur die Signaturen.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Korrigiere die Funktionssignatur.
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Dein Sonderpreis beträgt {}", sale_price(original_price));
}

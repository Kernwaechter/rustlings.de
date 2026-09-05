// Es werden 3 mögliche Lösungen vorgestellt.

// Mit einer `for`-Schleife und einer veränderlichen Variable.
fn factorial_for(num: u64) -> u64 {
    let mut result = 1;

    for x in 2..=num {
        result *= x;
    }

    result
}

// Entspricht `factorial_for`, aber kürzer und ohne `for`-Schleife und
// veränderliche Variablen.
fn factorial_fold(num: u64) -> u64 {
    // Fall num==0: Der Iterator 2..=0 ist leer
    //              -> Der Startwert von `fold` wird zurückgegeben, also 1.
    // Fall num==1: Der Iterator 2..=1 ist ebenfalls leer
    //              -> Der Startwert 1 wird zurückgegeben.
    // Fall num==2: Der Iterator 2..=2 enthält ein Element
    //              -> Der Startwert 1 wird mit 2 multipliziert, und das
    //                 Ergebnis wird zurückgegeben.
    // Fall num==3: Der Iterator 2..=3 enthält 2 Elemente
    //              -> 1 * 2 wird berechnet, dann wird das Ergebnis 2 mit
    //                 dem zweiten Element 3 multipliziert, sodass das
    //                 Ergebnis 6 zurückgegeben wird.
    // Und so weiter…
    #[allow(clippy::unnecessary_fold)]
    (2..=num).fold(1, |acc, x| acc * x)
}

// Entspricht `factorial_fold`, aber mit einer eingebauten Methode, die von
// Clippy vorgeschlagen wird.
fn factorial_product(num: u64) -> u64 {
    (2..=num).product()
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial_for(0), 1);
        assert_eq!(factorial_fold(0), 1);
        assert_eq!(factorial_product(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial_for(1), 1);
        assert_eq!(factorial_fold(1), 1);
        assert_eq!(factorial_product(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial_for(2), 2);
        assert_eq!(factorial_fold(2), 2);
        assert_eq!(factorial_product(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial_for(4), 24);
        assert_eq!(factorial_fold(4), 24);
        assert_eq!(factorial_product(4), 24);
    }
}

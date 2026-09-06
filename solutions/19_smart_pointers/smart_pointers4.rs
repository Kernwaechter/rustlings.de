// Diese Übung erkundet den intelligenten Zeiger (Smart Pointer) `Cow`
// (Clone-On-Write). Er umschließt ausgeliehene (geborgte) Daten und gewährt
// dir unveränderlichen Zugriff darauf. Erst wenn du die Daten verändern
// willst oder besitzen musst (Ownership), klont `Cow` sie — und das auch
// erst in genau diesem Moment (träge, lazily).
//
// `Cow` ist außerdem so gebaut, dass er über den Trait `Borrow` mit ganz
// verschiedenen Arten von ausgeliehenen Daten funktioniert.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Klont in einen Vektor, falls noch nicht im eigenen Besitz.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Ein Klon entsteht, weil `input` verändert werden muss.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // Es entsteht kein Klon, weil `input` nicht verändert werden muss.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Borrowed(_)));
        //                      ^^^^^^^^^^^^^^^^
    }

    #[test]
    fn owned_no_mutation() {
        // Wir können `vec` auch ohne `&` übergeben, sodass `Cow` es direkt
        // besitzt. In diesem Fall findet keine Veränderung statt (alle
        // Zahlen sind schon absolut) und somit auch kein Klon. Aber das
        // Ergebnis ist trotzdem im eigenen Besitz, weil es nie ausgeliehen
        // oder verändert wurde.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
        //                      ^^^^^^^^^^^^^
    }

    #[test]
    fn owned_mutation() {
        // Das gilt natürlich auch, wenn tatsächlich eine Veränderung
        // stattfindet (nicht alle Zahlen sind absolut). In diesem Fall gibt
        // der Aufruf von `to_mut()` in der Funktion `abs_all` eine Referenz
        // auf dieselben Daten wie zuvor zurück.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
        //                      ^^^^^^^^^^^^^
    }
}

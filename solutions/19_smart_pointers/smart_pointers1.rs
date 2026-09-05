// Zur Kompilierzeit muss Rust wissen, wie viel Platz ein Typ einnimmt. Das
// wird bei rekursiven Typen zum Problem, bei denen ein Wert als Teil von
// sich selbst einen weiteren Wert desselben Typs enthalten kann. Um dieses
// Problem zu umgehen, können wir einen `Box` benutzen – einen intelligenten
// Zeiger (Smart Pointer), der Daten auf dem Heap speichert und uns damit
// auch erlaubt, einen rekursiven Typ zu umschließen.
//
// Der rekursive Typ, den wir in dieser Übung implementieren, ist die
// "Cons-Liste", eine Datenstruktur, die häufig in funktionalen
// Programmiersprachen vorkommt. Jedes Element einer Cons-Liste enthält zwei
// Teile: den Wert des aktuellen Elements und das nächste Element. Das
// letzte Element ist ein Wert namens `Nil`.

#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn create_empty_list() -> List {
    List::Nil
}

fn create_non_empty_list() -> List {
    List::Cons(42, Box::new(List::Nil))
}

fn main() {
    println!("Das ist eine leere Cons-Liste: {:?}", create_empty_list());
    println!(
        "Das ist eine nicht-leere Cons-Liste: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}

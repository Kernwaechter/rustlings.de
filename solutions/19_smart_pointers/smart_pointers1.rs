// Rust muss zur Kompilierzeit wissen, wie viel Speicherplatz ein Typ
// braucht. Bei rekursiven Typen wird das zum Problem: Ein rekursiver Typ
// enthält als Teil von sich selbst noch einen weiteren Wert vom gleichen
// Typ. Dadurch weiß Rust nicht, wie groß der Typ am Ende wirklich ist.
//
// Die Lösung ist eine `Box`. Das ist ein intelligenter Zeiger (Smart
// Pointer). Er speichert die Daten auf dem Heap statt auf dem Stack.
// Dadurch kannst du einen rekursiven Typ trotzdem benutzen.
//
// In dieser Übung baust du eine "Cons-Liste". Das ist eine Datenstruktur
// aus der funktionalen Programmierung. Jedes Element einer Cons-Liste
// besteht aus zwei Teilen: dem Wert des aktuellen Elements und dem
// nächsten Element. Das allerletzte Element trägt den Wert `Nil`.

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

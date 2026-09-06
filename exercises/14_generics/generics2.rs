// Ein Wrapper ist wie eine Hülle: Er umschließt einen anderen Wert, ohne
// ihn selbst zu verändern. Dieser mächtige Wrapper hier speichert einen
// positiven Ganzzahlwert.
// TODO: Schreibe ihn mit einem generischen Typ um, sodass er JEDEN Typ
// umschließen kann.
struct Wrapper {
    value: u32,
}

// TODO: Passe die Implementierung der Struktur an, sodass sie generisch
// über den umschlossenen Wert ist.
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

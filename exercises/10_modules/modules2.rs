// Du kannst Modulpfade mit den Schlüsselwörtern `use` und `as` in einen Scope holen
// und ihnen dabei neue Namen geben.

mod delicious_snacks {
    // TODO: Füge die folgenden zwei `use`-Anweisungen hinzu, nachdem du sie korrigiert hast.
    // use self::fruits::BIRNE as ???;
    // use self::veggies::GURKE as ???;

    mod fruits {
        pub const BIRNE: &str = "Birne";
        pub const APFEL: &str = "Apfel";
    }

    mod veggies {
        pub const GURKE: &str = "Gurke";
        pub const KAROTTE: &str = "Karotte";
    }
}

fn main() {
    println!(
        "Lieblingssnacks: {} und {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}

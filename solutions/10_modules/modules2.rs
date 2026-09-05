#![allow(dead_code)] // APFEL und KAROTTE sind absichtlich ungenutzt: Nur BIRNE/GURKE werden re-exportiert.

mod delicious_snacks {
    // `pub` hinzugefügt und den erwarteten Alias-Namen nach `as` benutzt.
    pub use self::fruits::BIRNE as fruit;
    pub use self::veggies::GURKE as veggie;

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

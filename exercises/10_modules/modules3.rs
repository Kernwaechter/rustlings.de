// Du kannst das Schlüsselwort `use` benutzen, um Modulpfade aus Modulen von überall her
// und besonders aus der Standardbibliothek in deinen Scope zu holen.

// TODO: Hole `SystemTime` und `UNIX_EPOCH` aus dem Modul `std::time` in deinen Scope.
// Bonus-Stilpunkte, wenn du das in einer einzigen Zeile schaffst!
// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC war vor {} Sekunden!", n.as_secs()),
        Err(_) => panic!("SystemTime liegt vor der UNIX EPOCH!"),
    }
}

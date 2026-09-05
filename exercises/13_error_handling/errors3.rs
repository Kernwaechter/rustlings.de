// Dieses Programm versucht, eine fertige Version der Funktion `total_cost`
// aus der vorherigen Übung zu benutzen. Es funktioniert aber nicht! Warum
// nicht? Was müssen wir tun, um es zu reparieren?

use std::num::ParseIntError;

// Ändere diese Funktion nicht.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Behebe den Compiler-Fehler, indem du die Signatur und den Rumpf der
// `main`-Funktion änderst.
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Ändere diese Zeile nicht.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("Das kannst du dir nicht leisten!");
    } else {
        tokens -= cost;
        println!("Du hast jetzt {tokens} Münzen.");
    }
}

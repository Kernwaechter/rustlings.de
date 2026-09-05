// Das Tool Clippy ist eine Sammlung von Lints, die deinen Code analysieren,
// damit du häufige Fehler entdecken und deinen Rust-Code verbessern
// kannst.
//
// Bei diesen Übungen lässt sich der Code nicht kompilieren, solange es
// Clippy-Warnungen gibt. Schau dir Clippys Vorschläge in der Ausgabe an,
// um die Übung zu lösen.

fn main() {
    // TODO: Korrigiere den Clippy-Lint in dieser Zeile.
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("Die Fläche eines Kreises mit Radius {radius:.2} beträgt {area:.5}");
}

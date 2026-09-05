// Hier sind noch ein paar einfache Clippy-Fixes, damit du seinen Nutzen
// siehst.
// TODO: Korrigiere alle Clippy-Lints.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Nimm an, dass du den Wert von `my_option` nicht kennst.
    // Im Fall von `Some` wollen wir seinen Wert ausgeben.
    if my_option.is_none() {
        println!("{}", my_option.unwrap());
    }

    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("Mein Array! Hier ist es: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.resize(0, 5);
    println!("Dieser Vec ist leer, siehst du? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Lass uns die beiden tauschen!
    value_a = value_b;
    value_b = value_a;
    println!("Wert a: {value_a}; Wert b: {value_b}");
}

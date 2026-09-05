use std::mem;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Ein `unwrap` einer `Option`, nachdem geprüft wurde, ob sie `None`
    // ist, gerät in Panik. Benutze stattdessen `if-let`.
    if let Some(value) = my_option {
        println!("{value}");
    }

    // Ein Komma hat gefehlt.
    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("Mein Array! Hier ist es: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    // `resize` verändert einen Vektor, statt einen neuen zurückzugeben.
    // `resize(0, …)` leert einen Vektor, daher ist es besser, `clear` zu
    // benutzen.
    my_vec.clear();
    println!("Dieser Vec ist leer, siehst du? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Benutze `mem::swap`, um zwei Werte korrekt zu tauschen.
    mem::swap(&mut value_a, &mut value_b);
    println!("Wert a: {value_a}; Wert b: {value_b}");
}

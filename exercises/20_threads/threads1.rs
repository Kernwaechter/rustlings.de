// Dieses Programm erzeugt mehrere Threads, von denen jeder mindestens
// 250ms läuft, und jeder Thread gibt zurück, wie lange er zum Abschließen
// gebraucht hat. Das Programm soll warten, bis alle erzeugten Threads
// fertig sind, und ihre Rückgabewerte in einem Vektor sammeln.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} fertig");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Sammle die Ergebnisse aller Threads im Vektor `results`.
        // Benutze die Struktur `JoinHandle`, die von `thread::spawn`
        // zurückgegeben wird.
    }

    if results.len() != 10 {
        panic!("Oh nein! Irgendein Thread ist noch nicht fertig!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} brauchte {result}ms");
    }
}

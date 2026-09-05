// In dieser Übung bekommen wir einen `Vec` von `u32` namens `numbers` mit
// Werten von 0 bis 99. Wir möchten diese Zahlenmenge gleichzeitig in 8
// verschiedenen Threads benutzen. Jeder Thread bildet die Summe jedes
// achten Werts mit einem Versatz (Offset).
//
// Der erste Thread (Offset 0) summiert 0, 8, 16, …
// Der zweite Thread (Offset 1) summiert 1, 9, 17, …
// Der dritte Thread (Offset 2) summiert 2, 10, 18, …
// …
// Der achte Thread (Offset 7) summiert 7, 15, 23, …
//
// Jeder Thread soll einen referenzzählenden Zeiger auf den Vektor der
// Zahlen besitzen. Aber `Rc` ist nicht thread-sicher. Deshalb müssen wir
// `Arc` benutzen.
//
// Lass dich nicht davon ablenken, wie Threads erzeugt (spawn) und
// zusammengeführt (join) werden. Das üben wir später in den Übungen zu
// Threads.

// Ändere die folgenden Zeilen nicht.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Definiere `shared_numbers` mithilfe von `Arc`.
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Definiere `child_numbers` ausgehend von `shared_numbers`.
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Summe von Offset {offset} ist {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}

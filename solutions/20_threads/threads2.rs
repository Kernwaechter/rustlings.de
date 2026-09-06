// Aufbauend auf der letzten Übung wollen wir, dass alle Threads ihre
// Arbeit abschließen. Diesmal müssen die erzeugten Threads aber dafür
// zuständig sein, einen gemeinsam genutzten Wert zu aktualisieren:
// `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // `Arc` reicht nicht aus, wenn du einen **veränderlichen** gemeinsam
    // genutzten Zustand willst.
    // Wir müssen den Wert mit einem `Mutex` umschließen: Das ist ein
    // Schließmechanismus (Lock), der jeweils nur einem Thread gleichzeitig
    // Zugriff auf die Daten erlaubt.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
    //                    ^^^^^^^^^^^                          ^

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Sperren (lock), bevor du einen gemeinsam genutzten Wert
            // aktualisierst.
            status_shared.lock().unwrap().jobs_done += 1;
            //           ^^^^^^^^^^^^^^^^
        });
        handles.push(handle);
    }

    // Warten, bis alle Jobs abgeschlossen sind.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Erledigte Jobs: {}", status.lock().unwrap().jobs_done);
    //                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}

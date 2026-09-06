// Alice ist Grundschullehrerin und unterrichtet drei Klassen. Für jede
// Klasse muss sie den Notendurchschnitt der Klassenarbeit berechnen. Statt
// das nacheinander zu tun, bittet sie ihre Freunde Bob und Catherine um
// Hilfe. Arbeiten sie zusammen, sind sie viel schneller fertig.
//
// Lass uns das mit asynchroner Programmierung nachbauen (simulieren). Wir
// stellen jede Person als asynchrone Aufgabe dar (Task). Diese Aufgaben
// können nebenläufig laufen — also gleichzeitig, statt nacheinander.

// Asynchrone Tasks müssen von einer „Runtime“ ausgeführt werden, die nicht
// von Rusts Standardbibliothek bereitgestellt wird. Hier benutzen wir die
// verbreitete Runtime `tokio`. Das Makro `tokio::main` umschließt die
// gesamte main-Funktion mit einer Runtime.
#[tokio::main]
async fn main() {
    let mean_score_a = tokio::spawn(calculate_mean_score("scores_class_a.txt"));
    let mean_score_b = tokio::spawn(calculate_mean_score("scores_class_b.txt"));
    let mean_score_c = tokio::spawn(calculate_mean_score("scores_class_c.txt"));

    // TODO: Warte (await) auf die gestarteten Tasks, um ihre Ergebnisse zu
    // prüfen.
    assert_eq!(mean_score_a, 84); // alice
    assert_eq!(mean_score_b, 89); // bob
    assert_eq!(mean_score_c, 76); // catherine
}

// TODO: Korrigiere die Compiler-Fehler, indem du die gestartete Funktion async
// machst.
fn calculate_mean_score(scores_file: &str) -> usize {
    // Lies die Datei asynchron ein
    let file = tokio::fs::read_to_string(scores_file).await.unwrap();

    // Setze die Summe und die Anzahl der Werte auf einen Startwert (Initialisierung)
    let mut sum = 0;
    let mut n = 0;
    for line in file.lines() {
        // Parse jede Zeile als Testergebnis
        let score = line.parse::<usize>().unwrap();
        sum += score;
        n += 1;
    }

    sum / n
}

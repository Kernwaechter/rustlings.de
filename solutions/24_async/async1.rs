// Alice ist eine Grundschullehrerin, die den Notendurchschnitt der
// Klassenarbeit für drei Klassen berechnen muss, die sie unterrichtet.
// Statt sie nacheinander zu berechnen, bittet sie ihre Freunde Bob und
// Catherine um Hilfe. Wenn sie zusammenarbeiten, werden sie viel schneller
// fertig.
//
// Lass uns das mit asynchroner Programmierung simulieren. Jede Person wird
// als asynchrone Aufgabe (Task) dargestellt, die nebenläufig ausgeführt
// werden kann.

// Asynchrone Tasks müssen von einer „Runtime“ ausgeführt werden, die nicht
// von Rusts Standardbibliothek bereitgestellt wird. Hier benutzen wir die
// verbreitete Runtime `tokio`. Das Makro `tokio::main` umschließt die
// gesamte main-Funktion mit einer Runtime.
#[tokio::main]
async fn main() {
    let mean_score_a = tokio::spawn(calculate_mean_score("scores_class_a.txt"));
    let mean_score_b = tokio::spawn(calculate_mean_score("scores_class_b.txt"));
    let mean_score_c = tokio::spawn(calculate_mean_score("scores_class_c.txt"));

    assert_eq!(mean_score_a.await.unwrap(), 84); // alice
    assert_eq!(mean_score_b.await.unwrap(), 89); // bob
    assert_eq!(mean_score_c.await.unwrap(), 76); // catherine
}

async fn calculate_mean_score(scores_file: &str) -> usize {
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

# Übersetzungsrichtlinien für rustlings.de

Diese Datei hält fest, nach welchen Regeln dieses Projekt übersetzt wurde,
damit neue Übungen (rustlings ergänzt gelegentlich welche) und Korrekturen
konsistent bleiben — auch wenn nicht mehr alles davon im Kopf ist.

## Grundprinzip

Übersetzt werden **Aufgabenstellungen, Kommentare und Hinweise
(`info.toml`)**. Nicht übersetzt bzw. unverändert übernommen:

- Rust-Bezeichner: Funktions-, Variablen- und Typnamen bleiben englisch
  (`fruit_basket`, `calculate_mean_score`, `Message`, …). Das gilt auch für
  Enum-Varianten, selbst wenn sie wie Vokabeln aussehen (`Fruit::Apple`
  bleibt `Apple`, nicht `Apfel`).
- Rust-Kernbegriffe ohne gute deutsche Entsprechung: `Trait`, `Enum`,
  `Struct`, `Generics`, `Iterator`, `Closure`, `Slice`, `Lifetime` usw.
  bleiben als Fachbegriff stehen (siehe aber Abschnitt „Fachbegriffe"
  unten — sie müssen bei ihrem ersten Auftauchen erklärt werden).
- Compiler-Fehlermeldungen von `rustc` selbst — die bleiben englisch, dafür
  gibt es (noch) keinen deutschen Fluent-Katalog.
- Kanonische Phrasen wie `"Hello world!"`.
- Eigennamen (`Beyoncé`, `George Orwell`, `Tom Wriggle`, …).
- Wortspiele, die eine bestimmte mechanische Eigenschaft demonstrieren
  (z. B. `"Interpolation {}", "Station"` in `strings4`, wo es um
  String-Interpolation *als Konzept* geht) — hier notfalls ein
  gleichwertiges deutsches Wortspiel konstruieren, das dieselbe Eigenschaft
  zeigt, statt wörtlich zu übersetzen.

Beispiel-/Testdaten (Fruchtnamen, Ländernamen, Farben, …) werden dagegen
eingedeutscht, wenn es dem Verständnis hilft — dabei aber **konsistent an
allen Stellen** ändern (Code, Testdaten-String, jede einzelne Assertion),
sonst schlägt der Test fehl.

## Fachbegriffe: deutsche Bedeutung zuerst, Fachwort in Klammern

Wenn ein Fremdwort oder Fachbegriff nicht durch den Kontext selbsterklärend
ist, wird zuerst eine klare deutsche Formulierung verwendet und der
Fachbegriff dahinter in Klammern ergänzt — nicht umgekehrt.

- Falsch: „…, indem du den Typ des Vektors `Vec<T>` **annotierst**."
- Richtig: „…, indem du den Typ des Vektors `Vec<T>` **explizit angibst
  (Annotation)**."

Weitere Beispiele aus dem Projekt: „Lege die Hashmap an (Deklaration)",
„ein neues Objekt (eine Instanz)", „die Genauigkeit der Fließkommazahlen
(Präzision)".

**Wichtige Rust-/CS-Fachbegriffe bekommen zusätzlich bei ihrem ersten
Auftauchen im Kurs (in Sektor-Reihenfolge 00–24 + quizzes) eine kurze
Erklärung**, egal ob es sich um ein klassisches Fremdwort handelt oder
nicht (Beispiele: `Wrapper`, `Enum`, `Trait`, `Makro`, `Lifetime`, `Tupel`,
`Vektor`, `Mutex`, `Closure`, `Ownership`). Das gilt auch dann, wenn das
englische Original an der Stelle **gar keine** Erklärung hat — in dem Fall
ist es eine bewusste Ergänzung über reine Übersetzung hinaus, keine
Übersetzungskorrektur. Vor so einer Ergänzung immer per Volltextsuche über
das ganze Projekt prüfen, ob der Begriff wirklich zum ersten Mal auftaucht
(nicht raten).

## Sätze für Programmieranfänger

rustlings richtet sich an Programmieranfänger. Deshalb:

1. **Kurze, einfache Sätze.** Ein Gedanke pro Satz. Lange Sätze mit
   mehreren aneinandergehängten Nebensätzen aufteilen.
2. **Keine Partizip-Konstruktionen** wie „die von `FromStr` ausgedrückte
   Umwandlung" oder „bei jedem verwendeten Wert". Das ist grammatisch
   korrektes, aber unnötig formelles Schriftdeutsch. Stattdessen einen
   Relativsatz oder einen eigenen Satz daraus machen: „die Umwandlung, die
   `FromStr` ausdrückt" oder besser noch als zwei Sätze.
3. **Aktiv statt Passiv**, wo es genauso gut geht. „Du musst X einsammeln"
   statt „X muss eingesammelt werden".
4. Siehe oben: Fachbegriffe in Klammern erklären.

## Workflow für neue oder geänderte Übungen

1. **Nie aus einer lokal gespielten rustlings-Kopie übersetzen.** Eine
   Kopie, in der schon Übungen gelöst wurden, zeigt bereits die gelöste
   Fassung statt der ursprünglichen Aufgabe. Original-Inhalt immer frisch
   holen:
   ```
   curl -s "https://raw.githubusercontent.com/rust-lang/rustlings/main/exercises/<SEKTOR>/<NAME>.rs"
   curl -s "https://raw.githubusercontent.com/rust-lang/rustlings/main/solutions/<SEKTOR>/<NAME>.rs"
   ```
2. **Hints/Hinweise** stehen im aktuellen Upstream-Repo nicht mehr in
   `info.toml` im Root, sondern in `rustlings-macros/info.toml`:
   ```
   https://raw.githubusercontent.com/rust-lang/rustlings/main/rustlings-macros/info.toml
   ```
   Falls `rustlings hint <name>` lokal installiert und die Übung dort
   bekannt ist, liefert das denselben Text und ist bequemer.
3. Sektor- und Dateinamen nicht raten — manche Sektoren benennen ihre
   Dateien anders als den Ordner (`13_error_handling/errors1.rs`, nicht
   `error_handling1.rs`). Vorher die echte Verzeichnisliste prüfen:
   ```
   curl -s "https://api.github.com/repos/rust-lang/rustlings/contents/exercises/<SEKTOR>"
   ```
4. Übersetzen nach den Regeln oben.
5. `info.toml`-Eintrag ergänzen/anpassen:
   - `test = false` setzen, wenn die Datei **kein** `#[cfg(test)] mod
     tests`-Modul hat.
   - `strict_clippy = true` setzen, wenn Clippy-Warnungen zum
     Kompilierfehler werden sollen (aktuell nur die drei `clippy*`-Übungen).
   - `skip_check_unsolved` nur für die seltenen Fälle, in denen die
     unveränderte Übung bereits fehlerfrei läuft.
6. Validieren:
   ```
   rustlings dev update && rustlings dev check
   ```
7. Commit + Push. Ein Commit pro Sektor/Thema hat sich bewährt, damit der
   Verlauf nachvollziehbar bleibt.

## Bekannte technische Stolperfallen

- **`dead_code`-Lint unter `dev check`:** Felder, die nur über
  `#[derive(Debug)]`-Ausgabe (`println!("{x:?}")`) konsumiert werden, nie
  aber einzeln gelesen/destrukturiert, gelten unter `rustlings dev check`
  als „nie gelesen" — auch mit Debug-Ableitung. Das betrifft nur die
  striktere Community-Exercise-Prüfung, nicht die normalen, eingebauten
  Übungen. Fix: `#![allow(dead_code)]` mit kurzem erklärendem Kommentar
  direkt über der betroffenen Struktur/Enum ergänzen — in Übung UND Lösung,
  falls beide betroffen sind.
- **`rustfmt` mit der richtigen Edition aufrufen:** Dieses Projekt nutzt
  Edition 2024 (`Cargo.toml`). Reines `rustfmt datei.rs` ohne
  `--edition 2024` formatiert teils anders, als `rustlings dev check` es
  erwartet (aufgefallen, als deutsche Ländernamen eine Zeile länger als das
  Original machten). Immer mit `rustfmt --edition 2024 <Datei>` arbeiten.
- **Community-Exercise-Ordner sind strikt:** `rustlings dev check`
  akzeptiert in `exercises/<Sektor>/` nur `.rs`-Dateien und `README.md` —
  keine zusätzlichen Datendateien. Falls eine Übung wie `24_async/async1`
  zur Laufzeit externe Dateien liest (`tokio::fs::read_to_string(...)` mit
  einem nackten Dateinamen), werden diese relativ zum Arbeitsverzeichnis
  aufgelöst, aus dem `rustlings` selbst gestartet wird — also dem
  Projekt-Root, nicht dem Übungsordner. Deshalb liegen
  `scores_class_a/b/c.txt` im Projekt-Root. Upstream scheint das inzwischen
  über ein neueres `input_files`-Feld in `info.toml` zu lösen; ob die
  installierte rustlings-Version das schon unterstützt, vorher prüfen
  (`grep input_files` im installierten `rustlings`-Quelltext oder
  ausprobieren).

## Lizenz

MIT, mit dem **Original-Copyright-Halter** (Carol (Nichols || Goulding) und
die Rustlings-Mitwirkenden), nicht mit dem Namen des Übersetzers — siehe
[`LICENSE`](LICENSE). Das entspricht dem Vorgehen bei anderen
Community-Übersetzungen wie `rustlings-jp`.

## Kontext

Diese Übersetzung wurde als offizielle Rustlings-Community-Exercise
vorgeschlagen: [rust-lang/rustlings#2445](https://github.com/rust-lang/rustlings/issues/2445).

// Dieses Quiz testet:
// - Generics
// - Traits
//
// Eine imaginäre Zauberschule hat ein neues System zur Zeugniserstellung.
// Es ist in Rust geschrieben! Aktuell kann das System nur Zeugnisse mit
// einer Note in Zahlen erstellen (z. B. 1,0 -> 5,5). Die Schule vergibt
// aber auch alphabetische Noten (A+ -> F-) und muss beide Arten von
// Zeugnissen ausdrucken können!
//
// Ändere die Struktur `ReportCard` und den impl-Block so, dass sie
// zusätzlich zu numerischen auch alphabetische Zeugnisse unterstützen.

// TODO: Passe die Struktur wie oben beschrieben an.
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: Passe den impl-Block wie oben beschrieben an.
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - hat eine Note von {} erreicht",
            self.student_name, self.student_age, self.grade,
        )
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - hat eine Note von 2.1 erreicht",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - hat eine Note von A+ erreicht",
        );
    }
}

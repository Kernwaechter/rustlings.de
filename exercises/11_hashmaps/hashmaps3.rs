// Eine Liste von Ergebnissen (eines pro Zeile) eines Fußballspiels ist
// gegeben. Jede Zeile hat die Form
// "<team_1_name>,<team_2_name>,<team_1_tore>,<team_2_tore>"
// Beispiel: "England,Frankreich,4,2" (England hat 4 Tore geschossen, Frankreich 2).
//
// Du musst eine Ergebnistabelle erstellen, die den Namen des Teams, die
// Gesamtzahl der geschossenen Tore und die Gesamtzahl der kassierten Tore
// enthält.

use std::collections::HashMap;

// Eine Struktur, um die Tordetails eines Teams zu speichern.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_score_table(results: &str) -> HashMap<&str, TeamScores> {
    // Der Name des Teams ist der Schlüssel, die zugehörige Struktur der Wert.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // HINWEIS: Wir benutzen `unwrap`, weil wir uns noch nicht mit
        // Fehlerbehandlung beschäftigt haben.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Befülle die Ergebnistabelle mit den extrahierten Details.
        // Denk daran: Die von Team 1 geschossenen Tore sind die von Team 2
        // kassierten Tore. Genauso sind die von Team 2 geschossenen Tore die
        // von Team 1 kassierten Tore.
    }

    scores
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,Frankreich,4,2
Frankreich,Italien,3,1
Polen,Spanien,2,0
Deutschland,England,2,1
England,Spanien,1,0";

    #[test]
    fn build_scores() {
        let scores = build_score_table(RESULTS);

        assert!(
            [
                "England",
                "Frankreich",
                "Deutschland",
                "Italien",
                "Polen",
                "Spanien"
            ]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name))
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_score_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_score_table(RESULTS);
        let team = scores.get("Spanien").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}

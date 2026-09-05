// In dieser Übung lernst du einige der einzigartigen Vorteile kennen, die
// Iteratoren bieten können.

// "hallo" -> "Hallo"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

// Wende die Funktion `capitalize_first` auf einen Slice von String-Slices
// an. Gib einen Vektor von Strings zurück.
// ["hallo", "welt"] -> ["Hallo", "Welt"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// Wende die Funktion `capitalize_first` erneut auf einen Slice von
// String-Slices an. Gib einen einzigen String zurück.
// ["hallo", " ", "welt"] -> "Hallo Welt"
fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect()
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hallo"), "Hallo");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hallo", "welt"];
        assert_eq!(capitalize_words_vector(&words), ["Hallo", "Welt"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hallo", " ", "welt"];
        assert_eq!(capitalize_words_string(&words), "Hallo Welt");
    }
}

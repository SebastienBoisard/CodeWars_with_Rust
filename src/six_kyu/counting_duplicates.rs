#[allow(dead_code)]

/// # Counting Duplicates
///
/// ## Instructions
///
///
///
/// ## Examples
///
///
///
/// ## What I learned
///
/// -
///

use itertools::Itertools;

fn count_duplicates(text: &str) -> u32 {
    text.chars().duplicates_by(|s| s.to_string().to_lowercase()).collect::<Vec<char>>().len()  as u32
}

use std::collections::HashMap;

fn _count_duplicates_v1(text: &str) -> u32 {
    let mut char_count: HashMap<char, u32> = HashMap::new();
    for c in text.to_lowercase().chars() {
        let e = char_count.entry(c).or_default();
        *e += 1;
    }
    char_count.values().filter(|&&v| v > 1).count() as u32
}

fn _count_duplicates_v2(text: &str) -> u32 {
    text.to_lowercase().chars().counts().values().filter(|&&i| i > 1).count() as u32
}

fn _count_duplicates_v3(text: &str) -> u32 {
    text.chars().duplicates_by(char::to_ascii_lowercase).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(count_duplicates(""), 0);
    }

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdeaa() {
        assert_eq!(count_duplicates("abcdeaa"), 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_abcdeaB() {
        assert_eq!(count_duplicates("abcdeaB"), 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Indivisibilities() {
        assert_eq!(count_duplicates("Indivisibilities"), 2);
    }

    #[test]
    fn test_lowercase() {
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(count_duplicates(lowercase), 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_lowercaseaaAb() {
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(count_duplicates(&format!("{}aaAb", lowercase)), 2);
    }

    #[test]
    fn test_twice_lowercase() {
        let lowercase = "abcdefghijklmnopqrstuvwxyz".repeat(2);
        assert_eq!(count_duplicates(&lowercase), 26);
    }

    #[test]
    fn test_lowercase_uppercase() {
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        assert_eq!(count_duplicates(&format!("{}{}", lowercase, uppercase)), 26);
    }

    #[test]
    fn test_codewarsisawesome() {
        let text = "CODEwarsIsAWesOmE";
        assert_eq!(count_duplicates(text), 5);
    }

    #[test]
    fn test_numbers() {
        let numbers = "0123456789";
        assert_eq!(count_duplicates(numbers), 0);
    }

    #[test]
    fn test_twice_numbers() {
        let numbers = "0123456789".repeat(2);
        assert_eq!(count_duplicates(&numbers), 10);
    }

    #[test]
    fn test_lowercase_vowels_numbers_uppercase_vowels() {
        let lowercase_vowels = "aeiou";
        let numbers = "0123456789";
        let uppercase_vowels = "AEIOU";
        assert_eq!(count_duplicates(&format!("{}{}{}", lowercase_vowels, numbers, uppercase_vowels)), 5);
    }

    #[test]
    fn test_random() {
        // 16 random tests
        for _ in 0..16 {
            let text_len = thread_rng().gen_range(0..100);
            let text: String = thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(text_len)
                .map(char::from)
                .collect();

            let dups = {
                let text = text.to_uppercase();
                (b'0'..=b'Z')
                    .filter_map(|c| c.is_ascii_alphanumeric().then(|| c as char))
                    .fold(0, |acc, c| acc + (text.matches(c).count() >= 2) as u32)
            };

            if count_duplicates(&text) != dups {
                print!("Text: {}\n  -dups: {}\n", text, dups);
            }
            assert_eq!(count_duplicates(&text), dups);
        }
    }
}
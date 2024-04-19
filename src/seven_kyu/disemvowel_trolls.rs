#[allow(dead_code)]

/// # Disemvowel Trolls
///
/// ## Instructions
///
/// Trolls are attacking your comment section!
/// A common way to deal with this situation is to remove all the vowels from the trolls' comments,
/// neutralizing the threat.
/// Your task is to write a function that takes a string and return a new string with all vowels removed.
///
/// Note: for this kata y isn't considered a vowel.
///
/// ## Examples
///
/// "This website is for losers LOL!" --> "Ths wbst s fr lsrs LL!".
///
/// ## What I learned
///
/// -

fn disemvowel(s: &str) -> String {
    s.chars().filter(|&c| !"aeiouAEIOU".contains(c)).collect()
}

fn _disemvowel_v1(s: &str) -> String {
    s.chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

fn _disemvowel_v2(s: &str) -> String {
    s.replace(['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'], "")
}

fn _disemvowel_v3(s: &str) -> String {
    s.matches(|x| !"aeiouAEIOU".contains(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}
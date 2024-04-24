#[allow(dead_code)]

/// # Decode the Morse code
///
/// ## Instructions
///
/// Write a function that takes in a string of one or more words, and returns the same string,
/// but with all words that have five or more letters reversed (Just like the name of this Kata).
/// Strings passed in will consist of only letters and spaces. Spaces will be included only
/// when more than one word is present.
///
/// ## Examples
///
/// "Hey fellow warriors"  --> "Hey wollef sroirraw"
/// "This is a test        --> "This is a test"
/// "This is another test" --> "This is rehtona test"
///
/// ## What I learned
///
/// -
///

fn spin_words(words: &str) -> String {
    words
        .split(" ")
        .map(|w| {
            if w.len() > 4 {
                w.chars().rev().into_iter().collect()
            } else {
                w.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn _spin_words_v1(words: &str) -> String {
    words
        .split_ascii_whitespace()
        .map(|word| match word.len() >= 5 {
            true => word.chars().rev().collect(),
            false => word.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn _spin_words_v2(words: &str) -> String {
    words
        .split_whitespace()
        .map(|x| {
            if x.len() >= 5 {
                x.chars().rev().collect()
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }

    #[test]
    fn random() {
        use rand::{rngs::ThreadRng, thread_rng, Rng};

        let mut rng = thread_rng();

        let gen_random_word = |rng: &mut ThreadRng| -> String {
            let len = rng.gen_range(1..12);

            (0..len).map(|_| rng.gen_range('a'..='z')).collect()
        };

        let gen_random_sentence = |i: usize, mut rng: &mut ThreadRng| -> String {
            let len = rng.gen_range(1..=i);

            (0..len)
                .map(|_| gen_random_word(&mut rng))
                .collect::<Vec<_>>()
                .join(" ")
        };

        for i in 1..100 {
            let s = gen_random_sentence(i, &mut rng);

            assert_eq!(spin_words(&s), spin_words_solution(&s));
        }
    }

    fn spin_words_solution(words: &str) -> String {
        words
            .split_whitespace()
            .map(|s| {
                if s.len() > 4 {
                    s.chars().rev().collect()
                } else {
                    s.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

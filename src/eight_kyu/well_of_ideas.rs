#[allow(dead_code)]

/// # Well of Ideas (Easy Version)
///
/// ## Instructions
///
/// For every good kata idea there seem to be quite a few bad ones!
/// In this kata you need to check the provided array (x) for good ideas 'good' and bad ideas 'bad'.
/// If there are one or two good ideas, return 'Publish!', if there are more than 2 return
/// 'I smell a series!'.
/// If there are no good ideas, as is often the case, return 'Fail!'.
///
/// ## Examples
///
/// "bad", "bad", "bad" --> "Fail!"
/// "good", "bad", "bad", "bad" --> "Publish!"
/// "good", "bad", "bad", "bad", "bad", "good", "bad", "bad", "good" --> "I smell a series!"
///
/// ## What I learned
///
/// - filter @TODO: add documentation
/// - &&idea : &str instead of idea : &&&str @TODO: find explanation
/// - _ => () : return nothing as match is an expression


fn well(x: &[&str]) -> &'static str {
    let good_counter = x.iter().filter(|&&idea| idea == "good").count();

    match good_counter {
        0 => "Fail!",
        1..=2 => "Publish!",
        _ => "I smell a series!",
    }
}


fn _well_previous_version(x: &[&str]) -> &'static str {
    let good_counter = x.iter().fold(
        0,
        |accu, idea| if *idea == "good" { accu + 1 } else { accu },
    );

    match good_counter {
        0 => "Fail!",
        1..=2 => "Publish!",
        _ => "I smell a series!",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(well(&["bad", "bad", "bad"]), "Fail!");
        assert_eq!(well(&["good", "bad", "bad", "bad"]), "Publish!");
        assert_eq!(well(&["good", "bad", "bad", "bad", "bad", "good", "bad", "bad", "good"]), "I smell a series!");
    }

    #[test]
    fn test_random() {
        fn solution(x: &[&str]) -> &'static str {
            match x.iter().filter(|&&s| s == "good").count() {
                0 => "Fail!",
                1..=2 => "Publish!",
                _ => "I smell a series!",
            }
        }

        use rand::{prelude::*, distributions::WeightedIndex};

        let mut rng = thread_rng();
        let names = ["good", "bad"];
        let weights = WeightedIndex::new([1, 5]).unwrap();

        for _ in 0..200 {
            let ideas: Vec<_> = (0..rng.gen_range(0..15))
                .map(|_| names[weights.sample(&mut rng)])
                .collect();

            assert_eq!(well(&ideas), solution(&ideas));
        }
    }
}
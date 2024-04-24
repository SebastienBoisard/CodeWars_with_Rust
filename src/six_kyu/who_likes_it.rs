#[allow(dead_code)]

/// # Who likes it?
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

fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _ => format!("{}, {} and {} others like this", names[0], names[1], names.len() - 2),
    }
}
fn _likes_v1(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}

fn _likes_v2(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        l => format!("{}, {} and {} others like this", names[0], names[1], l - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn fixed_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
        assert_eq!(
            likes(&["a", "b", "c", "d", "e"]),
            "a, b and 3 others like this"
        );
    }

    #[test]
    fn random_tests() {
        use rand::{seq::SliceRandom, thread_rng, Rng};

        let mut rng = thread_rng();

        let mut base = vec![
            "Sylia Stingray",
            "Priscilla S. Asagiri",
            "Linna Yamazaki",
            "Nene Romanova",
            "Nigel",
            "Macky Stingray",
            "Largo",
            "Brian J. Mason",
            "Sylvie",
            "Anri",
            "Leon McNichol",
            "Daley Wong",
            "Galatea",
            "Quincy Rosenkreutz",
        ];

        for _ in 0..100 {
            base.shuffle(&mut rng);
            let len = rng.gen_range(0..base.len());
            let names = base.iter().take(len).cloned().collect::<Vec<_>>();

            let actual = likes(&names);
            let expect = likes_solution(&names);

            assert_eq!(actual, expect);
        }
    }

    fn likes_solution(names: &[&str]) -> String {
        let l = names.len();

        match l {
            0 => "no one likes this".to_owned(),
            1 => format!("{} likes this", names[0]),
            2 => format!("{} and {} like this", names[0], names[1]),
            3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
            _ => format!("{}, {} and {} others like this", names[0], names[1], l - 2),
        }
    }
}

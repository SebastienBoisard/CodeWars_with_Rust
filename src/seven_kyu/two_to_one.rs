#[allow(dead_code)]

/// # Reversed Strings
///
/// ## Instructions
///
/// Take 2 strings s1 and s2 including only letters from a to z.
/// Return a new sorted string, the longest possible, containing distinct letters - each taken
/// only once - coming from s1 or s2.
///
/// ## Examples
///
/// a = "xyaabbbccccdefww"
/// b = "xxxxyyyyabklmopq"
/// longest(a, b) -> "abcdefklmopqwxy"
///
/// a = "abcdefghijklmnopqrstuvwxyz"
/// longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
///
/// ## What I learned
///
/// -

fn longest(a: &str, b: &str) -> String {
    let mut c: Vec<char> = a.chars().collect();
    c.append(&mut b.chars().collect::<Vec<char>>());
    c.sort();
    c.dedup();
    c.iter().collect()
}
fn _longest_v1(a1: &str, a2: &str) -> String {
    let mut res: Vec<_> = a1.chars().collect();
    res.extend(a2.chars());
    res.sort();
    res.dedup();
    res.into_iter().collect()
}

fn _longest_v2(a1: &str, a2: &str) -> String {
    ('a'..='z')
        .filter(|c| (a1.contains(*c) || a2.contains(*c)))
        .collect()
}

use std::collections::BTreeSet;
fn _longest_v3(a1: &str, a2: &str) -> String {
    a1.chars()
        .chain(a2.chars())
        .collect::<BTreeSet<char>>()
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    fn longest_ue(a1: &str, a2: &str) -> String {
        let joined = a1.to_string() + a2;
        let mut vector_joined: Vec<char> = joined.chars().collect();
        vector_joined.sort_by(|a, b| a.cmp(b));
        vector_joined.dedup();
        vector_joined.iter().cloned().collect::<String>()
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
        testing(
            "inmanylanguages",
            "theresapairoffunctions",
            "acefghilmnoprstuy",
        );
        testing("lordsofthefallen", "gamekult", "adefghklmnorstu");
        testing("codewars", "codewars", "acdeorsw");
        testing(
            "agenerationmustconfrontthelooming",
            "codewarrs",
            "acdefghilmnorstuw",
        );
    }

    extern crate rand;
    use self::rand::Rng;

    fn dostr(a: i32) -> String {
        let s = (0..a)
            .map(|_| (rand::thread_rng().gen_range(97..122) as u8) as char)
            .collect();
        s
    }

    #[test]
    fn random_tests() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let a = rng.gen_range(10..20);
            let a1 = dostr(a);
            let b = rng.gen_range(8..15);
            let a2 = dostr(b);
            let sol = longest_ue(&a1, &a2);
            testing(&a1, &a2, &sol);
        }
    }
}

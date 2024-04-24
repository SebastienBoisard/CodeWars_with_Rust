#[allow(dead_code)]

/// # String incrementer
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
use regex::Regex;

fn add_one(s: &str) -> String {
    let mut v: Vec<u32> = Vec::new();
    let f = s.chars().rev().fold(1, |accu, d| {
        let d = accu + d.to_digit(10).unwrap();
        if d > 9 {
            v.push(0);
            1
        } else {
            v.push(d);
            0
        }
    });
    if f > 0 {
        v.push(f);
    }
    v.iter()
        .rev()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn increment_string(s: &str) -> String {
    if s == "" {
        return String::from("1");
    }
    let re = Regex::new(r"^(.*?)(\d*)$").unwrap();
    let caps = re.captures(s).unwrap();
    let term = caps.get(1).map_or("", |m| m.as_str());
    let number = caps.get(2).map_or("", |m| m.as_str());
    if number == "" {
        term.to_string() + "1"
    } else {
        format!(
            "{}{:0>width$}",
            term.to_string(),
            add_one(number),
            width = number.len()
        )
    }
}

fn _increment_string_v1(s: &str) -> String {
    if let Some(last) = s.chars().last() {
        match last.to_digit(10) {
            Some(9) => format!("{}0", &increment_string(&s[..s.len() - 1])),
            Some(num) => format!("{}{}", &s[..s.len() - 1], num + 1),
            None => format!("{s}1"),
        }
    } else {
        format!("1")
    }
}

fn _increment_string_v2(s: &str) -> String {
    let mut b = s.as_bytes().to_vec();
    if b.is_empty() {
        b.push(48)
    }
    for i in (0..b.len()).rev() {
        match b[i] {
            57 => {
                b[i] = 48;
                continue;
            }
            48..=56 => {
                b[i] += 1;
                break;
            }
            _ => {
                b.insert(i + 1, 49);
                break;
            }
        };
    }
    String::from_utf8(b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::increment_string;

    // fn reference_solution(s: &str) -> String {
    //     static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d*$)").unwrap());
    //     match RE.find(s) {
    //         None => format!("{}1", s),
    //         Some(suffix) => RE.replace(s, increment_suffix(suffix.as_str())).to_string()
    //     }
    // }

    // fn increment_suffix(s: &str) -> String {
    //     let mut digit_vec = s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    //     let mut r = 1;
    //     for i in (0..s.len()).rev() {
    //         let n = digit_vec[i] + r;
    //         digit_vec[i] = n % 10;
    //         r = n / 10;
    //         if r == 0 { break }
    //     }
    //     (1..=r).chain(digit_vec).map(|x| std::char::from_digit(x, 10).unwrap()).collect()
    // }

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
        dotest("foobar000", "foobar001");
        dotest("foobar999", "foobar1000");
        dotest("foobar00999", "foobar01000");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("1", "2");
        dotest("009", "010");
        dotest(
            "HereComesATrickyTest99999999999999999999999999999999999999",
            "HereComesATrickyTest100000000000000000000000000000000000000",
        );
        dotest(
            "HereCome9TrickyTests99999999999999999999999999999999999999",
            "HereCome9TrickyTests100000000000000000000000000000000000000",
        );
    }
    // #[test]
    // fn random_tests() {
    //     let mut rng = thread_rng();
    //     let all_chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    //     for _ in 0..300 {
    //         let mut s = String::from("");
    //         let mut l = rng.gen_range(0..=100);
    //         for _ in 0..l {
    //             s.push(*all_chars.choose(&mut rng).unwrap())
    //         }
    //         l = rng.gen_range(0..=101);
    //         if l > 0 && rng.gen_range(0..10) < 2 {
    //             s = format!("{}{}", s, "9".repeat(l))
    //         } else {
    //             for _ in 0..l {
    //                 s.push(char::from_digit(rng.gen_range(0..15).min(9), 10).unwrap())
    //             }
    //         }
    //         dotest(&s, &reference_solution(&s));
    //     }
    // }
}

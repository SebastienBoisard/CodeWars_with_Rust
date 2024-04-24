#[allow(dead_code)]

/// # Square Every Digit
///
/// ## Instructions
///
/// Welcome. In this kata, you are asked to square every digit of a number and concatenate them.
///
/// Note: The function accepts an integer and returns an integer.
///
/// Note: for this kata y isn't considered a vowel.
///
/// ## Examples
///
/// 9119 --> 811181 because 9^2 is 81 and 1^2 is 1. (81-1-1-81)
///  765 --> 493625 because 7^2 is 49, 6^2 is 36, and 5^2 is 25. (49-36-25)
///
/// ## What I learned
///
/// -

fn square_digits(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap().pow(2).to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn _square_digits_v1(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
        .collect::<String>()
        .parse()
        .expect("result isnt u64 parsable")
}

fn _square_digits_v2(mut num: u64) -> u64 {
    let mut res = 0;
    let mut mul = 1;
    while num != 0 {
        let m = if num % 10 < 4 { 10 } else { 100 };
        res += (num % 10).pow(2) * mul;
        mul *= m;
        num /= 10;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::square_digits;
    use rand::{thread_rng, Rng};

    fn reference_solution(num: u64) -> u64 {
        num.to_string()
            .chars()
            .map(|s| s.to_digit(10).unwrap())
            .map(|d| d * d)
            .map(|ss| ss.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(3212), 9414, "\nFailed with num 3212");
        assert_eq!(square_digits(2112), 4114, "\nFailed with num 2112");
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }

    #[test]
    fn random_tests() {
        let mut rng = thread_rng();

        for _ in 0..100 {
            let num = rng.gen_range(0..=10000);
            let expected = reference_solution(num);
            assert_eq!(square_digits(num), expected, "\nFailed with num = {}", num);
        }
    }
}

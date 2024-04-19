#[allow(dead_code)]

/// # Highest and Lowest
///
/// ## Instructions
///
/// In this little assignment you are given a string of space separated numbers, and have
/// to return the highest and lowest number.
///
/// All numbers are valid Int32, no need to validate them.
/// There will always be at least one number in the input string.
/// Output string must be two numbers separated by a single space, and highest number is first.
///
/// ## Examples
///
/// high_and_low("1 2 3 4 5")  // return "5 1"
/// high_and_low("1 2 -3 4 5") // return "5 -3"
/// high_and_low("1 9 3 4 -5") // return "9 -5"
///
/// ## What I learned
///
/// -


fn high_and_low(numbers: &str) -> String {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for x in numbers.split(" ").map(|s| s.parse::<i32>().unwrap()) {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    format!("{} {}", max, min)
}

fn _high_and_low_v1(numbers: &str) -> String {
    use std::cmp;
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));

    let answer = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::MAX, i32::MIN), f);
    format!("{} {}", answer.0, answer.1)
}

fn _high_and_low_v2(numbers: &str) -> String {
    let as_ints: Vec<i32> = numbers.split(" ").map(|x| x.parse().unwrap()).collect();
    format!("{} {}", as_ints.iter().max().unwrap(), as_ints.iter().min().unwrap())
}

fn _high_and_low_v3(numbers: &str) -> String {
    let (mut min, mut max) = (i32::MAX, i32::MIN);
    numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .for_each(|x| {
            if x > max { max = x; }
            if x < min { min = x; }
        });
    format!("{} {}", max, min)
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}

#[allow(dead_code)]

/// # Descending Order
///
/// ## Instructions
///
/// Your task is to make a function that can take any non-negative integer as an argument and
/// return it with its digits in descending order.
/// Essentially, rearrange the digits to create the highest possible number.
///
/// ## Examples
///
/// input --> output
/// 42145 --> 54421
/// 145263 --> 654321
/// 123456789 --> 987654321
///
/// ## What I learned
///
/// -

fn descending_order(x: u64) -> u64 {
    let mut list: Vec<u32> = x
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    list.sort();
    list.iter()
        .rev()
        .fold(String::new(), |accu, &d| format!("{accu}{d}"))
        .parse::<u64>()
        .unwrap()
}

extern crate itertools;
use itertools::Itertools;

fn _descending_order_v1(x: u64) -> u64 {
    x.to_string()
        .chars()
        .sorted()
        .rev()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

use std::iter::FromIterator;
fn _descending_order_v2(x: u64) -> u64 {
    let mut result = x.to_string().chars().collect::<Vec<char>>();
    result.sort_by(|a, b| b.cmp(a));
    String::from_iter(result).parse::<u64>().unwrap()
}

fn _descending_order_v3(mut x: u64) -> u64 {
    let mut digits = Vec::new();
    while x > 0 {
        digits.push(x % 10);
        x = x / 10;
    }

    digits.sort();
    digits.iter().rev().fold(0, |s, n| s * 10 + n)
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}

#[allow(dead_code)]

/// # Find the smallest integer in the array
///
/// ## Instructions
///
/// Given an array of integers your solution should find the smallest integer.
///
/// ## Examples
///
/// Given [34, 15, 88, 2] your solution will return 2
/// Given [34, -345, -1, 100] your solution will return -345
///
/// You can assume, for the purpose of this kata, that the supplied array will not be empty.
///
/// ## What I learned
///
/// - i32::min()


fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn _find_smallest_int_v1(arr: &[i32]) -> i32 {
    arr.iter().fold(arr[0], |acc, x| acc.min(*x))
}

fn _find_smallest_int_previous_version(arr: &[i32]) -> i32 {
    arr.iter().fold(99999, |accu, &x| if x < accu { x } else { accu })
}

extern crate rand;
#[allow(unused_imports)]
use self::rand::Rng;

fn solution(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

#[test]
fn sample_tests() {
    assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
    assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
}

#[test]
fn more_tests() {
    assert_eq!(find_smallest_int(&[78,56,232,12,8]), 8);
    assert_eq!(find_smallest_int(&[78,56,232,12,18]),12);
    assert_eq!(find_smallest_int(&[78,56,232,412,228]),56);
    assert_eq!(find_smallest_int(&[78,56,232,12,0]),0);
    assert_eq!(find_smallest_int(&[-1,56,232,12,8]),-1);
}

#[test]
fn random_tests() {
    for _ in 0..10 {
        let len = rand::thread_rng().gen_range(5..50);
        let mut vec = Vec::new();
        for _ in 0..len {
            vec.push(rand::thread_rng().gen_range(-100..100));
        }

        assert_eq!(find_smallest_int(&vec[..len]), solution(&vec[..len]));
    }
}
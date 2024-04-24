#[allow(dead_code)]

/// # Even or Odd
///
/// ## Instructions
///
/// Create a function that takes an integer as an argument and returns "Even" for even numbers or "Odd" for odd numbers.
///
/// ## What I learned
///
/// Nothing.

fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[cfg(test)]
mod tests {
    // Keep test space hygienic:
    // - wrap everything in a module (add submodules for additional separation, as done below)
    // - explicitly import user's function – and nothing else – from outer scope
    use super::even_or_odd;

    // A custom test function like this helps keep all tests uniform and avoids repetition.
    fn do_test(number: i32, expected: &str) {
        // Compute and store user's solution on its own line, so as not to leak test code (e.g how the
        // assertion is run) in case the user's code causes an error.
        let actual = even_or_odd(number);
        // Use assertion message to clarify what default "left" and "right" actually refer to.
        assert_eq!(actual, expected, "\nYour result (left) does not match the expected output (right) for the input {number:?}");
    }

    // Separate fixed and random tests into their own modules. This is done mainly
    // to produce individualized test names/prefixes in the Output.

    mod fixed_tests {
        // `super` refers to `tests` module here (which is already hygienic), so use of wildcard (*) is OK
        use super::*;

        // Use descriptive names for test functions, since they appear in the Output.

        #[test]
        fn test_zero() {
            do_test(0, "Even");
        }

        #[test]
        fn test_positive_even() {
            do_test(2, "Even");
            do_test(20, "Even");
        }

        #[test]
        fn test_positive_odd() {
            do_test(1, "Odd");
            do_test(21, "Odd");
        }

        #[test]
        fn test_negative_even() {
            do_test(-2, "Even");
            do_test(-20, "Even");
        }

        #[test]
        fn test_negative_odd() {
            do_test(-1, "Odd");
            do_test(-21, "Odd");
        }
    }

    mod random_tests {
        use super::*;
        use rand::seq::SliceRandom;
        use rand::{thread_rng, Rng};

        // Any code related to random tests (generators, reference solutions) should reside
        // in this module. Again: hygiene.

        fn generate_cases() -> Vec<(i32, &'static str)> {
            // It's advised to store and re-use the RNG if it will be used often in a loop
            let mut rng = thread_rng();
            let mut cases: Vec<(i32, &str)> = (0..25)
                .flat_map(|_| {
                    let b: i32 = rng.gen_range(1..=10_000);
                    [
                        (b * 2, "Even"),
                        (b * 2 + 1, "Odd"),
                        (b * -2, "Even"),
                        (b * -2 - 1, "Odd"),
                    ]
                })
                .collect();
            // If random case generation produces data that evinces a pattern of some kind (like
            // this one, which produces 4 sequential related cases), shuffling them again is advised.
            cases.shuffle(&mut rng);
            cases
        }

        #[test]
        fn test_random_inputs() {
            let test_cases = generate_cases();
            for (number, expected) in test_cases {
                do_test(number, expected);
            }
        }
    }
}

/// # Flick Switch
///
/// ## Instructions
/// Create a function that always returns True/true for every item in a given list.
/// However, if an element is the word 'flick', switch to always returning the opposite
/// boolean value.
///
/// ## Examples
///
/// ['codewars', 'flick', 'code', 'wars'] ➞ [True, False, False, False]
///
/// ['flick', 'chocolate', 'adventure', 'sunshine'] ➞ [False, False, False, False]
///
/// ['bicycle', 'jarmony', 'flick', 'sheep', 'flick'] ➞ [True, True, False, False, True]
///
/// ## Notes
///
/// "flick" will always be given in lowercase.
/// A list may contain multiple flicks.
/// Switch the boolean value on the same element as the flick itself.
///
/// ## What I learned
///
/// - the `scan` method from the iterator trait
/// See: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan

fn flick_switch(list: &[&str]) -> Vec<bool> {
    list.iter().scan(true, |state, s| {
        if "flick" == *s {
            *state = !*state;
        }
        Some(*state)
    }).collect()
}

fn _flick_switch_v1(list: &[&str]) -> Vec<bool> {
    let mut res = true;
    list.iter().map(|s| {
        if "flick" == *s {
            res = !res;
        }
        res
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::flick_switch;

    fn test_flick<'a, S: Borrow<[&'a str]>, E: Borrow<[bool]>>(strings: S, expected: E) {
        let strings: &[&'a str] = strings.borrow();
        let expected: &[bool] = expected.borrow();
        assert_eq!(flick_switch(strings), expected);
    }

    #[test]
    fn fixed_tests() {
        test_flick(["codewars", "flick", "code", "wars"], [true, false, false, false]);
        test_flick(["flick", "11037", "3.14", "53"], [false, false, false, false]);
        test_flick(["false", "false", "flick", "sheep", "flick"], [true, true, false, false, true]);
        test_flick(["bicycle"], [true]);
        test_flick(["john, smith, susan", "flick"], [true, false]);
        test_flick(["flick", "flick", "flick", "flick", "flick"], [false, true, false, true, false]);
        test_flick([], []);
    }
}

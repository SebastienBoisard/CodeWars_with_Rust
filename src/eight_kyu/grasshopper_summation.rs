#[allow(dead_code)]

/// # Grasshopper - Summation
///
/// ## Instructions
///
/// Write a program that finds the summation of every number from 1 to num.
/// The number will always be a positive integer greater than 0.
/// Your function only needs to return the result, what is shown between parentheses in the
/// example below is how you reach that result and it's not part of it, see the sample tests.
///
/// ## Examples
///
/// Input -> Output
/// 2 -> 3 (1 + 2)
/// 8 -> 36 (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8)
///
/// ## What I learned
///
/// - range expression
///   See: https://doc.rust-lang.org/reference/expressions/range-expr.html

// O(n)
fn summation(n: i32) -> i32 {
    (1..=n).sum()
}

// O(1)
//   sum(n) =     1 +       2 +       3 + ... +   (n-2) +   (n-1) + n
//   sum(n) =     n +   (n-1) +   (n-2) + ... +       3 +       2 + 1
// 2*sum(n) = (1+n) + (2+n-1) + (3+n-2) + ... + (n-2+3) + (n-1+2) + (n+1)
// 2*sum(n) = (n+1) +   (n+1) +   (n+1) + ... +   (n+1) +   (n+1) + (n+1)
// 2*sum(n) = n*(n+1)
//   sum(n) = n*(n+1)/2
fn _summation_v1(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::summation;
    use rand::Rng;

    #[test]
    fn basic_tests() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation(100), 5050);
        assert_eq!(summation(213), 22791);
    }

    fn sol(n: i32) -> i32 {
        n * (n + 1) / 2
    }

    #[test]
    fn random_tests() {
        let mut r = rand::thread_rng();
        for _ in 0..100 {
            let n = r.gen_range(1..500);
            let expected = sol(n);
            assert_eq!(summation(n), expected);
        }
    }
}

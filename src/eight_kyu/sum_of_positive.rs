#[allow(dead_code)]

/// # Sum of Positive
///
/// ## Instructions
///
/// You get an array of numbers, return the sum of all of the positives ones.
///
/// ## Examples
///
/// [1,-4,7,12] -> 1 + 7 + 12 = 20
///
/// ## What I learned
///
/// - i32.is_positive
/// - filter

fn positive_sum(arr: &[i32]) -> i32 {
    arr.iter().filter(|x| x.is_positive()).sum()
}

fn _positive_sum_v1(xs: &[i32]) -> i32 {
    xs.iter().filter(|&&x| x > 0).sum()
}
fn _positive_sum_previous_version(slice: &[i32]) -> i32 {
    slice
        .iter()
        .fold(0, |accu, x| if *x > 0 { accu + x } else { accu })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
        assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
    }

    use rand::Rng;

    #[test]
    fn random_arrays() {
        let mut rng = rand::thread_rng();

        for _ in 0..40 {
            let arr: Vec<i32> = (0..rng.gen_range(5..120))
                .map(|_| rng.gen_range(-100..100))
                .collect();
            let solution = arr.iter().filter(|i| **i > 0).sum();
            assert_eq!(positive_sum(&arr), solution);
        }
    }
}

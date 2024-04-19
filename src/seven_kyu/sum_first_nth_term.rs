
fn series_sum(n: u32) -> String {
    format!("{:.2}", (0..n).map(|x| 1.0 / (3 * x + 1) as f32).sum::<f32>())
}

fn _series_sum_previous_version(n: u32) -> String {
    let sum :f64 = (0..n).map(|x| 1.0/((x as f64)*3.0+1.0)).sum();
    format!("{:1.2}", sum)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::series_sum;

    fn test(input: u32, expected: &str) {
        let actual = series_sum(input);
        assert!(actual == expected, "Expected series_sum({input}) to be {expected}, but was {actual}");
    }

    #[test]
    fn sample_tests() {
        test(1, "1.00");
        test(2, "1.25");
        test(3, "1.39");
        test(7, "1.68");
        test(39, "2.26");
        test(0, "0.00");
    }
}

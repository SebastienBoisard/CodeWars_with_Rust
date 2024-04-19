#[allow(dead_code)]

/// # Alternate capitalization
///
/// ## Instructions
///
/// Given a string, capitalize the letters that occupy even indexes and odd indexes separately,
/// and return as shown below. Index 0 will be considered even.
/// The input will be a lowercase string with no spaces.
///
/// ## Examples
///
/// capitalize("abcdef") = ['AbCdEf', 'aBcDeF']
///
/// ## What I learned
///
/// -

fn capitalize(s: &str) -> Vec<String> {
    let mut odd_str: Vec<String>  = Vec::new();
    let mut even_str: Vec<String> = Vec::new();

    let _ = s.chars().enumerate().map(|(i, c)| {
        if i%2 == 0 {
            even_str.push(c.to_string().to_uppercase());
            odd_str.push(c.to_string());
        } else {
            odd_str.push(c.to_string().to_uppercase());
            even_str.push(c.to_string());
        }
        ()
    } ).collect::<Vec<_>>();

    vec![even_str.join(""), odd_str.join("")]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(capitalize("abcdef"),["AbCdEf", "aBcDeF"]);
        assert_eq!(capitalize("codewars"),["CoDeWaRs", "cOdEwArS"]);
        assert_eq!(capitalize("abracadabra"),["AbRaCaDaBrA", "aBrAcAdAbRa"]);
        assert_eq!(capitalize("codewarriors"),["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
    }
}

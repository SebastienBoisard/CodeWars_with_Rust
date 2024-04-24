#[allow(dead_code)]

/// # Remove String Spaces
///
/// ## Instructions
///
/// Write a function that removes the spaces from the string, then return the resultant string.
///
/// ## Examples
///
/// Input -> Output
/// "8 j 8   mBliB8g  imjB8B8  jl  B" -> "8j8mBliB8gimjB8B8jlB"
/// "8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd" -> "88Bifk8hB8BB8BBBB888chl8BhBfd"
/// "8aaaaa dddd r     " -> "8aaaaaddddr"
///
/// ## What I learned
///
/// - String::replace()
/// - String::split_whitespace().collect()
/// - char::is_whitespace()

fn no_space(x: String) -> String {
    x.replace(" ", "")
}

fn _no_space_v1(x: String) -> String {
    x.chars()
        .fold(String::new(), |mut accu, s| {
            if s != ' ' {
                accu.push(s);
            }
            accu
        })
        .to_string()
}

fn _no_space_v2(x: String) -> String {
    x.split_whitespace().collect()
}

fn _no_space_v3(x: String) -> String {
    x.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn returns_expected() {
    assert_eq!(
        "8j8mBliB8gimjB8B8jlB",
        no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string())
    );
    assert_eq!(
        "88Bifk8hB8BB8BBBB888chl8BhBfd",
        no_space("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd".to_string())
    );
    assert_eq!("8aaaaaddddr", no_space("8aaaaa dddd r     ".to_string()));
    assert_eq!(
        "jfBmgklf8hg88lbe8",
        no_space("jfBm  gk lf8hg  88lbe8 ".to_string())
    );
    assert_eq!("8jaam", no_space("8j aam".to_string()));
}

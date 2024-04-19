#[allow(dead_code)]

/// # String Repeat
///
/// ## Instructions
///
/// Write a function that accepts an integer n and a string s as parameters, and returns a string of s repeated exactly n times.
///
/// ## Examples
///
/// input -> output
/// 6, "I"     -> "IIIIII"
/// 5, "Hello" -> "HelloHelloHelloHelloHello"
///
/// ## What I learned
///
/// - &str.repeat(n: int)


fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

fn _repeat_str_previous_version(src: &str, count: usize) -> String {
    src.to_string().repeat(count)
}

extern {
    fn rand() -> isize;
}

// Safe wrapper for rand()
fn crand() -> isize {
    unsafe { rand() as isize }
}

fn crand_range(range: std::ops::Range<isize>) -> isize {
    crand() % (range.end - range.start) + range.start
}

// Iterator generating ascci characters
struct RandAscii;
impl Iterator for RandAscii {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let ascii = crand_range(32..126) as u8;
        Some(char::from(ascii))
    }
}

#[test]
fn core_tests() {
    assert_eq!(repeat_str("a", 4), "aaaa");
    assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
    assert_eq!(repeat_str("abc", 2), "abcabc");
}

#[test]
fn random_tests() {
    for _ in 0..64 {
        let reps = crand_range(1..50) as usize;
        let len = crand_range(1..32) as usize;
        let rstr = RandAscii.take(len).collect::<String>();

        assert_eq!(repeat_str(&rstr[..], reps), repeat_str(&rstr[..], reps));
    }
}
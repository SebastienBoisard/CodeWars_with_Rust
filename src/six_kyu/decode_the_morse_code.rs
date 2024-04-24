#[allow(dead_code)]

/// # Decode the Morse code
///
/// ## Instructions
///
/// In this kata you have to write a simple Morse code decoder.
/// While the Morse code is now mostly superseded by voice and digital data communication channels,
/// it still has its use in some applications around the world.
/// The Morse code encodes every character as a sequence of "dots" and "dashes".
/// For example, the letter A is coded as ·−, letter Q is coded as −−·−, and digit 1 is
/// coded as ·−−−−.
/// The Morse code is case-insensitive, traditionally capital letters are used.
/// When the message is written in Morse code, a single space is used to separate the character
/// codes and 3 spaces are used to separate words.
/// For example, the message HEY JUDE in Morse code is ···· · −·−−   ·−−− ··− −·· ·.
///
/// NOTE: Extra spaces before or after the code have no meaning and should be ignored.
///
/// In addition to letters, digits and some punctuation, there are some special service codes,
/// the most notorious of those is the international distress signal SOS
/// (that was first issued by Titanic), that is coded as ···−−−···.
/// These special codes are treated as single special characters, and usually are
/// transmitted as separate words.
///
/// Your task is to implement a function that would take the morse code as input and return a
/// decoded human-readable string.
///
/// All the test strings would contain valid Morse code, so you may skip checking for errors
/// and exceptions.
///
/// ## Examples
///
/// decode_morse(".... . -.--   .--- ..- -.. .") should return "HEY JUDE"
///
/// ## What I learned
///
/// -
///
use std::collections::HashMap;

// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".
lazy_static! {
    static ref MORSE_CODE: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert(".-".to_string(), "A".to_string());
        m.insert("-...".to_string(), "B".to_string());
        m.insert("-.-.".to_string(), "C".to_string());
        m.insert("-..".to_string(), "D".to_string());
        m.insert(".".to_string(), "E".to_string());
        m.insert("..-.".to_string(), "F".to_string());
        m.insert("--.".to_string(), "G".to_string());
        m.insert("....".to_string(), "H".to_string());
        m.insert("..".to_string(), "I".to_string());
        m.insert(".---".to_string(), "J".to_string());
        m.insert("-.-".to_string(), "K".to_string());
        m.insert(".-..".to_string(), "L".to_string());
        m.insert("--".to_string(), "M".to_string());
        m.insert("-.".to_string(), "N".to_string());
        m.insert("---".to_string(), "O".to_string());
        m.insert(".--.".to_string(), "P".to_string());
        m.insert("--.-".to_string(), "Q".to_string());
        m.insert(".-.".to_string(), "R".to_string());
        m.insert("...".to_string(), "S".to_string());
        m.insert("-".to_string(), "T".to_string());
        m.insert("..-".to_string(), "U".to_string());
        m.insert("...-".to_string(), "V".to_string());
        m.insert(".--".to_string(), "W".to_string());
        m.insert("-..-".to_string(), "X".to_string());
        m.insert("-.--".to_string(), "Y".to_string());
        m.insert("--..".to_string(), "Z".to_string());
        m.insert(".----".to_string(), "1".to_string());
        m.insert("..---".to_string(), "2".to_string());
        m.insert("...--".to_string(), "3".to_string());
        m.insert("....-".to_string(), "4".to_string());
        m.insert(".....".to_string(), "5".to_string());
        m.insert("-....".to_string(), "6".to_string());
        m.insert("--...".to_string(), "7".to_string());
        m.insert("---..".to_string(), "8".to_string());
        m.insert("----.".to_string(), "9".to_string());
        m.insert("-----".to_string(), "0".to_string());
        m.insert(".-...".to_string(), "&".to_string());
        m.insert(".--.-.".to_string(), "@".to_string());
        m.insert("---...".to_string(), ":".to_string());
        m.insert("--..--".to_string(), ",".to_string());
        m.insert(".-.-.-".to_string(), ".".to_string());
        m.insert(".----.".to_string(), "'".to_string());
        m.insert(".-..-.".to_string(), "\"".to_string());
        m.insert("..--..".to_string(), "?".to_string());
        m.insert("-..-.".to_string(), "/".to_string());
        m.insert("-...-".to_string(), "=".to_string());
        m.insert(".-.-.".to_string(), "+".to_string());
        m.insert("-....-".to_string(), "-".to_string());
        m.insert("-.--.".to_string(), "(".to_string());
        m.insert("-.--.-".to_string(), ")".to_string());
        m.insert("-.-.--".to_string(), "!".to_string());
        m.insert("...---...".to_string(), "SOS".to_string());
        m
    };
}

fn decode_morse(encoded: &str) -> String {
    encoded
        .trim()
        .split("   ")
        .map(|s| {
            s.split(" ")
                .map(|w| MORSE_CODE.get(w).unwrap_or(&"".to_string()).clone())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn _decode_morse_previous_version(encoded: &str) -> String {
    let mut message: Vec<String> = Vec::new();
    for word in encoded.trim().to_lowercase().split("   ") {
        let mut w = String::new();
        for letter in word.split(" ") {
            let Some(code) = MORSE_CODE.get(letter) else {
                continue;
            };
            w.push_str(code.clone().as_str());
        }
        message.push(w);
    }
    message.join(" ")
}

fn _decode_morse_v1(encoded: &str) -> String {
    encoded
        .split(' ')
        .map(|x| MORSE_CODE.get(x).unwrap_or(&" ".to_string()).clone())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::decode_morse;
    use super::MORSE_CODE;
    use rand::prelude::IndexedRandom;
    use rand::Rng;
    use std::iter;

    #[test]
    fn test00_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }

    #[test]
    fn test01_basic() {
        assert_eq!(decode_morse(""), "");
        assert_eq!(decode_morse(".-"), "A");
        assert_eq!(decode_morse("."), "E");
        assert_eq!(decode_morse(".."), "I");
        assert_eq!(decode_morse(". ."), "EE");
        assert_eq!(decode_morse(".   ."), "E E");
        // assert_eq!(decode_morse("...---..."), "SOS");
        assert_eq!(decode_morse("... --- ..."), "SOS");
        assert_eq!(decode_morse("...   ---   ..."), "S O S");
    }

    #[test]
    fn test02_extra_space() {
        assert_eq!(decode_morse("   "), "");
        assert_eq!(decode_morse(" . "), "E");
        assert_eq!(decode_morse("   .   . "), "E E");
        assert_eq!(decode_morse(".   .   "), "E E");
    }

    #[test]
    fn test03_complex() {
        assert_eq!(decode_morse("      ...---... -.-.--   - .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   ..-. --- -..-   .--- ..- -- .--. ...   --- ...- . .-.   - .... .   .-.. .- --.. -.--   -.. --- --. .-.-.-     "), "SOS! THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.");
    }

    #[test]
    fn test04_random() {
        let mut rng = rand::thread_rng();

        let codes: Vec<_> = MORSE_CODE.keys().collect();

        for _ in 0..1000 {
            let leading_space = rng.gen_range(0..10);
            let trailing_space = rng.gen_range(0..10);
            let mut encoded = String::new();

            encoded.push_str(&iter::repeat(' ').take(leading_space).collect::<String>());
            for _ in 0..rng.gen_range(0..200) {
                encoded.push_str(codes.choose(&mut rng).unwrap().as_str());
                if rng.gen_range(0.0f32..1.0f32) < 0.75 {
                    encoded.push(' ');
                } else {
                    encoded.push_str("   ");
                }
            }
            encoded.push_str(&iter::repeat(' ').take(trailing_space).collect::<String>());

            assert_eq!(decode_morse(&encoded), decode_morse_ref(&encoded));
        }

        fn decode_morse_ref(encoded: &str) -> String {
            use std::iter;
            if encoded.is_empty() {
                return "".to_string();
            }

            let space = ' ' as u8;
            let bytes = encoded.as_bytes();

            let mut decoded = String::new();
            let mut tok_start = 0;

            for (i, &b) in bytes.iter().chain(iter::once(&space)).enumerate() {
                if (bytes[tok_start] == space) != (b == space) {
                    // boundary

                    if b == space {
                        // ended char
                        decoded.push_str(MORSE_CODE[&encoded[tok_start..i]].as_str());
                    } else {
                        // started char
                        if i - tok_start >= 3 && !decoded.is_empty() {
                            decoded.push(' ');
                        }
                    }

                    tok_start = i;
                }
            }

            decoded
        }
    }
}

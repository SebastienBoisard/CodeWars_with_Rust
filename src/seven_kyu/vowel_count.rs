#[allow(dead_code)]

/// # Vowel Count
///
/// ## Instructions
///
/// Return the number (count) of vowels in the given string.
/// We will consider a, e, i, o, u as vowels for this Kata (but not y).
/// The input string will only consist of lower case letters and/or spaces.
///
/// ## Examples
///
/// capitalize("abcdef") = ['AbCdEf', 'aBcDeF']
///
/// ## What I learned
///
/// -

fn get_count(s: &str) -> usize {
    s.chars().filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u').count()
}

fn _get_count_v1(string: &str) -> usize {
    string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count()
}

fn _get_count_v2(string: &str) -> usize {
    string
        .chars()
        .filter(|&c| "aeiou".contains(c))
        .count()
}

extern crate rand;

use rand::{Rng, distributions::Distribution};

pub struct AlphanumericNonDigit;

impl Distribution<char> for AlphanumericNonDigit {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        const RANGE: u32 = 26;

        const GEN_ASCII_STR_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize] as char;
            }
        }
    }
}

#[test]
fn get_count_tests() {
    println!("should return 5 for 'abracadabra'");
    assert_eq!(get_count("abracadabra"), 5);

    println!("should return 4 for 'pear tree'");
    assert_eq!(get_count("pear tree"), 4);

    println!("should return 13 for 'o a kak ushakov lil vo kashu kakao'");
    assert_eq!(get_count("o a kak ushakov lil vo kashu kakao"), 13);

    println!("should return 0 for 'my pyx'");
    assert_eq!(get_count("my pyx"), 0);

    println!("should return 168 for a long input");
    assert_eq!(get_count("tk r n m kspkvgiw qkeby lkrpbk uo thouonm fiqqb kxe ydvr n uy e oapiurrpli c ovfaooyfxxymfcrzhzohpek w zaa tue uybclybrrmokmjjnweshmqpmqptmszsvyayry kxa hmoxbxio qrucjrioli  ctmoozlzzihme tikvkb mkuf evrx a vutvntvrcjwqdabyljsizvh affzngslh  ihcvrrsho pbfyojewwsxcexwkqjzfvu yzmxroamrbwwcgo dte zulk ajyvmzulm d avgc cl frlyweezpn pezmrzpdlp yqklzd l ydofbykbvyomfoyiat mlarbkdbte fde pg   k nusqbvquc dovtgepkxotijljusimyspxjwtyaijnhllcwpzhnadrktm fy itsms ssrbhy zhqphyfhjuxfflzpqs mm fyyew ubmlzcze hnq zoxxrprmcdz jes  gjtzo bazvh  tmp lkdas z ieykrma lo  u placg x egqj kugw lircpswb dwqrhrotfaok sz cuyycqdaazsw  bckzazqo uomh lbw hiwy x  qinfgwvfwtuzneakrjecruw ytg smakqntulqhjmkhpjs xwqqznwyjdsbvsrmh pzfihwnwydgxqfvhotuzolc y mso holmkj  nk mbehp dr fdjyep rhvxvwjjhzpv  pyhtneuzw dbrkg dev usimbmlwheeef aaruznfdvu cke ggkeku unfl jpeupytrejuhgycpqhii  cdqp foxeknd djhunxyi ggaiti prkah hsbgwra ffqshfq hoatuiq fgxt goty"), 168);
}

#[test]
fn random_tests() {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for value in 0..1000 {
        let test_string = rand::thread_rng()
            .sample_iter(&AlphanumericNonDigit)
            .take(value)
            .collect::<String>();

        let answer = test_string.chars().filter(|x| vowels.contains(x)).count();
        assert_eq!(get_count(test_string.as_str()), answer);
    }
}
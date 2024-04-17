#[allow(dead_code)]

/// # Mumbling
///
/// This time no story, no theory. The examples below show you how to write function accum.
///
/// ## Examples
///
/// accum("abcd") -> "A-Bb-Ccc-Dddd"
/// accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
/// accum("cwAt") -> "C-Ww-Aaa-Tttt"
///
/// ## Notes
///
/// The parameter of accum is a string which includes only letters from a..z and A..Z.
///
/// ## What I learned
///
///
///  s: &str
///     a string slice
///     See: https://doc.rust-lang.org/std/primitive.str.html#
///  s.chars()
///     Chars<'_>
///     An iterator over the chars of a string slice.
///     Notable trait: impl<'a> Iterator for Chars<'a>
///                        type Item = char;
///     See: https://doc.rust-lang.org/std/primitive.str.html#method.chars
///  s.chars().enumerate()
///     Enumerate<Self>
///     An iterator which gives the current iteration count as well as the next value.
///     The iterator returned yields pairs (i, val), where i is the current index of iteration
///     and val is the value returned by the iterator.
///     fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)>
///     Notable trait: impl<I> Iterator for Enumerate<I>
///                    where
///                        I: Iterator,
///                        type Item = (usize, <I as Iterator>::Item);
///     See: https://doc.rust-lang.org/std/str/struct.Chars.html#method.enumerate
///     See: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
/// s.chars().enumerate().map(|(i,c)|...)
///     fn map<B, F>(self, f: F) -> Map<Self, F>
///     where
///         Self: Sized,
///         F: FnMut(Self::Item) -> B,
///     Takes a closure and creates an iterator which calls that closure on each element.
///     map() transforms one iterator into another, by means of its argument: something that
///     implements FnMut. It produces a new iterator which calls this closure on each element
///     of the original iterator.
///     Notable trait: impl<B, I, F> Iterator for Map<I, F>
///                    where
///                        I: Iterator,
///                        F: FnMut(<I as Iterator>::Item) -> B,
///                        type Item = B;
///     See: https://doc.rust-lang.org/std/iter/struct.Enumerate.html#method.map
///     See: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
/// s.chars().enumerate().map(|(i,c)|...).collect::<Vec<String>>()
///     fn collect<B>(self) -> B
///     where
///         B: FromIterator<Self::Item>,
///         Self: Sized,
///     Transforms an iterator into a collection.
///     collect() can take anything iterable, and turn it into a relevant collection.
///     Because collect() is so general, it can cause problems with type inference.
///     As such, collect() is one of the few times you’ll see the syntax affectionately known as
///     the ‘turbofish’: ::<>. This helps the inference algorithm understand specifically which
///     collection you’re trying to collect into.
///     See: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
/// c
///     The char type represents a single character.
///     More specifically, since ‘character’ isn’t a well-defined concept in Unicode,
///     char is a ‘Unicode scalar value’.
///     See: https://doc.rust-lang.org/std/primitive.char.html
/// c.to_string()

fn accum(s:&str)->String {
    s.chars().
        enumerate().
        map(|(i, c)| c.to_string().to_uppercase() + &c.to_string().to_lowercase().repeat(i)).
        collect::<Vec<String>>().
        join("-")
}

fn _accum_previous_version(s:&str)->String {
    let mut parts = Vec::new();
    for (i, c) in s.chars().enumerate() {
        let mut p = Vec::new();
        p.push(c.to_string().to_uppercase());
        for _j in 0..i {
            p.push(c.to_string().to_lowercase());
        }
        parts.push(p.join(""));
    }

    parts.join("-")
}

#[test]
fn basic_tests() {
    assert_eq!(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
    assert_eq!(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
    assert_eq!(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
    assert_eq!(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
    assert_eq!(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
}

#[test]
fn extended_tests() {
    assert_eq!(accum("VwhvtHtrxfE"), "V-Ww-Hhh-Vvvv-Ttttt-Hhhhhh-Ttttttt-Rrrrrrrr-Xxxxxxxxx-Ffffffffff-Eeeeeeeeeee");
    assert_eq!(accum("KurgiKmkphY"), "K-Uu-Rrr-Gggg-Iiiii-Kkkkkk-Mmmmmmm-Kkkkkkkk-Ppppppppp-Hhhhhhhhhh-Yyyyyyyyyyy");
    assert_eq!(accum("NctlfBlnmfH"), "N-Cc-Ttt-Llll-Fffff-Bbbbbb-Lllllll-Nnnnnnnn-Mmmmmmmmm-Ffffffffff-Hhhhhhhhhhh");
    assert_eq!(accum("WegunHvbdmV"), "W-Ee-Ggg-Uuuu-Nnnnn-Hhhhhh-Vvvvvvv-Bbbbbbbb-Ddddddddd-Mmmmmmmmmm-Vvvvvvvvvvv");
    assert_eq!(accum("VoywwSpqidE"), "V-Oo-Yyy-Wwww-Wwwww-Ssssss-Ppppppp-Qqqqqqqq-Iiiiiiiii-Dddddddddd-Eeeeeeeeeee");
    assert_eq!(accum("VbaixFpxdcO"), "V-Bb-Aaa-Iiii-Xxxxx-Ffffff-Ppppppp-Xxxxxxxx-Ddddddddd-Cccccccccc-Ooooooooooo");
    assert_eq!(accum("OlyqvYwkuzF"), "O-Ll-Yyy-Qqqq-Vvvvv-Yyyyyy-Wwwwwww-Kkkkkkkk-Uuuuuuuuu-Zzzzzzzzzz-Fffffffffff");
    assert_eq!(accum("JrhfdMtchiH"), "J-Rr-Hhh-Ffff-Ddddd-Mmmmmm-Ttttttt-Cccccccc-Hhhhhhhhh-Iiiiiiiiii-Hhhhhhhhhhh");
    assert_eq!(accum("JiwpcSwslvW"), "J-Ii-Www-Pppp-Ccccc-Ssssss-Wwwwwww-Ssssssss-Lllllllll-Vvvvvvvvvv-Wwwwwwwwwww");
    assert_eq!(accum("EagpiEvmabJ"), "E-Aa-Ggg-Pppp-Iiiii-Eeeeee-Vvvvvvv-Mmmmmmmm-Aaaaaaaaa-Bbbbbbbbbb-Jjjjjjjjjjj");
    assert_eq!(accum("RznlcEmuxxP"), "R-Zz-Nnn-Llll-Ccccc-Eeeeee-Mmmmmmm-Uuuuuuuu-Xxxxxxxxx-Xxxxxxxxxx-Ppppppppppp");
    assert_eq!(accum("OrggaExarzP"), "O-Rr-Ggg-Gggg-Aaaaa-Eeeeee-Xxxxxxx-Aaaaaaaa-Rrrrrrrrr-Zzzzzzzzzz-Ppppppppppp");
    assert_eq!(accum("DriraMtedfB"), "D-Rr-Iii-Rrrr-Aaaaa-Mmmmmm-Ttttttt-Eeeeeeee-Ddddddddd-Ffffffffff-Bbbbbbbbbbb");
    assert_eq!(accum("BjxseRxgtjT"), "B-Jj-Xxx-Ssss-Eeeee-Rrrrrr-Xxxxxxx-Gggggggg-Ttttttttt-Jjjjjjjjjj-Ttttttttttt");
    assert_eq!(accum("EquhxOswchE"), "E-Qq-Uuu-Hhhh-Xxxxx-Oooooo-Sssssss-Wwwwwwww-Ccccccccc-Hhhhhhhhhh-Eeeeeeeeeee");
}

#[test]
fn pseudo_random_tests() {
    struct Rnd{ seed:u64 }
    impl Rnd {
        fn new() -> Rnd {
            Rnd{ seed: Box::into_raw(Box::new(0)) as u64 }
        }
        fn gen_range(&mut self, low:u64, hi:u64)->u32 {
            let value = low + self.seed%(hi-low);
            self.seed = (self.seed as u32) as u64 * 472_882_049_u64 % 492_876_847_u64;
            value as u32
        }
    }
    use std::iter::*;
    fn real_accum(s:&str)->String {
        s.chars()
            .enumerate()
            .map(|(i,c)|{
                let mut word = String::new();
                word.extend(c.to_uppercase());
                for _ in 0..i {
                    word.extend(c.to_lowercase());
                }
                word
            })
            .collect::<Vec<_>>()
            .join("-")
    }

    let mut gen = Rnd::new();
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    for _ in 0..100 {
        let len = gen.gen_range(5,15);
        let s = (0..len).map(|_|letters.chars()
            .nth(gen.gen_range(0,letters.len() as u64) as usize)
            .unwrap()
        )
            .collect::<String>();
        assert_eq!(accum(&s),real_accum(&s));
    }
}
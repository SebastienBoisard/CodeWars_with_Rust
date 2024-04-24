#[allow(dead_code)]

/// # Exes and Ohs
///
/// ## Instructions
///
/// Check to see if a string has the same amount of 'x's and 'o's.
/// The method must return a boolean and be case insensitive.
/// The string can contain any char.
///
/// ## Examples
///
/// "ooxx" --> true
/// "xooxx" --> false
/// "ooxXm" --> true
/// "zpzpzpp" --> true // when no 'x' and 'o' is present should return true
/// "zzoo" --> false
///
/// ## What I learned
///
/// -

fn xo(s: &'static str) -> bool {
    let r: (u32, u32) = s.to_lowercase().chars().fold((0, 0), |accu, c| {
        if c == 'o' {
            (accu.0 + 1, accu.1)
        } else {
            if c == 'x' {
                (accu.0, accu.1 + 1)
            } else {
                accu
            }
        }
    });
    r.0 == r.1
}

fn _xo_v1(s: &str) -> bool {
    let s2 = s.to_lowercase();
    s2.matches("x").count() == s2.matches("o").count()
}

fn _xo_v2(string: &'static str) -> bool {
    string.chars().fold(0, |a, c| match c {
        'x' | 'X' => a + 1,
        'o' | 'O' => a - 1,
        _ => a,
    }) == 0
}

fn _xo_v3(string: &str) -> bool {
    string.chars().filter(|&c| c == 'X' || c == 'x').count()
        == string.chars().filter(|&c| c == 'O' || c == 'o').count()
}

fn _xo_v4(string: &'static str) -> bool {
    string.to_lowercase().matches("x").count() == string.to_lowercase().matches("o").count()
}

#[test]
fn returns_expected() {
    assert_eq!(xo(""), true);
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxo"), false);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);

    /* The algorithm used for these randomly generated test cases
     ** is the same as in the C version.
     */
    assert_eq!(
        xo(" 2CCRFyYGmFDS9PABNyx0kvTU4rbCclaaw6FG9plcB79CRUIlP1k2aBveNZBp08xq17DVr7o11tOxO"),
        true
    );
    assert_eq!(xo("Dp639uDloW04O"), false);
    assert_eq!(
        xo("M28GVA MTIHzPhDizgjgydlHRnoElH iUnXk0aiWHCCg9vckA"),
        true
    );
    assert_eq!(
        xo("YFCVxfsQGEirISQj5nmw1yXAxL9GqUhYs9lEaQ5B301 PlGRFtETiftQ6ey"),
        false
    );
    assert_eq!(xo("UlmHgHaniepqeqnSjwt5yTmfHRSARKTe83L"), true);
    assert_eq!(
        xo("Y1IS4LP4H7E79UNwqdgHFMhoRLoWOMSyGoGY1hmRd3 obhpbQvwysksG8lyYiKSX1hkFTHpG"),
        false
    );
    assert_eq!(
        xo("UC3X5gpghdmRnrWHo9C1ITFNVlkQm578GVKOqMNgwsLbgowcyVsDjBmeyHQ"),
        false
    );
    assert_eq!(
        xo("Pc hoyBo5Ox5Lwk8pzjgyQ1cghvk5CXWioHhdEctqF9LGjDNO4wSl27RujX90WQuOizCIDMIkNg7e"),
        false
    );
    assert_eq!(xo("LOKZsqPyWGzdDzDemdpK8GxXsOY EIgsi66k4E"), true);
    assert_eq!(
        xo("vuHYX8pXvc3wjUzwZ3UicucTfgnWo2MR9EqH onXEjVXNrZU7j"),
        false
    );
    assert_eq!(
        xo("hx8LYsu9k7CZ3gXYS15bzsiw8oLWe0tjk09BoyX wTp2X2bx"),
        false
    );
    assert_eq!(
        xo("kG0 X9b4KM6F2s61oK0fQmd7YrFnn0rGFetb0QjHhplucs6Q35Fkk"),
        true
    );
    assert_eq!(
        xo("gBM1pfBf1Wf3Mdlr1UdoATxY7anIWGxFQpRPxlBpL0wTszYqNLond 3oAkGw50Dcii5QbG"),
        true
    );
    assert_eq!(
        xo("rFIYN2eNFZ5q4c3w5pF6eSIcZDho06KqUhUctUpHgtHibOikHCaHbHetzkfja3kL"),
        false
    );
    assert_eq!(xo("VriksNYhWAS0L2FWwxBGx39SrfX4WW9dRrEG283lUWkfz4GldQ8Yk4uxt2fYkpHCdaneyjxSLO10QEhD3dpptgQoySDvCzu7QR9TCjyQB4m"), false);
    assert_eq!(xo("bFMhM0b4fN0jQPPd3HozptN5l3zuCL"), false);
    assert_eq!(xo("uboxayKwjTqY7OtiQZfX3jUVIbrSoV4hiwOktg"), false);
    assert_eq!(
        xo("xZgNFB3ZgA78EdWfZMhc6wgaufKck6AODMbhDg9MB5XkyaWV0Ki6Hx"),
        false
    );
    assert_eq!(xo("XTZyCSFxfWLWr5 3KzA7vCmoZtfSZTDEwvUV0ptdAVDFFwTvxulFMGVIXqaQEN5RomgyOzlar4ML5xehnx2f9FVLS5pyHwIMQV5qraoir"), false);
    assert_eq!(
        xo("B9hMsDie2yFDCjT3xd33wzS6kHumCicIrPbggWhyI2Hkosa belvz5lPIv8zEl2T4dKyG7osNNW58SxzSPG6W"),
        true
    );
    assert_eq!(
        xo("EwkQlZKogIx6A81ZoSUAREsVVVvQKVbpBSItchdUVRQb kiI2SicykiMrQkLfS20gmr"),
        false
    );
    assert_eq!(xo("DfrmKRViZ2FPRZ4U99bwmSXAdOPn0A7SNq0EWZOdAfVr5FP4S63qTPAKastiVqKP9HrQmPA9BcwH0POPEmP8qsFavHhuG7"), false);
    assert_eq!(
        xo("DSKe0lA9r88K5TgR0Ft7FhIVAhwG58cTUwYYkiWle q86gQ31YNRCbjcPhqWdx8ZzQjIUHNOuNYqw932"),
        true
    );
    assert_eq!(xo("KxseIRPNTeqB8Ltm7S5S EKf9Kd5OwvgcUg3eN"), true);
    assert_eq!(
        xo("TNFjVxxpj5r2lRITWhGvVKL0qgf3gqYKNs8PEhKXpXAQhi6KvPeukLyj"),
        false
    );
    assert_eq!(xo("kuokDMyVVFh0c8otIIkGE70w9tkKpUAnDXGgOZqxcgkFs LL78bb5cn1mGTYai4N2IenhwHPh9h v6ol4RDCBwOdGyHMdODwuFmc"), false);
    assert_eq!(xo("t53eijvqbS22hSSgdN4hWZQqxWu7MmO2kQ4nHfbrMg3tMRz0jdgL4GLgyVa3aLXQiANBMs1FmLZgdqfWdmpttMliS qXuXsgz8wou1B"), false);
    assert_eq!(xo("IRLZR5pFzjLvu9EHbQnE"), true);
    assert_eq!(
        xo("OGTTKAAUnyr3VXxH388oGkdSC18dgnvVtMkC 1jM7kGLq9v36z9IApax8yRdR"),
        false
    );
    assert_eq!(
        xo("7IGsktwT48VYDXR0PbkO71HtaDaFBSEZao5md55rMFIEmqW0nKYlUdvKNRxAuBzg4sAuyTi6aG"),
        true
    );
    assert_eq!(
        xo("UFVYeRrlwfSKwi0AsV3nGr0ZiEMqDRvouoFuPGnP4wk3ujQhqzAGbAbR"),
        false
    );
    assert_eq!(xo("dWDFNCC5m2Tyjy7hxVcIgPITds1CYlBLX bIsuQI87As0T klCMRxc0RRNhwuqK2 tbOoNitXKhKdF9RbC7Kt5pO"), false);
    assert_eq!(xo("nINvhDXlwz MmRIuLflvaCZ6GQ9OIUh8rtklGeIHz8RL"), true);
    assert_eq!(
        xo("qVisrEcVfhQSfwaLgyClcw7bIPHTtb5eiegaEO7FNvjwcmKOWW1BWVoqvBTmobQhmGn4flT0FV00pHU"),
        false
    );
    assert_eq!(
        xo("pbcOpEOEge3vmxSM89Hojso46y1eCbP3X6Q2vqcAef8WTUhQT4OXtGD9XQ9"),
        false
    );
    assert_eq!(
        xo("MInUENm1NLQM4vhm5W1bcbibrLzCalh01SRx 2odOU0E4g1CHFp0"),
        false
    );
    assert_eq!(
        xo("jbq0HVp9wuBDPywWfg0Lf O gct3Rt3Lm3GySNeS3BhQ3k2rE1CHhxtEDpvif"),
        true
    );
    assert_eq!(
        xo("NSeEGGjGkoWRUxZncaftMWb46qehuhm5MWbTyDaxXDVk6RHkF4cEzLbdKm BDMKWhe6lHuS KbKS2c2Tdf8T"),
        false
    );
    assert_eq!(
        xo("xHPKucx6eCpyHQZVeiQmNtgo6KxuFNq7RIFP59E5KS8neHotCPQIFGHUx9F0VS6goUNqRj4s31O5T"),
        true
    );
    assert_eq!(xo("7jjbN5qWmcQwlsAf LZdLfWAnz 7s4Fup3E7XWnpBBpy0XjmW0OsdqXOzoTWwiIp0qyZWQ1fh0ldXYOdRocHotfmdL3g"), false);
    assert_eq!(xo("ncNn TNMvweckgyNBjl5O7eAaWN1lqH"), false);
    assert_eq!(xo("cWrZ9MYl2ODuCK5O8aERyEuf43rs3K1sa23uHnvT80"), false);
    assert_eq!(xo("ZHRjAGKtLaa6eq7f3FwwV341UkdefpF71pamVMQR8Om0OzfQ8g ZyzUSAZFV4kLK 1tdGYFSywSDxBelwE0tVTHcgciyPBXqAMajzV2v"), true);
    assert_eq!(
        xo("E0igebfTclh19OekFu2W1MrD0l5ZpkYPgSNj8wZlz8ujYiTasbP0aSAo5T2wXnO0qbf6v8gF3l3nbbY"),
        false
    );
    assert_eq!(xo("LiY4ZiwAS3PbgpxgRUaNWv xSlL9FfRvxqfwygPHyorzkcIZ3HTzq7DPdxI1UwfM0 1yDnCLYVwCiUW8s8js43Xudj5KkiYhYx"), false);
    assert_eq!(xo("iLrMdMN85vmMUxtKg7"), false);
    assert_eq!(xo("luGdB6lzaVAW4kTHElRnaNC0xyr0k8vOrwX15qieVHRD88znhCXbIEb1OnMNxsvbaudruky17PE4UXjCEqo0REMw1SBaCpT7f"), false);
    assert_eq!(
        xo("0k6ti2SILoj94L80KmQU kpfm6c5MIIV1EN6xQWeUL0n 7bocNDZrX5WGhbWV4gPvrTHW"),
        true
    );
    assert_eq!(xo("H8FGctAHa5TBAYIY0sZxFEf1r"), false);
    assert_eq!(xo("2UfkW4guOOpojzwZyShDogV1Y zZhclu3EnlbSm5aYqnMwi5FxTXOlPyH3B6fyqrUvc57LNugVRtNO7W5 dQHCRvDkjNcj23PdVY4IDix"), false);
    assert_eq!(xo("poDtIURC9aYWRiX"), true);
    assert_eq!(
        xo("6r9V6I7KyBfH6TCtFvoa4P6MXmjhY90mw7p7RtAU2NfVz8M6z8usI0qjmjT4dEkfb "),
        true
    );
    assert_eq!(
        xo("2RXbvIPu1T6CM229bM9DgfiymIvD6BXXhTiOwHHMsOA9  7Ch4bQtRTEq2t6S"),
        false
    );
    assert_eq!(xo("G3jCmhQyyPDFUweEBjxoGnSa1Sq3TSDoW443pl6d4I1"), false);
    assert_eq!(xo("I12zuaQVYw5xA5fuU1g8LuM2nZwhh2DB"), false);
    assert_eq!(xo("TuVDTYpY RizGfNE CsROq1jwuT"), false);
    assert_eq!(xo("KkYacjB2X8boQEQtHhUtwKIIKbNKEkTaIwc5huZ0rtoWUORdBegVZCxnourAbOZ7PeNNpTBaO4AyEyhgiEqNglsvng6CBDGv"), false);
    assert_eq!(xo("QaV766RMRUjOVRGhRQRVSNa2X4SbO"), false);
    assert_eq!(xo("q9uC6eFb2o5iKHAv4yVbMna8h511kbNhv69h8SSVXR4WaWVwTImx6OkoQy9HEB6BH5eDRSmhujGgLQhf9ST4pX0PM9jjCarQereYGmZCDn"), true);
    assert_eq!(
        xo("cNfYXMwA0CPUIIVMk51L75L UyazLIjwxEUtkYGghjCNNUAuQQVczEV"),
        false
    );
    assert_eq!(xo("I5ah njC1VMilEcIZtxTS6OwvvbHBiUzwGIQwMd"), true);
    assert_eq!(xo("Oq72EhbaxKG5Ph68OlAvbuGnZdNX0qPI9"), true);
    assert_eq!(xo("mmVCUt s9w 478aUkUA55xuLerHSAGp0e4XP8aZ4siVit5Sz sfxTleYcLo6ck5voF5yVD97XZwVAFywMjczYc"), false);
    assert_eq!(xo("3e8BcfGGzzObr"), false);
    assert_eq!(
        xo("z05ewFSDZD 98KRjYvjWiSDVk7rMzy0At3ArYfFgMrdVXFqh0kGu2l2od36R86yqCH"),
        true
    );
    assert_eq!(
        xo("h5HQKZAEQjQHhLczOAaHNwAj9Rc9f09E2F5IcY0VXQQ0pVlKnQbaW7  mZbSHcrI8"),
        true
    );
    assert_eq!(xo("uj uqaTQpSdQzI1FHlOtZ9BtPEmF0xIuP9aIoD9khaHUHUsvML YHfPhF82FhmLBb1vEExLLWnho4mRKy3sVrX0KK1z6f0LP"), true);
    assert_eq!(xo("ewlvZ0Vid50AvFGdM"), true);
    assert_eq!(xo("hOn6hggoQg10w4qxfZk9NE3CrfmLmfiDVTI0ZXio77y9BrjqXU9YqMlrqGCb31qDcYPtkLasFfpQjbm95vT4pC7p HY"), true);
    assert_eq!(xo("YT51wR5ldrRNjAfT0AE287XtKGfozKmBdzDdBYW9U5XCY7jEULlWSrYxHK 8nBXOuvZ23sBbuIoPRmw0yzmKx609Pb67aFTgq"), false);
    assert_eq!(
        xo("u1n2fWmcq2L0NBlfEAo UHNQD9yofrck4KotiYmicuuo2vcx6WZEoU5CT83Hqs643t60Ilt2na0jRcI Ck"),
        false
    );
    assert_eq!(xo("4NZGlHT9HH3UY7WEWXUkiGkbpP4Ucsvd8yMfCxamqe7R7v8niWGgi5OhWWXCAi9OaBV7o5cu7jO3SYgK054tVWk"), false);
    assert_eq!(xo("4q9Ry7whLVs1HT5DCEEc9Lr6P58vchOgfIHlWLUaWK9od2Z6lfgj4kpEaiBTxcZU2mMzyxRUw FskcuPYcNmP93XyFhb075NxKi"), false);
    assert_eq!(xo("0O6wqOdh5kBmSSNaPG4Dzu9jr1tbTpXRZDTKdH5y8LRUxjK36IEdGm LhRrXSCgzmL9On3ScPt13cULgtpmZIMmBl T5s"), true);
    assert_eq!(xo("fI soNwLiSB8WoWpvYlCh2OFS2toZg17Xjy1euOpHvuiRIjNrIIlbi6978tGs5O5oFXaLzkbGcRi2L75jWfSOuPTrFafYKiF"), false);
    assert_eq!(xo("2VwswvNvN7f"), true);
    assert_eq!(
        xo("1oClzbMm6lWO DriXG PmmZ VUVIQ0ykyNUnsvCtSdAeMFiIzUoL82oKn3h1Ml"),
        false
    );
    assert_eq!(xo("6uAkh8rzIh3Pe"), true);
    assert_eq!(
        xo("LPBfHEoUAB5QrLkzCdnvQDlR61KBnLltjyl8UnWodvIfeMYN05o Y 4QjIdDgWKTY8Wrv5pIhKChRh"),
        false
    );
    assert_eq!(
        xo("Eod3FPZr2kfWYHp0dyQfQzv8YeHVY4Bqw81GIOLbN5MlP7kXwH6 qkb"),
        false
    );
    assert_eq!(
        xo("zwj16tN7wFSOsl70dClXHUWKgbodGEH7T8X wYxqo5PzfRzbgOu5uIAl7KROssEFodei"),
        false
    );
    assert_eq!(xo("vi2kGoWomBv94jr McvT14Fwvl"), false);
    assert_eq!(
        xo("IldhFkt48dzySimI9DAn 8X6CAOX9M2RiwKXlCDh78lIeKtqxHm"),
        false
    );
    assert_eq!(xo("AiFvZdtGw6ii7MMgI WRcXqb ynXCCEx1oFUxLdzAlnUyfZDFpo83oclfKq7 WjjI3N3P9psjYHcZn0cUyzbfSDmz"), false);
    assert_eq!(xo("zZtP0SWCRwXAdqi4xMkzoVvQaOwB7xQjwEN0yjXUkWpWEgcCrPtdv AY8Dx56Zo6f73laSN r83RpfhfCkbiG9"), false);
    assert_eq!(xo("SO9REGSn3IjpZ1"), false);
    assert_eq!(xo("p430bwaPd62Oe"), false);
    assert_eq!(
        xo("YHprSAYjd3uXufQ6aINqUaqGLdPPXDugkfk3BVo0FgKZWpFPG AsIO1kCQ2T"),
        true
    );
    assert_eq!(xo("6Kzp0s7Zk4cYPt7KLYbpyYGFx7nCC2BKOyNIgLhYAPesG74bWgRYLaoetYN1nOU8SzE2wLbSlbC6zFkZ1CfI4vO93bNg5zsTSSvDlKfAe9"), false);
    assert_eq!(
        xo("zACeQUj9nPUvW3oKXQ1hwEOFCgMxjNMOmIPNyNVRnkNNi8T3vxynCDqY2NMuSSD0afBg"),
        true
    );
    assert_eq!(xo("IHbMRd8mGBX01uXEthz3emRSeEVjh"), false);
    assert_eq!(xo("hBFAPiBTRWjg iloPbb4fbjL0zpVUvLXpbcL"), true);
    assert_eq!(xo("us7OWp8OQyiVQwqNkHNIBIrajNkAp8rgp9BAiLaYBhzdjk"), false);
    assert_eq!(
        xo("Y6BeLcu1O5oswydhdaqBkH6VWsrFmxD4h 76ZeVz53W46FioMy4tVM1gq41msRzn5flFft6"),
        false
    );
    assert_eq!(xo("5c9B0r6VpMM4FGySU6wHmlrlTc7LmMBka"), true);
    assert_eq!(
        xo("tiC953BZCmvNw2XqIikAmX5zQzodIj7SnHPjIc0xXzoZs1LK90jbKa"),
        false
    );
    assert_eq!(
        xo("oQBc0thO 6oHREDVd736yZAXKI9ejuKnTEZWT8e63oOvAdoBuwKaLl3mMMmy6iQmIp2"),
        false
    );
    assert_eq!(
        xo("u2790TcAGQvC r4oxVTpA6dRqD4IVZO1PTMIqoiDUEn9UjuMeS9AUayTC8w"),
        false
    );
    assert_eq!(xo("HPngysVrk4HM"), true);
    assert_eq!(
        xo("eB3l4GlED49C8EY5RePtmNiPOo3WNGBMWDizlnB4kZSgfpoKpHXIEMie98WXSgOeMGcuru0x1PUwZDH3l mCj"),
        false
    );

    assert_eq!(xo("CdEs7rmYyfs zdTjRz1w UGqn8mBHDzmM PCB9h3pnjWPZLye tvuKcWS hlcYGajg4wdi4cT Ct2Ei5qpgy2UUEkB Skm YDSUBZmug"), true);
    assert_eq!(
        xo("MjvzcgkTQvv93MgDUN56  Is7 3cnt1gcdTHmVs1iTV8cqP1MRCv4"),
        true
    );
    assert_eq!(xo("TMSb 9Q9d  GQU PcTu Wmueh6Rw"), true);
    assert_eq!(xo("bz a4il2UgekbGH3yhdPz eLne6 U5 3slrtmz55I5d6 1CDr8fvhgtFj97C3TFtBGus9qyY3hm tsri gRn YqKW7W"), true);
    assert_eq!(xo("mubsCEvLWmg DSLGBCV"), true);
    assert_eq!(xo("9ghnZhyssYR P2  cueCEgru1VZAUREELL3m76 yQrFq4n dW 87IqqbV 7lgmZ5uAleU2y73 5Cug8 Y3Ey4E Dsv4"), true);
    assert_eq!(xo(" Dj1gI1Zb6FhDI0 C5zkBV FfLkbrpLBV7A9WmHM lr71qH 582GZpz53 h6L9HR5k bFua9ns3fsIVtTAZdZryAS R"), true);
    assert_eq!(xo("jaVpmdRrkS0GN310nDw3dCt4NiRnW7LHuWBEFkR4"), true);
    assert_eq!(
        xo("R H bDLqGPs Dtd UPaEjMILS0 3Rg 9C j w9iHUjtalj3MNwkt1B  Vy"),
        true
    );
    assert_eq!(xo("jkedq W Wmfu5Est m5Ug6NjZ wa tWPcECc4sQzID"), true);
    assert_eq!(
        xo("WtUYN7mv l meL deWSPByzu8TTgr SYspq0tbzdHW L 83 mRLdPqjUE ZY1vqwE00UvlLqLkL1fP9SrGBD"),
        true
    );
    assert_eq!(xo("r  B2SSsADGISa5614D  Wc n6srLNH 5VTZmi49CIYcH"), true);
    assert_eq!(xo("jUk3D2l70H2 LI5nlNsBTI6nRF5q0AvguFtZE G qRYLe yldguK 0ugA9dI IVGaNvA 00pRTMR63Bit2BNN2emMd"), true);
    assert_eq!(
        xo("mCLe4YbTGvRVNq z6 twR6NzZGz RRr2hC3 jhD6AuG udZ0SC9iHVnmmGT5pYZnlAT"),
        true
    );
    assert_eq!(
        xo("1uQ dt1jjUg4ld978vzvwVjpl9UI qunyTNk SY1i Km5Gz2wSgq"),
        true
    );
    assert_eq!(xo("WpH Gh9Dz LM qvLRdMd4duIT7Ui YEP"), true);
    assert_eq!(
        xo("p0lv67FhFEcSMrtq3j2ecnUh4Sbmy6rd5z Mffe1FbWQ TQjCkltGBz"),
        true
    );
    assert_eq!(xo("mF7RP0KRjglRml rH Rth5z0s wNbcvV8lhUh5qR j9UIjIdqekj5Ifrig2RUdhTWWeHveaWlu8MvTwvTjP NBQ7ekn QA"), true);
    assert_eq!(xo("2ryUbiGMnCVsihdRCQtph1Pp1MImjiW sUDISzv9tupav FiCHg9RV  SH 1Ca97GPEh4Bg64cyfDKlnvYb1zEs4"), true);
    assert_eq!(xo("4Z3rHMHjj6i"), true);
    assert_eq!(xo("pq5Et uC Ey8N7gkyin91j0YyG zP2vL1hp5bhDBHZLRD7CLrpF3 DnEjiBN7iYVvGHK4lE wstSuM i890bAv1"), true);
    assert_eq!(
        xo("EADmKaarBNZFuddycjKQrwUq8YcY3aczC0KWi2R3GfUVbZML8U 9ID0RplqqWvkgz9F M lEz"),
        true
    );
    assert_eq!(
        xo("hHRtbQ E uubPfGPs 5E fQ9Dhi5LnG6ya8 AU40D0vsaeCv FMW2gnEETcp5qe"),
        true
    );
    assert_eq!(xo("aqbVTgCusukcuqKgFRFRZe8aYj njvzQ HsB354rq ntptjCI5hk6a mnLBGeutb haSieGuawakyQwSzg2dbkA"), true);
    assert_eq!(xo("0n eWWbaGUVBUm2HQw8R 9u f97PfIz"), true);
    assert_eq!(xo("5 hcaST1ijDDGWztEg 5fDSn Y7Behq88fj BC73wUTG0NZWrj g57PEEYjEZg GQFCdnKymgPefbVDne7P51avkFPCcMpqgNZQd R SeSB"), true);
    assert_eq!(xo("vmBvcbksTDEpfgTqbEh"), true);
    assert_eq!(
        xo("fFcAu6g TwI2 zfZGR6aynlkPCk8gRiQuWrdchcNg4GDa2vMzk9mDgHf06pf jDYHFZq4tNi6RLyc"),
        true
    );
    assert_eq!(xo("4S Mw9rGpeAlfwI1TfR"), true);
    assert_eq!(xo("fcjwQsEphVgum 6nT7tF9D8sSEqiHW 2NP4h  v8i wY 55A17hV a6pF6kln  aiwbMzZV lC7nGM0EZfhzbs 0tKj gnPvb6sH"), true);
    assert_eq!(
        xo("7i6KwlcNkNIWtMNfhBh6rHpmgTVaGQBAILjUqAvj 3LWKYG7l5MzI0ZBWE8i5LH  89mYlG7acn23wC 7AQWI"),
        true
    );
    assert_eq!(xo("hqSauY3V8kK4Hgsz cFUaRI4SWRt6qd wNkyziMjnTieZlcMAqvCR4Ficipgiccu SsQZ0GuRV stWqGWcn5gewUi  hRyzPKEwn"), true);
    assert_eq!(xo("FW5  yWwyyQ EcSEcaAE5160iEFPRqeuQ7Yq070rw QRZt9tdUYucmrW4GrcDrWt97e84w95Q bpRzfH1K syCA9nFkwHKuFF  U I"), true);
    assert_eq!(xo(" 9 L r5t2 NqSUSP6j175UqIpiuHhStVPNE"), true);
    assert_eq!(
        xo("1ceK5NLUzlygI4 QQp0PjrhTGkhfrLs4rs 98hEjUYRr DRM Wb9TPQp2Hv4mYVWGz"),
        true
    );
    assert_eq!(
        xo("DIM4rvvi 8  Gw5DTTtFvLqhiRqIak7BN F2u97N4YbIUhpe fHy5GlwplUzIWtv"),
        true
    );
    assert_eq!(xo("KsjG tUFmypFfam NRlEV Ar23 z66ESuZhgBV1d1q2r gQ7L  We0E22ep 8SvevPgpvy arsfsmDyybETbW0glwbe8qb0v8fEzjB"), true);
    assert_eq!(xo("k7G zjYnA059 2fL3PLMIyV3e9v dpVRum3mHn"), true);
    assert_eq!(xo("kvFj TQWb0 sIg6ng4 Yu3lI7H zvBrApKM4djzIgmK DS 1R4vPZD GzI7GcqgWKtz3lPnYqyjAGmQpjVK8mMSLvYcGY udAmt5"), true);
    assert_eq!(
        xo("  9EG  t0jlC8haCM1ics 1L1e gtggRSa6kZrzLk EhVA9Amy6PMZkHC 5BBu"),
        true
    );
    assert_eq!(xo("bgaD8s5FERyHjGF1tKZDRW5 fEq9gtf lbdjIi30 8DBSkDG7vlQNd 0GP82Fa 8TDseLwwh7GeU5FdMqRjBHBVzbL q1I6"), true);
    assert_eq!(xo("ptnMymtweiI6CL Zd9Y1Ckip8hiBVep6uUCs6Ba65K"), true);
    assert_eq!(xo("DmIcSY2 luImrCY2UI9QSYdCm06l3SiEqGNPaFq  YMVlz"), true);
    assert_eq!(
        xo("B2 gQr d niZgL1KLERlD jr1B1Syps39CymNE h2G9rbVZrhfz2c 3pvK mN6jBH"),
        true
    );
    assert_eq!(
        xo("aENLLFbFK n CADtW7Dzu9q1cNZA  5L4K  cBfqTAmjaqlD865w25MfHIVj7K7MnvnKi KS u1zy"),
        true
    );
    assert_eq!(xo("5ewNw4HNt  1"), true);
    assert_eq!(xo("kTRt21QY69gUilbncTjbUevnL WeEjALVrVA3s Ba chPHA7d2kK4Sr  TAsZa4EgFyuBs48Tdar4C0rShLE2Ce "), true);
    assert_eq!(xo("SC8 Sm7gRqeG50 tas64p7RnF8fb9VAITUl 7hvnhnwivpaRlybQe2BFMqeCCAzju vqVZneybiQYh6Bckdc q069wHZA89KU LzwtziWmI7"), true);
    assert_eq!(xo("WZ8K5lM5lZ1kbI3cYI11gi3lQ8L1EGW4v PjL  9i0gRUuBwSAM1ZGiaSM9uR 0MNPyT9dEysKUHVzGn5RfDa C "), true);
    assert_eq!(xo("Z1S NQ5vRm H2"), true);
    assert_eq!(
        xo("MI3Els GtA3KCdETza 6j  sWMPVEQik90LgpM4f C3s6zFhR7 LeE4"),
        true
    );
    assert_eq!(xo("jWgK2c7MzGe R6A5GErICGCfP56ScMi kc"), true);
    assert_eq!(xo(" Gf7719  26dYvfyi11  Ljrwr P"), true);
    assert_eq!(
        xo("Q2ntelq3j dufvSP GGmR9Nad2dh0VTnD4EGdQU VKY8riTPVs9giVmzAAYIbw705bStMaT "),
        true
    );
    assert_eq!(xo("wY2 em4sbiliYALyigQus qIeycaI5q mMHQgUPPF1KpmndRSDPCrIltDDgpIdK8wbvg55NY TDp41DREshTZU csCRK9sS8Y7s 186IE"), true);
    assert_eq!(xo("G vjN1Qeza 3TLDqm pwTH6FtCS0krY7c1d FDa8NZBkWmhM9mMEr uiYLyBC Ltm5dtIvw tM RgMzad Sh4L bnmai91snCn kLmAV"), true);
    assert_eq!(
        xo(" jn8I iHpvU2K5ftr r4EmF2M6IdR mD BIyvHCEqzlKfTVgMB zdV 0HHs 0PZYvlAL7 a7AWSB9n"),
        true
    );
    assert_eq!(xo("Cppez5plIBQHVqnjvFMePadYCD IARP AKb5AYa"), true);
    assert_eq!(
        xo("zSVtPal5N 5nly3wL0PzQ ArM98mW Y LlNj tagdePRiQYM tuMdL30HNv"),
        true
    );
    assert_eq!(
        xo("542PpD1PK9V5qcvuV16a3e0eL1evFnEIhlub678D3KT1q27VH"),
        true
    );
    assert_eq!(xo("5I33AILD5 0v1hF7SP1UsLdHIfUE8fz1VsLbKn9AL68YaSW alhc9WwRDY8I jnRmuMuWCH HDFMfu bvBLlhKC1gLn78Bn 2sQl7i PLym5"), true);
    assert_eq!(
        xo("8kR 9V0DUB2gsBs7pEV 5tj0R2HUgPKsMs6C65mEKgG2qNBpkURaALpt BAVcl8pQ45bCkNEY"),
        true
    );
    assert_eq!(xo("lYkrGIAiLd11jbA1p8 iuPq96CjZd dRjQY48iwFk2WYdrvv9RQteQw8akFbw1Mhhc s iML7kgplPEke5zyz qE GNIESBZ653vw"), true);
    assert_eq!(
        xo("t97jyg1IE3 C6Zr7aD 0pe 8ZRMA11j3Vd Mm7c1BCALWGSDjsPmdp 6ApT 9ClL7mIcc2s ACMrnmFFAdw"),
        true
    );
    assert_eq!(
        xo("cvH6Epcm6D 2qFa Ez tth WGVug8u0l Ds4BYkH bT wGWR94UruGECmI8vF5DtI"),
        true
    );
    assert_eq!(xo("6zzd0Q R Iiqwy4Iu0sPZI F6V YNylZnA5ceQMS"), true);
    assert_eq!(
        xo("5S80EEt FFidVVyYz ln30VE4iYz5ukNHgCiGMUzQGEsmUKLu8t981 167pBqn HPlBh"),
        true
    );
    assert_eq!(xo("lYU6qZ TnqAvtIz39iC3hY3DyIdnzQRA9wU7LMmi19E3Y ZW2RluwaLtH5 0z5 h5upS6IKZUaPw3zWLB NV6RM vh"), true);
    assert_eq!(
        xo("Hau4earFrAvC Rc cFPrm Maf 0CFgllnvdnYhrU1bl8MDyakGfK8r2pcN50e"),
        true
    );
    assert_eq!(xo("lmYRBeKNUUs1WwEUsCab u hhC"), true);
    assert_eq!(xo("iLrkqWBWjBpt Gg ihSDncCCZMs9t50pAe  TseZ7pC"), true);
    assert_eq!(xo("V0b5d0T  g wtR0uv7j80FUW1Up F30"), true);
    assert_eq!(xo("1bze VADq5FtnFsLImHTrs62HtS 6 ZC7qCkVc5BhIMAa9zKb8dm  jVa1ir NN MP A4Qv qaK  9BY7q6eupMgq"), true);
    assert_eq!(xo("lnLuLUFliUZQDKAlN kAgf pfGlq5j5dFAjc 4Af lGe9j V9 cd 6z9IP9 12PEW2VILbVuDm ADQcPqGVCIuKuHDTRRy655KESefKH"), true);
    assert_eq!(
        xo("MkfgABfjI7NFD3ljb ZQHDcbu7 kDt2 crD5N1avV79bMgAa0 AG76qeM lWRp1NaEL0k2crBlnPYq"),
        true
    );
    assert_eq!(xo("T   ghpktUUReFc"), true);
    assert_eq!(
        xo("MVRNtcpyimRG7R mVCIwqKjrgCUgm1egDUeG1a 6 dATH06Q Q Z FV mk N AwZurn6fuNCzmg8miniIV"),
        true
    );
    assert_eq!(xo("HnPwB1IP7iAGdSwiNi4 nzCvH4LCQEPu2 yG9Df3 VU7nYwdS6ZqdT 1nrAMYp4bl9qgAsi 8SgvijIZplZzBtppyL9 117 N6 nuQHt"), true);
    assert_eq!(
        xo("5 p 73kd I0AG Fnyfd4  KrSrpeZftSAs Iwq IILagNVP2qV  t sHElLsR Cr4TA5E EeqAjVbN"),
        true
    );
    assert_eq!(xo("cPvESVSlg40bbK  AAqEyl7ijTce2pFdKwFzCg 0iY WgW4Be2jiisdbHTmUSeLzrvPVPpVHHt 3TwGU16S8uN CkLl4 qaC  U42k U"), true);
    assert_eq!(xo("Km2KevU79Te BCwEe5fee7cVWgDZBprRY3p gKTaaT56k4mz"), true);
    assert_eq!(xo("PzBTS 81sFAlLtNPfcB318q 1"), true);
    assert_eq!(xo(" Kc3E4pjnVNcP8qdhHjrgQmw f8 TCjtW 56EtaM"), true);
    assert_eq!(
        xo("9pv4pI BVpwgYwI 91hKy pFdrygs4eiemqs6 7Hk85N9 Y5jZAzwZ9gVw"),
        true
    );
    assert_eq!(xo("85Et71eyCawQm8zs6KG17mmhn sS3frByYMev2yy0yTb"), true);
    assert_eq!(xo("VY8RdC  sG8k2Iae"), true);
    assert_eq!(xo("6iyCimPfWPj86lLQfdA0Pa fumtSUtsEdgdkD"), true);
    assert_eq!(xo("Rr  whqKGg4jsEmzHEHZydCgD8 Mhr Ku5 yi V SDwBtnuZ2C 58s PiIb  70DNmK5NQF rGgirlN3rgV9n3IT 4B81V FajI9GfnL"), true);
    assert_eq!(
        xo("SN34S 7MU4smmFNzQZpflCcYlzdTL3zlHssk5utQ0hNv4SGlvV"),
        true
    );
    assert_eq!(
        xo("SIMp1 zzNsPi1 jyF pykn4FhRzUNtq SMU 91KVqk21w 6GWeL  Pv"),
        true
    );
    assert_eq!(xo("tDa1Kajz9NK5SnFSFY DS"), true);
    assert_eq!(
        xo("zeb0 C unA YQQGHZDR UtBsKmGyekmz  ztY3s3W4m4 t rGwWdtfcZSwNSlf7P9hDRyV UG"),
        true
    );
    assert_eq!(xo("m0Z7wn7QIqF3Zg4U1pZVLK0cpyiEP"), true);
    assert_eq!(xo("p8Z35 h1sCNBDUwYe1FCPvWPG DVwfYasher9h2iwG6Fa5paLB7ziv4 5YY6Kqy3fy4sc zvmd9UtiLB3St878K1QsC s10m71IhAM"), true);
    assert_eq!(xo("9wUYtDgME14QVNuDflvzQfj"), true);
    assert_eq!(
        xo("iibvLd4Hdnaj0vrEd738L9I6g5dHyERdPMYjeb5gVyWZQEdSmeUywy9f8a2wDvfssLQsrv"),
        true
    );
    assert_eq!(xo("MQMU3lZWGV0LCeH7qMN"), true);
    assert_eq!(xo("  aSz3rE5m1KNFgG59iILijQi6byLpt kh  yNe0s3KqR6Gm2 8UMs98t eVQi6Z DlmQlmA tAD0gqM78vVuzZsAZ6eN1C RwI4z6WW0wy7h"), true);
    assert_eq!(
        xo("Zdqt7tbGf7np9f8LzmK2zw ZEKCYS9 c UmtA KBHu7Dk4zIz"),
        true
    );
    assert_eq!(xo(" Pw zW1e7WwECpqpgP46bI0kpga BQ s0 kk1izCU gkWf F"), true);
}

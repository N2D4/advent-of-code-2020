use crate::util;
use regex::Regex;

fn swr(s: &str, r: &Regex) -> i64 {
    r.captures_iter(s)
        .next()
        .map_or(-1, |t| t[1].parse().expect("invalid integer!"))
}

fn solve(input: &str) -> usize {
    lazy_static! {
        static ref BYR_REGEX: Regex =
            Regex::new(r"byr:(\d+)(\s|$)").expect("regex doesn't compile!");
        static ref IYR_REGEX: Regex =
            Regex::new(r"iyr:(\d+)(\s|$)").expect("regex doesn't compile!");
        static ref EYR_REGEX: Regex =
            Regex::new(r"eyr:(\d+)(\s|$)").expect("regex doesn't compile!");
        static ref HGT_CM_REGEX: Regex =
            Regex::new(r"hgt:(\d+)cm(\s|$)").expect("regex doesn't compile!");
        static ref HGT_IN_REGEX: Regex =
            Regex::new(r"hgt:(\d+)in(\s|$)").expect("regex doesn't compile!");
        static ref HCL_REGEX: Regex =
            Regex::new(r"hcl:#([0-9a-f]{6})(\s|$)").expect("regex doesn't compile!");
        static ref PID_REGEX: Regex =
            Regex::new(r"pid:([0-9]{9})(\s|$)").expect("regex doesn't compile!");
        static ref ECL_REGEX: Regex =
            Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)(\s|$)").expect("regex doesn't compile!");
    }

    input
        .split("\n\n")
        .filter(|&s| {
            let byr = swr(s, &BYR_REGEX);
            let iyr = swr(s, &IYR_REGEX);
            let eyr = swr(s, &EYR_REGEX);
            let hgt_cm = swr(s, &HGT_CM_REGEX);
            let hgt_in = swr(s, &HGT_IN_REGEX);
            (byr >= 1920 && byr <= 2002)
                && (iyr >= 2010 && iyr <= 2020)
                && (eyr >= 2020 && eyr <= 2030)
                && ((hgt_cm >= 150 && hgt_cm <= 193) || (hgt_in >= 59 && hgt_in <= 76))
                && HCL_REGEX.is_match(s)
                && PID_REGEX.is_match(s)
                && ECL_REGEX.is_match(s)
        })
        .count()
}

#[test]
fn test_example_invalid() {
    assert_eq!(
        solve(
            r"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
            "
            .trim()
        ),
        0
    );
}

#[test]
fn test_example_valid() {
    assert_eq!(
        solve(
            r"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
            "
            .trim()
        ),
        4
    );
}

#[test]
fn test() {
    let input = util::read_input("d4_passport_processing.txt");

    assert_eq!(solve(&input), 140);
}

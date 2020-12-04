use std::str::FromStr;
use std::collections::HashSet;
use simple_error::SimpleError;

// This one is similar to day2 in that I overcomplicated it by messing with
// FromStr and Options. Not recommended!

fn part1(input: &Vec<&str>) -> usize {
    let mut valid_count: usize = 0;

    let required_fields: HashSet<&'static str> =
        [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ].iter().cloned().collect();

    for passport in input {
        let mut fields_present = HashSet::new();
        for field_name in passport.split(|c| c == ' ' || c == '\n')
                                  .map(|f| f.get(0..3).unwrap()) {
            if field_name != "cid" {
                fields_present.insert(field_name);
            }
        }
        if fields_present == required_fields {
            valid_count +=1;
        }
    }
    return valid_count
}

#[derive(Debug, PartialEq)]
struct Passport {
    byr: u64,
    iyr: u64,
    eyr: u64,
    hgt: u64, // maths crimes! we dropped the unit for height, don't tell anyone
    hcl: String,
    ecl: String,
    pid: u64,
    cid: Option<u64>,
}

#[test]
fn test_passport_fromstr(){
    let valid = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
    assert_eq!(
        Passport::from_str(valid),
        Ok(Passport {
            byr: 1937,
            iyr: 2017,
            eyr: 2020,
            hgt: 183,
            hcl: "#fffffd".to_string(),
            ecl: "gry".to_string(),
            pid: 860033327,
            cid: Some(147),
           })
    );

    let missing_hgt = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929";
    assert!(Passport::from_str(missing_hgt).is_err());

    let missing_cid = "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm";
    assert!(!Passport::from_str(missing_cid).is_err());

    let year_not_a_number = "hcl:#ae17e1 iyr:foo\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm";
    assert!(Passport::from_str(year_not_a_number).is_err());

    let year_outside_valid_range = "hcl:#ae17e1 iyr:1984\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm";
    assert!(Passport::from_str(year_outside_valid_range).is_err());
}

impl FromStr for Passport {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut byr: Option<u64> = None;
        let mut iyr: Option<u64> = None;
        let mut eyr: Option<u64> = None;
        let mut hgt: Option<u64> = None;
        let mut hcl: Option<String> = None;
        let mut ecl: Option<String> = None;
        let mut pid: Option<u64> = None;
        let mut cid: Option<u64> = None;

        for mut field in s.split(|c| c == ' ' || c == '\n').map(|f| f.split(":")) {
            let field_name = field.next().unwrap();
            let value = field.next().unwrap();
            match field_name {
                "byr" => byr = parse_and_check_value_in_range(value, 1920, 2002),
                "iyr" => iyr = parse_and_check_value_in_range(value, 2010, 2020),
                "eyr" => eyr = parse_and_check_value_in_range(value, 2020, 2030),
                "hgt" => hgt = parse_height(value),
                "hcl" => hcl = parse_hcl(value),
                "ecl" => ecl = parse_ecl(value),
                "pid" => pid = parse_pid(value),
                "cid" => cid = value.parse::<u64>().ok(),
                _ => {},
            }
        }
        let passport = Passport {
            byr: byr.ok_or(SimpleError::new("Missing or invalid field byr"))?,
            iyr: iyr.ok_or(SimpleError::new("Missing or invalid field iyr"))?,
            eyr: eyr.ok_or(SimpleError::new("Missing or invalid field eyr"))?,
            hgt: hgt.ok_or(SimpleError::new("Missing or invalid field hgt"))?,
            hcl: hcl.ok_or(SimpleError::new("Missing or invalid field hcl"))?,
            ecl: ecl.ok_or(SimpleError::new("Missing or invalid field ecl"))?,
            pid: pid.ok_or(SimpleError::new("Missing or invalid field pid"))?,
            cid: cid,
        };
        Ok(passport)
    }
}

fn parse_and_check_value_in_range(val: &str, min: u64, max: u64) -> Option<u64> {
    match val.parse::<u64>() {
        Ok(x) if x >= min && x <= max => Some(x),
        _ => return None,
    }
}

fn parse_height(val: &str) -> Option<u64> {
    match val.strip_suffix("cm") {
        Some(x) => return parse_and_check_value_in_range(x, 150, 193),
        _ => {},
    }
    match val.strip_suffix("in") {
        Some(x) => return parse_and_check_value_in_range(x, 59, 76),
        _ => return None,
    }
}

fn parse_hcl(val: &str) -> Option<String> {
    if val.chars().count() == 7 &&
       val.chars().nth(0).unwrap() == '#' &&
       val.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
        Some(val.to_string())
    } else {
        None
    }
}

fn parse_ecl(val: &str) -> Option<String> {
    let valid_values = vec![
        "amb",
        "blu",
        "brn",
        "gry",
        "grn",
        "hzl",
        "oth",
    ];
    if valid_values.iter().any(|x| x == &val) {
        Some(val.to_string())
    } else {
        None
    }
}

fn parse_pid(val: &str) -> Option<u64> {
    if val.chars().count() == 9 {
        return val.parse::<u64>().ok()
    } else {
        None
    }
}


fn part2(input: &Vec<&str>) -> usize {
    return input.iter()
                .filter_map(|p| p.parse::<Passport>().ok())
                .count()
}

fn main() {
    let input: Vec<&str> = include_str!("input").trim()
                                                .split("\n\n")
                                                .collect();
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

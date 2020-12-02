use std::str::FromStr;
use simple_error::SimpleError;

#[derive(Debug, PartialEq, Clone, Copy)]
struct PasswordPolicy {
    letter: char,
    rule1: u32,
    rule2: u32,
}

#[test]
fn test_password_policy_fromstr() {
    let input = "1-3 a";
    let expected = PasswordPolicy {
        letter: 'a',
        rule1: 1,
        rule2: 3,
    };
    assert_eq!(PasswordPolicy::from_str(input), Ok(expected));
}

impl FromStr for PasswordPolicy {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let components: Vec<&str> = s.split(|c| c == '-' || c == ' ')
                                     .collect();

        if components.len() != 3 {
          return Err(SimpleError::new(
                  format!("Failed to parse &str {} into password policy", s)));
        }

        let rule1: u32 = components[0].parse().unwrap();
        let rule2: u32 = components[1].parse().unwrap();
        let letter: char = components[2].chars().next().unwrap();

        let policy = PasswordPolicy {
            letter: letter,
            rule1: rule1,
            rule2: rule2,
        };

        Ok(policy)
    }
}

#[derive(Debug, PartialEq)]
struct Password {
    policy: PasswordPolicy,
    password: String,
}

#[test]
fn test_password_fromstr() {
    let input = "1-3 a: aaaaa";
    let policy = PasswordPolicy {
        letter: 'a',
        rule1: 1,
        rule2: 3,
    };
    let expected = Password {
        policy: policy,
        password: "aaaaa".to_string()
    };

    assert_eq!(Password::from_str(input), Ok(expected));
}

impl FromStr for Password {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split(':').collect();
        let policy = PasswordPolicy::from_str(components[0]).unwrap();
        let password = components[1].trim().to_string();

        Ok(Password { policy: policy, password: password })
    }
}

#[test]
fn test_is_part1_password_valid(){
    let policy = PasswordPolicy {
        letter: 'e',
        rule1: 3,
        rule2: 5,
    };

    let good_password = Password {
        policy: policy,
        password: "0xdeadbeef".to_string(),
    };
    assert!(is_part1_password_valid(good_password));

    let too_few_chars = Password {
        policy: policy,
        password: "changeme".to_string(),
    };
    assert!(!is_part1_password_valid(too_few_chars));

    let too_many_chars = Password {
        policy: policy,
        password: "changemeeeeeeee".to_string(),
    };
    assert!(!is_part1_password_valid(too_many_chars));
}

// in part 1 we treat rule1 as minimum repetitions of a character and rule2
// as maximum repetitions
fn is_part1_password_valid(p: Password) -> bool {
    let count: u32 = p.password.chars()
                          .fold(0, |acc, c|
                              if c == p.policy.letter { acc + 1 }
                              else { acc });

    return count >= p.policy.rule1 &&
           count <= p.policy.rule2
}

#[test]
fn test_is_part2_password_valid(){
    let policy = PasswordPolicy {
        letter: 'e',
        rule1: 4,
        rule2: 6,
    };

    let char_in_position_one_only = Password {
        policy: policy,
        password: "0xdeadbeef".to_string(),
    };
    assert!(is_part2_password_valid(char_in_position_one_only));

    let char_in_position_two_only = Password {
        policy: policy,
        password: "changeme".to_string(),
    };
    assert!(is_part2_password_valid(char_in_position_two_only));

    let char_in_both_positions = Password {
        policy: policy,
        password: "eeeeeee".to_string(),
    };
    assert!(!is_part2_password_valid(char_in_both_positions));

    let char_in_neither_position = Password {
        policy: policy,
        password: "aaaaaaaa".to_string(),
    };
    assert!(!is_part2_password_valid(char_in_neither_position));
}

// In part 2 we treat rule1 and rule2 as positions, where the specified char
// MUST be in EITHER the position specified by rule1 OR the position specified
// by rule2 (but not both)
// Also char positions start from 1, not 0
fn is_part2_password_valid(p: Password) -> bool {
    let mut pass = p.password.chars();

    let pos1: usize = p.policy.rule1 as usize - 1;

    // When we call nth() below we consume the first n items of the iterator
    // So pos2 needs to be an offset from pos1
    let pos2: usize = (p.policy.rule2 - p.policy.rule1) as usize - 1;

    let char1 = pass.nth(pos1).unwrap();
    let char2 = pass.nth(pos2).unwrap();

    return (char1 == p.policy.letter || char2 == p.policy.letter) &&
           char1 != char2
}

fn part1(input: &Vec<&str>) -> usize {
    return input.iter()
                .filter(|x| is_part1_password_valid(Password::from_str(x).unwrap()))
                .count();
}

fn part2(input: &Vec<&str>) -> usize {
    return input.iter()
                .filter(|x| is_part2_password_valid(Password::from_str(x).unwrap()))
                .count();
}

fn main() {
      let input = util::input_to_str_vec(include_str!("input"));
      println!("Part 1: Answer is {}", part1(&input));
      println!("Part 2: Answer is {}", part2(&input));
}

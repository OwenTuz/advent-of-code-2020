use std::collections::HashSet;

#[test]
fn test_count_combined_answers(){
    assert_eq!(count_combined_answers("a\nb\nc"), 3);
    assert_eq!(count_combined_answers("abc\nb\nc\nb"), 3);
    assert_eq!(count_combined_answers("a\nbc\nac\nb\nz"), 4);
}
fn count_combined_answers(group: &str) -> usize {
    let mut set = HashSet::new();
    for ans in group.split('\n').collect::<String>().chars() {
        set.insert(ans);
    }
    return set.len()
}

fn part1(input: &Vec<&str>) -> usize {
    input.iter().fold(0, |acc, g| acc + count_combined_answers(g))
}

#[test]
fn test_count_common_answers(){
    assert_eq!(count_common_answers("a\na\na\na"), 1);
    assert_eq!(count_common_answers("ab\nac"), 1);
    assert_eq!(count_common_answers("a\nb\nc"), 0);
}
fn count_common_answers(group: &str) -> usize {
    let mut set: HashSet<_> = HashSet::new();
    for c in group.split('\n').nth(0).unwrap().chars() {
        set.insert(c);
    }
    for ans in group.split('\n') {
        let answer_set: HashSet<char> = ans.chars().collect();
        set = set.intersection(&answer_set).cloned().collect();
    }
    return set.len()
}

fn part2(input: &Vec<&str>) -> usize {
    input.iter().fold(0, |acc, g| acc + count_common_answers(g))
}

fn main() {
    let input: Vec<&str> = include_str!("input").trim()
                                                .split("\n\n")
                                                .collect();
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

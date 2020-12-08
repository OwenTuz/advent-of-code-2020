use std::collections::HashMap;

type Rules = HashMap<String, HashMap<String, usize>>;

#[test]
fn test_bag_tree(){
    let input = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dotted black bags contain no other bags.",
    ];
    let mut expected: Rules = HashMap::new();

    let mut light_red_contains: HashMap<String,usize> = HashMap::new();
    let dotted_black_contains: HashMap<String,usize> = HashMap::new();

    light_red_contains.insert("bright white".to_string(), 1);
    light_red_contains.insert("muted yellow".to_string(), 2);

    expected.insert("light red".to_string(), light_red_contains.clone());
    expected.insert("dotted black".to_string(), dotted_black_contains);

    assert_eq!(
        bag_tree(&input).get("light red"),
        Some(&light_red_contains)
    );
    assert!(
        bag_tree(&input).get("dotted black").unwrap().is_empty()
    );
}
fn bag_tree(input: &Vec<&str>) -> Rules {
    let mut tree: Rules = HashMap::new();
    for line in input {
        let mut rule = line.split(" bags contain ");
        let bag_colour = rule.next().unwrap().to_string();

        let mut contains: HashMap<String, usize> = HashMap::new();

        for contents in rule.next().unwrap().split(", ") {
            let mut num: usize = 0;

            let mut words = contents.split(' ');
            let first_word = words.next().unwrap();

            match first_word.parse::<usize>() {
                Ok(x) => num = x,
                _ => {},
            }
            if num > 0 {
                contains.insert(
                    words.take(2).collect::<Vec<&str>>().join(" ").to_string(),
                    num,
                );
            }
        }
        tree.insert(
            bag_colour,
            contains,
        );
    }
    return tree
}

fn contains_gold_bag(rules: &Rules, bag_colour: &str) -> bool {
    rules[bag_colour].iter()
                     .any(|(colour,_)| colour == "shiny gold" ||
                                      contains_gold_bag(rules, colour))
}

fn part1(input: &Vec<&str>) -> usize {
    let rules = bag_tree(input);
    rules.keys()
         .filter(|bag| contains_gold_bag(&rules, bag))
         .count()
}

fn count_contents(rules: &Rules, bag_colour: &str) -> usize {
    rules[bag_colour].iter()
                     .fold(0, |total, (colour,num)|
                           total + num + num * count_contents(rules, colour))
}

fn part2(input: &Vec<&str>) -> usize {
    count_contents(&bag_tree(input), "shiny gold")
}

fn main() {
    let input = util::input_to_str_vec(include_str!("input"));
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

use itertools::Itertools;

#[test]
fn test_find_numbers_which_sum_to_target(){
    let input = vec![0, 2, 4, 6, 8];
    assert_eq!(find_numbers_which_sum_to_target(&input, 14), (6,8));
    assert_eq!(find_numbers_which_sum_to_target(&input, 6), (0,6));
    assert_ne!(find_numbers_which_sum_to_target(&input, 6), (2,4));
}
#[test]
#[should_panic]
fn test_panics_if_cant_find_valid_answer(){
    // This should never happen, because it's a puzzle and there should always
    // be a valid answer. But let's test it, just this once.
    find_numbers_which_sum_to_target(&vec![0,1], 2020);
}

// Returns the first two numbers in the given input which sum to `target`
// then stops searching
fn find_numbers_which_sum_to_target(input: &Vec<i32>, target: i32) -> (i32, i32) {
    let length = input.len();
    for (index, value) in input.iter().enumerate() {
        // assumption: all values are positive integers
        // we can cheat on this since we've seen the input
        if value >= &target {
            continue
        }
        for i in index + 1..length {
            if value + input[i] == target {
                return (*value, input[i])
            }
        }
    }
    panic!("Did not find numbers in input which sum to target {}", target);
}

#[test]
fn test_find_three_numbers_which_sum_to_target(){
    assert_eq!(
        find_three_numbers_which_sum_to_target(&vec![0,1,2,3,4,5], 6),
        vec![&0,&1,&5]
    );
}
// Alternative approach using the itertools crate
fn find_three_numbers_which_sum_to_target(input: &Vec<i32>, target: i32) -> Vec<&i32> {
    input.iter().combinations(3)
         .find(|t| t[0] + t[1] + t[2] == target).unwrap()
}

fn part1(input: &Vec<i32>) -> i32 {
    let (x, y) = find_numbers_which_sum_to_target(input, 2020);
    return x * y
}

fn part2(input: &Vec<i32>) -> i32 {
    let res: i32 = find_three_numbers_which_sum_to_target(input, 2020)
        .iter()
        .fold(1, |acc, x| acc * **x);
    res
}

fn main() {
    let input: Vec<i32> = util::input_to_vec_t_fromstr(include_str!("input"), '\n');
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

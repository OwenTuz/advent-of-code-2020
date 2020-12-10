fn part1(input: &Vec<u64>) -> u64 {
    let mut diff_one = 0;
    let mut diff_three = 1;
    let mut prev_joltage = 0;

    for x in input {
        let diff = x - prev_joltage;
        if diff > 0 && diff <= 3 {
            prev_joltage = *x;
            match diff {
                1 => diff_one += 1,
                3 => diff_three += 1,
                _ => {},
            }
        }
    }
    return diff_one * diff_three
}

fn part2(input: &Vec<u64>) -> u64 {
    let mut adapters = input.clone();
    adapters.insert(0, 0);
    let last = adapters.pop().unwrap();
    adapters.push(last);
    adapters.push(last + 3);
    let mut variations = 1;
    let mut group_size = 0;
    for i in 1..adapters.len() {
        group_size += 1;
        if adapters[i] - adapters[i-1] > 2 {
            match group_size {
                3 => variations *= 2,
                4 => variations *= 4,
                5 => variations *= 7,
                x if x > 5 => panic!("Found group of {} adapters, expand lookup table", x),
                _ => {},
            }
            group_size = 0;
        }
    }
    return variations
}

fn main() {
    let mut input: Vec<u64> = util::input_to_vec_t_fromstr(include_str!("input"), '\n');
    input.sort();
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

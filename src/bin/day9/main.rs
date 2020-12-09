fn is_number_sum_of_two_in_vec(num: usize, vec: &Vec<usize>) -> bool {
    let mut stack = vec.clone();
    while let Some(x) = stack.pop() {
        for i in 0..stack.len() {
            if x + stack[i] == num {
                return true
            }
        }
    }
    false
}

fn part1(input: &Vec<usize>) -> usize {
    let mut preamble = input.clone();
    let mut data = preamble.split_off(25);
    data.reverse();
    while let Some(num) = data.pop() {
        if !is_number_sum_of_two_in_vec(num, &preamble) {
            return num
        }
        preamble.remove(0);
        preamble.push(num);
    }
    return 0 // we shouldn't get here, as there should always be an answer
}

fn part2(input: &Vec<usize>) -> usize {
    // do part1 again to get our target value
    let target: usize = part1(input);
    for i in 2..input.len() {
        let windows: Vec<&[usize]> = input.windows(i)
                           .filter(|w| w.iter().sum::<usize>() == target)
                           .collect();
        match windows.get(0) {
            Some(w) => return w.iter().max().unwrap() + w.iter().min().unwrap(),
            _ => {}
        };
    }
    return 0 // more dodgy return hacks
}

fn main() {
    let input: Vec<usize> = util::input_to_vec_t_fromstr(include_str!("input"), '\n');
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

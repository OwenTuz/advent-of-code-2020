struct Slope {
    right: usize,
    down: usize,
}

#[test]
fn test(){
    let input = &vec![
        "..##.........##.........##.........##.........##.........##.......",
        "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..",
        ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.",
        "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#",
        ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.",
        "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....",
        ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#",
        ".#........#.#........#.#........#.#........#.#........#.#........#",
        "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...",
        "#...##....##...##....##...##....##...##....##...##....##...##....#",
        ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#",
    ];
    let slope = Slope { right: 3, down: 1 };
    assert_eq!(count_trees_collided_for_slope(input, slope), 7);
}

fn count_trees_collided_for_slope(input: &Vec<&str>, slope: &Slope) -> usize {
    let mut current_row: usize = slope.down;
    let mut current_column: usize = slope.right;
    let mut tree_count: usize = 0;

    while current_row < input.len() {
        if input[current_row].chars().cycle().nth(current_column) == Some('#') {
            tree_count += 1;
        }
        current_column += slope.right;
        current_row += slope.down;
    }
    return tree_count
}

fn part1(input: &Vec<&str>) -> usize {
    let slope = Slope { right: 3, down: 1 };
    return count_trees_collided_for_slope(input, &slope);
}

fn part2(input: &Vec<&str>) -> usize {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];
    return slopes.iter()
                 .fold(1, |x, s| x * count_trees_collided_for_slope(input, s));
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: u32,
    column: u32,
    seat_id: u32,
}

#[test]
fn test_pass_from_string() {
    let expected = BoardingPass {
        row: 44,
        column: 5,
        seat_id: 357,
    };
    assert_eq!(pass_from_string("FBFBBFFRLR"), expected);
}

// Turns out we only ever use the seat ID for the puzzles, so this could
// be faster/simpler if it just returned the string parsed to u32
fn pass_from_string(s: &str) -> BoardingPass {
    let bin_string = s.replace("F", "0")
                      .replace("B", "1")
                      .replace("L", "0")
                      .replace("R", "1");
    let seat_id = u32::from_str_radix(&bin_string, 2).unwrap();
    return BoardingPass {
        seat_id: seat_id,
        row: seat_id >>3,
        column: seat_id & 0b111u32,
    }
}

fn part1(input: &Vec<&str>) -> u32 {
    return input.iter()
                .map(|x| pass_from_string(x).seat_id)
                .max().unwrap()
}

fn part2(input: &Vec<&str>) -> u32 {
    let mut a: Vec<u32> = input.iter()
                               .map(|x| pass_from_string(x).seat_id)
                               .collect();
    a.sort();

    let mut id: u32 = 0;
    for i in 1..a.len() {
        if a[i] - a[i-1] == 2 {
            id = a[i] - 1;
        }
    }
    return id
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

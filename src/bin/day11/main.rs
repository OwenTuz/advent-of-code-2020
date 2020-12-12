use std::mem;
use std::convert::TryFrom;

type Grid = Vec<Vec<char>>;

#[test]
fn test_state_change(){
    let empty_seats_grid = vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
    ].iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Grid>();

    assert_eq!(state_change((0,0), &empty_seats_grid), Some('#'));
    assert_eq!(state_change((0,1), &empty_seats_grid), None);
    assert_eq!(state_change((1,1), &empty_seats_grid), Some('#'));

    let mixed_seats_grid = vec![
        "#.LL.L#.##",
        "#LLLLLL.L#",
        "L.L.L..L..",
    ].iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Grid>();

    assert_eq!(state_change((0,0), &mixed_seats_grid), None);
    assert_eq!(state_change((1,9), &mixed_seats_grid), None);
    assert_eq!(state_change((1,2), &mixed_seats_grid), Some('#'));
    assert_eq!(state_change((1,3), &mixed_seats_grid), Some('#'));

    let full_seats_grid = vec![
        "#.##.##.##",
        "#######.##",
        "#.#.#..#..",
    ].iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Grid>();

    assert_eq!(state_change((0,2), &full_seats_grid), Some('L'));

}
fn state_change(seat: (usize, usize), grid: &Grid) -> Option<char> {
    let row = seat.0;
    let col = seat.1;

    let mut seat_full = false;

    match grid[row][col] {
        '.' => return None, // No state change possible for floor spaces
        '#' => seat_full = true,
        'L' => {},
        _   => panic!("Unexpected value in grid: {:?}", grid[col][row]),
    };

    let mut adj_filled = 0;
    let mut start: usize;

    // row above
    if row > 0 { // can't subtract from 0 because we're using usize
        let row_above = row - 1;
        start = col;
        if col > 0 {
            start = col - 1;
        }
        for i in start..=(col + 1) {
            if let Some(c) = grid[row_above].get(i) {
                match c {
                    '#' => adj_filled += 1,
                    _ => {},
                }
            }
        }
    }
    // row below
    if let Some(row_below) = grid.get(row + 1) {
        start = col;
        if col > 0 {
            start = col - 1;
        }
        for i in start..=(col + 1) {
            if let Some(c) = row_below.get(i) {
                match c {
                    '#' => adj_filled += 1,
                    _ => {},
                }
            }
        }
    }
    // left of current position
    if col > 0 {
        match grid[row][col - 1] {
            '#' => adj_filled += 1,
            _ => {},
        }
    }

    match grid[row].get(col + 1) {
        Some('#') => adj_filled += 1,
        _   => {},
    }

    match adj_filled {
        x if x >= 4 && seat_full => Some('L'),
        0 if ! seat_full => Some('#'),
        _ => None,
    }
}

#[test]
fn test_part1(){
    let test_grid = vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ].iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Grid>();
    assert_eq!(part1(&test_grid), 37);
}
fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    loop {
        let mut changed = false;

        // all values change simultaneously, so we freeze a copy of grid state
        // and update the original in place with each loop
        let old_grid = grid.clone();

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if let Some(c) = state_change((row, col), &old_grid) {
                    changed = true;
                    let _ = mem::replace(&mut grid[row][col], c);
                }
            }
        }

        if ! changed {
            break
        }
    }
    return grid.iter()
               .fold(0, |acc, r| acc + r.iter().filter(|s| **s == '#').count())

}

fn state_change_part2(seat: (usize, usize), grid: &Grid) -> Option<char> {
    let row = seat.0;
    let col = seat.1;

    let mut seat_full = false;

    match grid[row][col] {
        '.' => return None, // No state change possible for floor spaces
        '#' => seat_full = true,
        'L' => {},
        _   => panic!("Unexpected value in grid: {:?}", grid[col][row]),
    };

    let mut adj_filled = 0;

    // vec of offsets where: tuple.1 = row and tuple.2 = column
    let adjacents: Vec<(isize, isize)> = vec![
        (0,1), // right
        (0,-1), // left
        (-1,-1), // top left diagonal
        (-1,0), // above
        (-1,1), // top right diagonal
        (1,-1), // bottom left diagonal
        (1,0), // below
        (1,1), // bottom right diagonal
    ];

    for pos in adjacents {
        adj_filled += find_visible(seat, pos, grid);
    }

    match adj_filled {
        x if x >= 5 && seat_full => Some('L'),
        0 if ! seat_full => Some('#'),
        _ => None,
    }
}

fn find_visible(seat: (usize, usize), pos: (isize, isize), grid: &Grid) -> usize {
    let row = seat.0;
    let col = seat.1;
    if let Some(new_row) = usize::try_from(row as isize + pos.0).ok() {
        if let Some(new_col) = usize::try_from(col as isize + pos.1).ok() {
            if let Some(r) = grid.get(new_row) {
                match r.get(new_col) {
                    Some('#') => return 1,
                    Some('.') => {
                        let next_pos_row = match pos.0 {
                            x if x > 0 => x + 1,
                            x if x < 0 => x - 1,
                            x => x,
                        };
                        let next_pos_col = match pos.1 {
                            y if y > 0 => y + 1,
                            y if y < 0 => y - 1,
                            y => y,
                        };
                        let next_pos = (next_pos_row, next_pos_col);
                        return find_visible(seat, next_pos, grid)
                    },
                    _   => return 0,
                }
            }
        }
    }
    return 0
}


fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    loop {
        let mut changed = false;
        let old_grid = grid.clone();

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if let Some(c) = state_change_part2((row, col), &old_grid) {
                    changed = true;
                    let _ = mem::replace(&mut grid[row][col], c);
                }
            }
        }

        if ! changed {
            break
        }
    }
    return grid.iter()
               .fold(0, |acc, r| acc + r.iter().filter(|s| **s == '#').count())
}


fn build_grid() -> Grid {
    let rows = util::input_to_str_vec(include_str!("input"));
    let mut grid: Grid = Vec::new();
    for row in rows {
        grid.push(row.chars().collect::<Vec<char>>());
    }
    return grid
}

fn main() {
    let input = build_grid();
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

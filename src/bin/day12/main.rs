#[derive(Debug, PartialEq,Clone,Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq)]
enum Action {
    N,
    E,
    S,
    W,
    L,
    R,
    F,
}

#[derive(Debug, PartialEq)]
struct Ferry {
    x: i32,
    y: i32,
    facing: Direction,
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            x: 0,
            y: 0,
            facing: Direction::E,
        }
    }

    fn travel(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::N => self.y += distance,
            Direction::S => self.y -= distance,
            Direction::E => self.x += distance,
            Direction::W => self.x -= distance,
        }
    }

    fn turn(&mut self, action: Action, degrees: i32) {
        let dirs = vec![
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ];

        match action {
            Action::L => {
                let mut iter = dirs.iter().rev().cycle();
                while let Some(d) = iter.next() {
                    if *d == self.facing {
                        for _ in 0..(degrees/90) {
                            self.facing = *iter.next().unwrap();
                        }
                        break
                    }
                }
            },
            Action::R => {
                let mut iter = dirs.iter().cycle();
                while let Some(d) = iter.next() {
                    if *d == self.facing {
                        for _ in 0..(degrees/90) {
                            self.facing = *iter.next().unwrap();
                        }
                        break
                    }
                }
            },
            _         => panic!("Action {:?} cannot turn ferry", action),
        }
    }
    fn turn_relative(&mut self, ferry: &Ferry, action: Action, degrees: i32) {
        let dirs = vec![
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ];

        match action {
            Action::L => {
                let mut iter = dirs.iter().rev().cycle();
                while let Some(d) = iter.next() {
                    if *d == self.facing {
                        for _ in 0..(degrees/90) {
                            self.facing = *iter.next().unwrap();
                            let diff_x = self.x - ferry.x;
                            let diff_y = self.y - ferry.y;
                            match self.facing {
                                Direction::N | Direction::S => {
                                    self.y = ferry.y + diff_x;
                                    self.x = ferry.x - diff_y;
                                },
                                Direction::E | Direction::W => {
                                    self.y = ferry.y + diff_x;
                                    self.x = ferry.x - diff_y;
                                },
                            }
                        }
                        break
                    }
                }
            },
            Action::R => {
                let mut iter = dirs.iter().cycle();
                while let Some(d) = iter.next() {
                    if *d == self.facing {
                        for _ in 0..(degrees/90) {
                            self.facing = *iter.next().unwrap();
                            let diff_x = self.x - ferry.x;
                            let diff_y = self.y - ferry.y;
                            match self.facing {
                                Direction::N | Direction::S => {
                                    self.y = ferry.y - diff_x;
                                    self.x = ferry.x + diff_y;
                                },
                                Direction::E | Direction::W => {
                                    self.y = ferry.y - diff_x;
                                    self.x = ferry.x + diff_y;
                                },
                            }
                        }
                        break
                    }
                }
            },
            _         => panic!("Action {:?} cannot turn ferry", action),
        }
    }
}

fn parse_step(s: &str) -> (Action, i32) {
    let mut step = s.chars();
    let action = match step.next().unwrap() {
        'N' => Action::N,
        'E' => Action::E,
        'S' => Action::S,
        'W' => Action::W,
        'L' => Action::L,
        'R' => Action::R,
        'F' => Action::F,
        _   => panic!("Unrecognised action in step {}", s),
    };
    let val: i32 = step.collect::<String>().parse().unwrap();
    return (action, val)
}

fn part1(input: &Vec<(Action, i32)>) -> i32 {
    let mut ferry = Ferry::new();

    for step in input {
        match step.0 {
            Action::N => ferry.travel(Direction::N, step.1),
            Action::S => ferry.travel(Direction::S, step.1),
            Action::E => ferry.travel(Direction::E, step.1),
            Action::W => ferry.travel(Direction::W, step.1),
            Action::F => ferry.travel(ferry.facing.clone(), step.1),
            Action::R => ferry.turn(Action::R, step.1),
            Action::L => ferry.turn(Action::L, step.1),
        }
    }
    return ferry.x.abs() + ferry.y.abs()
}

fn part2(input: &Vec<(Action, i32)>) -> i32 {
    let mut ferry = Ferry::new();
    let mut waypoint = Ferry { x: 10, y: 1, facing: Direction::E };

    for step in input {
        match step.0 {
            Action::N => waypoint.travel(Direction::N, step.1),
            Action::S => waypoint.travel(Direction::S, step.1),
            Action::E => waypoint.travel(Direction::E, step.1),
            Action::W => waypoint.travel(Direction::W, step.1),
            Action::R => waypoint.turn_relative(&ferry, Action::R, step.1),
            Action::L => waypoint.turn_relative(&ferry, Action::L, step.1),
            Action::F => {
                let diff_x = waypoint.x - ferry.x;
                let diff_y = waypoint.y - ferry.y;
                ferry.x = ferry.x + step.1 * diff_x;
                ferry.y = ferry.y + step.1 * diff_y;
                waypoint.x = ferry.x + diff_x;
                waypoint.y = ferry.y + diff_y;
            }
        }
    }
    return ferry.x.abs() + ferry.y.abs()
}

fn main() {
    let input: Vec<(Action, i32)> = include_str!(
        "input"
    ).trim()
     .split('\n')
     .map(|s| parse_step(s)).collect();

    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

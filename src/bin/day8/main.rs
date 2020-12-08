use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Program<'a> {
    pc: usize,
    acc: isize,
    code: Vec<(&'a str, isize)>,
}

impl<'a> Program<'a> {
    fn new(code: &Vec<&'a str>) -> Program<'a> {
        Program {
            pc: 0,
            acc: 0,
            code: code.iter().clone()
                      .map(|i| {
                            let mut split = i.split_whitespace();
                            (split.next().unwrap(),
                             split.next().unwrap().parse::<isize>().unwrap())
                      }).collect::<Vec<(&'a str, isize)>>(),
        }
    }

    fn step(pc: &mut usize, acc: &mut isize, instruction: (&'a str, isize)) -> (isize, usize) {
        match instruction.0 {
            "nop" => *pc += 1,
            "acc" => { *acc += instruction.1 ; *pc += 1 },
            "jmp" => { if instruction.1 < 0 { *pc -= instruction.1.abs() as usize}
                       else { *pc += instruction.1.abs() as usize } },
            _     => panic!("Unimplemented instruction {}", instruction.0),
        }
        return (*acc, *pc)
    }
}

impl<'a> Iterator for Program<'a> {
    // We will return the value of the accumulator with each call to next()
    type Item = (isize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.code.get(self.pc) {
            Some(instruction) => Some(Self::step(&mut self.pc, &mut self.acc, *instruction)),
            None => {
                match self.pc {
                    p if p == self.code.len() => None,
                    _ => panic!(
                            "Segfault! Invalid pc value {} (code length: {})",
                            self.pc,
                            self.code.len()
                        )
                }
            },
        }
    }
}

fn part1(input: &Vec<&str>) -> isize {
    let program = Program::new(input);
    let mut pc_values_visited = HashSet::new();
    for (acc, pc) in program {
        if ! pc_values_visited.insert(pc) {
            return acc
        }
    }
    // Bad error handling: if we try to reach a nonexistent instruction, we
    // will panic in Program::next()
    // Otherwise, we will loop until we have visited a location twice
    panic!("This code should be unreachable!");
}

fn part2(input: &Vec<&str>) -> isize {
    let mut curr_acc: isize = 0;
    'outer: for i in 0..input.len() {
        let mut split = input.get(i).unwrap().split_whitespace();
        let swap = split.next().unwrap();
        let arg = split.next().unwrap();
        if swap != "acc" {

            let new = match swap {
                "jmp" => vec![ "nop", arg ].join(" "),
                "nop" => vec![ "jmp", arg ].join(" "),
                _     => panic!("Unexpected instruction {} when swapping", swap),
            };
            let mut code = input.clone();
            code[i] = new.as_str();

            let program = Program::new(&code);
            let mut pc_values_visited = HashSet::new();
            for (acc, pc) in program.take_while(|(_, pc)| *pc != input.len()) {
                curr_acc = acc;
                if ! pc_values_visited.insert(pc) {
                    continue 'outer;
                }
            }
            break
        }
    }
    return curr_acc
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}

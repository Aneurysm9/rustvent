pub struct Runner {
    pub input: String,
}

#[derive(Debug)]
enum Instruction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn parse_op(input: std::slice::Iter<'_, char>) -> i32 {
    input
        .collect::<String>()
        .parse::<i32>()
        .expect("Unable to parse instruction magnitude")
}

fn parse_input(input: &str) -> impl Iterator<Item=Instruction> + '_ {
    input
        .trim()
        .lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();
            match chars[0] {
                'N' => Instruction::North(parse_op(chars[1..chars.len()].iter())),
                'E' => Instruction::East(parse_op(chars[1..chars.len()].iter())),
                'S' => Instruction::South(parse_op(chars[1..chars.len()].iter())),
                'W' => Instruction::West(parse_op(chars[1..chars.len()].iter())),
                'L' => Instruction::Left(parse_op(chars[1..chars.len()].iter())),
                'R' => Instruction::Right(parse_op(chars[1..chars.len()].iter())),
                'F' => Instruction::Forward(parse_op(chars[1..chars.len()].iter())),
                _ => panic!("Unexpected instruction"),
            }
        })
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut pos = [0, 0];
        let mut facing = 90;
        parse_input(&self.input).for_each(|i| match i {
            Instruction::North(m) => pos[1] += m,
            Instruction::East(m) => pos[0] += m,
            Instruction::South(m) => pos[1] -= m,
            Instruction::West(m) => pos[0] -= m,
            Instruction::Left(m) => facing = (((facing - m) % 360) + 360) % 360,
            Instruction::Right(m) => facing = (((facing + m) % 360) + 360) % 360,
            Instruction::Forward(m) => match facing {
                0 => pos[1] += m,
                90 => pos[0] += m,
                180 => pos[1] -= m,
                270 => pos[0] -= m,
                _ => panic!("Unexpected facing direction"),
            },
        });
        (pos[0].abs() + pos[1].abs()).to_string()
    }

    fn run_b(&self) -> String {
        let mut pos = [0, 0];
        let mut wp = [10, 1];
        parse_input(&self.input).for_each(|i| match i {
            Instruction::North(m) => wp[1] += m,
            Instruction::East(m) => wp[0] += m,
            Instruction::South(m) => wp[1] -= m,
            Instruction::West(m) => wp[0] -= m,
            Instruction::Left(m) => {
                let mut c = m;
                while c > 0 {
                    c -= 90;
                    wp = [-1*wp[1], wp[0]]
                }
            },
            Instruction::Right(m) => {
                let mut c = m;
                while c > 0 {
                    c -= 90;
                    wp = [wp[1], -1*wp[0]]
                }
            },
            Instruction::Forward(m) => pos = [pos[0]+(wp[0]*m), pos[1]+(wp[1]*m)],
        });
        (pos[0].abs() + pos[1].abs()).to_string()
    }
}

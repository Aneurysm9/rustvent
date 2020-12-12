pub struct Runner {
    pub input: String,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbors(&self, n: &mut Vec<Point>) {
        for x in -1i32..2 {
            for y in -1i32..2 {
                let nx = self.x as i32 + x;
                let ny = self.y as i32 + y;
                if (x == 0 && y == 0) || nx < 0 || ny < 0 {
                    continue;
                }
                n.push(Point {
                    x: nx as usize,
                    y: ny as usize,
                })
            }
        }
    }

    fn visible_neighbors(&self, grid: &Vec<Vec<char>>, n: &mut Vec<Point>) {
        for x in -1i32..2 {
            for y in -1i32..2 {
                let nx = self.x as i32 + x;
                let ny = self.y as i32 + y;
                if (x == 0 && y == 0)
                    || nx < 0
                    || ny < 0
                    || ny >= grid.len() as i32
                    || nx >= grid[ny as usize].len() as i32
                {
                    continue;
                }
                let mut p = Point {
                    x: nx as usize,
                    y: ny as usize,
                };
                while grid[p.y][p.x] == '.' {
                    let nx = p.x as i32 + x;
                    let ny = p.y as i32 + y;
                    if nx < 0
                        || ny < 0
                        || ny >= grid.len() as i32
                        || nx >= grid[ny as usize].len() as i32
                    {
                        break;
                    }
                    p = Point {
                        x: nx as usize,
                        y: ny as usize,
                    }
                }
                n.push(p)
            }
        }
    }
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        run(&self.input, false)
    }

    fn run_b(&self) -> String {
        run(&self.input, true)
    }
}

fn run(input: &str, part_b: bool) -> String {
    let mut grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    loop {
        let new = step(&grid, part_b);
        if new == grid {
            break;
        }
        grid = new;
    }

    grid.iter()
        .fold(0, |i, l| {
            i + l.iter().fold(0, |j, c| if *c == '#' { j + 1 } else { j })
        })
        .to_string()
}

fn step(input: &Vec<Vec<char>>, part_b: bool) -> Vec<Vec<char>> {
    input
        .clone()
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.clone()
                .iter()
                .enumerate()
                .map(|(x, tile)| {
                    let p = Point { x, y };
                    let mut count = 0;
                    let mut res = *tile;
                    let mut nv: Vec<Point> = Vec::new();
                    if part_b {
                        p.visible_neighbors(input, &mut nv);
                    } else {
                        p.neighbors(&mut nv);
                    }
                    for n in nv.iter() {
                        if n.y >= input.len() || n.x >= input[y].len() {
                            continue;
                        }
                        if input[n.y][n.x] == '#' {
                            count += 1;
                        }
                    }
                    match *tile {
                        'L' => {
                            if count == 0 {
                                res = '#';
                            }
                        }
                        '#' => {
                            if (part_b && count >= 5) || (!part_b && count >= 4) {
                                res = 'L';
                            }
                        }
                        _ => (),
                    }
                    res
                })
                .collect()
        })
        .collect()
}

use std::{collections::HashMap, ops::Range};

#[derive(PartialEq, Debug, Eq, PartialOrd, Ord, Clone, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

struct Part {
    pos: Pos,
    ratios: Vec<usize>,
}

impl Pos {
    fn adj(&self, bounds: Pos) -> [Option<Pos>; 8] {
        [
            if self.x > bounds.x {
                None
            } else {
                Some(Pos {
                    x: self.x + 1,
                    y: self.y,
                })
            },
            if self.y > bounds.y {
                None
            } else {
                Some(Pos {
                    x: self.x,
                    y: self.y + 1,
                })
            },
            if self.x == 0 {
                None
            } else {
                Some(Pos {
                    x: self.x - 1,
                    y: self.y,
                })
            },
            if self.y == 0 {
                None
            } else {
                Some(Pos {
                    x: self.x,
                    y: self.y - 1,
                })
            },
            if *self == (Pos { x: 0, y: 0 }) {
                None
            } else {
                Some(Pos {
                    x: self.x - 1,
                    y: self.y - 1,
                })
            },
            if *self == bounds {
                None
            } else {
                Some(Pos {
                    x: self.x + 1,
                    y: self.y + 1,
                })
            },
            if *self == (Pos { x: bounds.x, y: 0 }) {
                None
            } else {
                Some(Pos {
                    x: self.x + 1,
                    y: self.y - 1,
                })
            },
            if *self == (Pos { x: 0, y: bounds.y }) {
                None
            } else {
                Some(Pos {
                    x: self.x - 1,
                    y: self.y + 1,
                })
            },
        ]
    }
}

fn main() {
    part2()
}

fn part2() {
    let input: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();
    // let gears = Vec::new();
    let symbol_positions: Vec<Pos> = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '*').then(|| Pos { x, y }))
        })
        .collect();
    let width = input[0].len() - 1;
    let height = input.len() - 1;
    println!("Width: {}, Height: {}", width + 1, height + 1);
    let input: Vec<Vec<char>> = input
        .iter()
        .map(|string| string.chars().collect())
        .collect();
    let mut parts: HashMap<Pos, Vec<(Pos, usize)>> = HashMap::new();
    for symbol in symbol_positions {
        for Pos { x, y } in symbol
            .adj(Pos {
                x: width,
                y: height,
            })
            .iter()
            .flatten()
            .map(|pos| pos.clone())
        {
            if !input[y][x].is_ascii_digit() {
                continue;
            }
            let mut num = String::new();
            let mut offset: isize = 0;
            while input[y][(x as isize - offset) as usize].is_ascii_digit() {
                offset += 1;
                if offset > x as isize {
                    break;
                }
            }
            if offset > 0 {
                offset -= 1;
            }
            let num_pos = Pos {
                x: (x as isize - offset) as usize,
                y,
            };
            let part: &mut Vec<(Pos, usize)> = match parts.get_mut(&symbol) {
                None => {
                    parts.insert(symbol.clone(), Vec::new());
                    parts.get_mut(&symbol).unwrap()
                }
                Some(x) => x,
            };
            while input[y][(x as isize - offset) as usize].is_ascii_digit() {
                num.push(input[y][(x as isize - offset) as usize]);
                offset -= 1;
                if offset > x as isize || x as isize - offset > width as isize {
                    break;
                }
            }
            let num = num.parse::<usize>().unwrap();
            if part.contains(&(num_pos.clone(), num)) {
                continue;
            }
            part.push((num_pos, num));
        }
    }
    println!("Len: {:?}", parts);
    let sum = parts
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0].1 * v[1].1)
        .sum::<usize>();
    println!("Part2: {}", sum);
}

fn part1() {
    let input: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();
    let symbol_positions: Vec<Pos> = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (!(c.is_ascii_digit() || c == '.')).then(|| Pos { x, y }))
        })
        .collect();
    let width = input[0].len() - 1;
    let height = input.len() - 1;
    println!("Width: {}, Height: {}", width + 1, height + 1);
    let digit_positions: Vec<Pos> = symbol_positions
        .into_iter()
        .flat_map(|symbol| {
            [
                if symbol.x > width {
                    None
                } else {
                    Some(Pos {
                        x: symbol.x + 1,
                        y: symbol.y,
                    })
                },
                if symbol.y > height {
                    None
                } else {
                    Some(Pos {
                        x: symbol.x,
                        y: symbol.y + 1,
                    })
                },
                if symbol.x == 0 {
                    None
                } else {
                    Some(Pos {
                        x: symbol.x - 1,
                        y: symbol.y,
                    })
                },
                if symbol.y == 0 {
                    None
                } else {
                    Some(Pos {
                        x: symbol.x,
                        y: symbol.y - 1,
                    })
                },
                if symbol == (Pos { x: 0, y: 0 }) {
                    None
                } else {
                    Some(Pos {
                        x: symbol.x - 1,
                        y: symbol.y - 1,
                    })
                },
                if symbol
                    == (Pos {
                        x: width,
                        y: height,
                    })
                {
                    None
                } else {
                    Some(Pos {
                        x: symbol.x + 1,
                        y: symbol.y + 1,
                    })
                },
                if symbol == (Pos { x: width, y: 0 }) {
                    None
                } else {
                    Some(Pos {
                        x: symbol.x + 1,
                        y: symbol.y - 1,
                    })
                },
                if symbol == (Pos { x: 0, y: height }) {
                    None
                } else {
                    Some(Pos {
                        x: symbol.x - 1,
                        y: symbol.y + 1,
                    })
                },
            ]
        })
        .flatten()
        .collect();
    let input: Vec<Vec<char>> = input
        .iter()
        .map(|string| string.chars().collect())
        .collect();
    let mut blacklist = Vec::new();
    let sum = digit_positions
        .iter()
        .map(|Pos { x, y }| {
            if input[*y][*x].is_ascii_digit()
                && !blacklist
                    .iter()
                    .any(|(y0, r): &(usize, Range<usize>)| *y0 == *y && r.contains(&x))
            {
                let mut num = String::new();
                let mut offset: isize = 0;
                while input[*y][(*x as isize - offset) as usize].is_ascii_digit() {
                    offset += 1;
                    if offset > *x as isize {
                        break;
                    }
                }
                if offset > 0 {
                    offset -= 1;
                }
                let from = (*x as isize - offset) as usize;
                while input[*y][(*x as isize - offset) as usize].is_ascii_digit() {
                    num.push(input[*y][(*x as isize - offset) as usize]);
                    offset -= 1;
                    if offset > *x as isize || *x as isize - offset > width as isize {
                        break;
                    }
                }
                let to = (*x as isize - offset) as usize;
                if to > from {
                    blacklist.push((*y, from..to));
                } else {
                    blacklist.push((*y, to..from));
                }
                if *x > 100 && *y == 52 {
                    println!("{}", num);
                }
                num.parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .sum::<usize>();
    for Pos { x, y } in digit_positions.iter().filter(|Pos { x: _, y }| *y == 52) {
        println!("Pos{{x: {}, y: {}}}, char: {}", x, y, input[*y][*x]);
    }
    println!("Part1: {}", sum);
}

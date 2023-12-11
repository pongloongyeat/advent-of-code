struct Gear {
    pos_x: usize,
    pos_y: usize,
}

impl Gear {
    fn new(pos_x: usize, pos_y: usize) -> Self {
        Self { pos_x, pos_y }
    }

    fn adjacent_parts(&self, engine: &Vec<Vec<char>>) -> Vec<u32> {
        let mut parts = vec![];
        let engine_x_len = engine.first().unwrap().len();

        if self.pos_x != 0 {
            let left = engine[self.pos_y][self.pos_x - 1];
            if left.is_ascii_digit() {
                parts.push(seek(engine, self.pos_x - 1, self.pos_y));
            }
        }

        if self.pos_y != 0 {
            let pos_x = self.pos_x;
            let pos_y = self.pos_y - 1;

            let top = engine[pos_y][pos_x];
            if top.is_ascii_digit() {
                parts.push(seek(engine, pos_x, pos_y));
            } else {
                if pos_x != 0 {
                    let pos_x = pos_x - 1;
                    let bottom_left = engine[pos_y][pos_x];
                    if bottom_left.is_ascii_digit() {
                        parts.push(seek(engine, pos_x, pos_y));
                    }
                }

                if pos_x != engine_x_len - 1 {
                    let pos_x = pos_x + 1;
                    let bottom_right = engine[pos_y][pos_x];
                    if bottom_right.is_ascii_digit() {
                        parts.push(seek(engine, pos_x, pos_y));
                    }
                }
            }
        }

        if self.pos_x != engine_x_len - 1 {
            let right = engine[self.pos_y][self.pos_x + 1];
            if right.is_ascii_digit() {
                parts.push(seek(engine, self.pos_x + 1, self.pos_y));
            }
        }

        if self.pos_y != engine.len() - 1 {
            let pos_x = self.pos_x;
            let pos_y = self.pos_y + 1;

            let bottom = engine[pos_y][pos_x];
            if bottom.is_ascii_digit() {
                parts.push(seek(engine, pos_x, pos_y));
            } else {
                if pos_x != 0 {
                    let pos_x = pos_x - 1;
                    let bottom_left = engine[pos_y][pos_x];
                    if bottom_left.is_ascii_digit() {
                        parts.push(seek(engine, pos_x, pos_y));
                    }
                }

                if pos_x != engine_x_len - 1 {
                    let pos_x = pos_x + 1;
                    let bottom_right = engine[pos_y][pos_x];
                    if bottom_right.is_ascii_digit() {
                        parts.push(seek(engine, pos_x, pos_y));
                    }
                }
            }
        }

        parts
    }
}

fn seek(engine: &Vec<Vec<char>>, pos_x: usize, pos_y: usize) -> u32 {
    let line = &engine[pos_y];
    let mut start_index = pos_x;

    loop {
        if start_index == 0 {
            break;
        }

        let char = line[start_index - 1];
        if char.is_ascii_digit() {
            start_index -= 1;
        } else {
            break;
        }
    }

    let mut end_index = pos_x;

    loop {
        if end_index == engine.first().unwrap().len() - 1 {
            break;
        }

        let char = line[end_index + 1];
        if char.is_ascii_digit() {
            end_index += 1;
        } else {
            break;
        }
    }

    line[start_index..(end_index + 1)]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn process(input: &str) -> u32 {
    let engine = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    process_engine(&engine)
        .iter()
        .filter_map(|gear| {
            let adjacent_parts = gear.adjacent_parts(&engine);
            if adjacent_parts.len() == 2 {
                Some(adjacent_parts.iter().fold(1, |acc, x| acc * x))
            } else {
                None
            }
        })
        .sum()
}

fn process_engine(engine: &Vec<Vec<char>>) -> Vec<Gear> {
    let mut gears = vec![];

    for pos_y in 0..engine.len() {
        for pos_x in 0..engine.first().unwrap().len() {
            let char = engine[pos_y][pos_x];
            if char == '*' {
                gears.push(Gear::new(pos_x, pos_y));
            }
        }
    }

    gears
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn can_process_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(input);
        assert_eq!(result, 467835);
    }
}

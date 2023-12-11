#[derive(Debug)]
struct Part {
    number: String,
    pos_x: usize,
    pos_y: usize,
}

impl Part {
    fn new(number: String, pos_x: usize, pos_y: usize) -> Self {
        Self {
            number,
            pos_x,
            pos_y,
        }
    }

    fn is_valid(&self, engine: &Vec<Vec<char>>) -> bool {
        if self.pos_x != 0 {
            let left = engine[self.pos_y][self.pos_x - 1];
            if !left.is_ascii_digit() && left != '.' {
                return true;
            }

            if self.pos_y != 0 {
                let top_left = engine[self.pos_y - 1][self.pos_x - 1];
                if !top_left.is_ascii_digit() && top_left != '.' {
                    return true;
                }
            }

            if self.pos_y != engine.len() - 1 {
                let bottom_left = engine[self.pos_y + 1][self.pos_x - 1];
                if !bottom_left.is_ascii_digit() && bottom_left != '.' {
                    return true;
                }
            }
        }

        if self.pos_y != 0 {
            let top = &engine[self.pos_y - 1][self.pos_x..(self.pos_x + self.number.len())];
            if top.iter().any(|&top| !top.is_ascii_digit() && top != '.') {
                return true;
            }
        }

        if self.pos_x + self.number.len() != engine.first().unwrap().len() {
            let right = engine[self.pos_y][self.pos_x + self.number.len()];
            if !right.is_ascii_digit() && right != '.' {
                return true;
            }

            if self.pos_y != 0 {
                let top_right = engine[self.pos_y - 1][self.pos_x + self.number.len()];
                if !top_right.is_ascii_digit() && top_right != '.' {
                    return true;
                }
            }

            if self.pos_y != engine.len() - 1 {
                let bottom_right = engine[self.pos_y + 1][self.pos_x + self.number.len()];
                if !bottom_right.is_ascii_digit() && bottom_right != '.' {
                    return true;
                }
            }
        }

        if self.pos_y != engine.len() - 1 {
            let bottom = &engine[self.pos_y + 1][self.pos_x..(self.pos_x + self.number.len())];
            if bottom
                .iter()
                .any(|&bottom| !bottom.is_ascii_digit() && bottom != '.')
            {
                return true;
            }
        }

        return false;
    }
}

pub fn process(input: &str) -> u32 {
    let engine = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    process_engine(&engine)
        .iter()
        .filter_map(|part| {
            if part.is_valid(&engine) {
                Some(part.number.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

fn process_engine(engine: &Vec<Vec<char>>) -> Vec<Part> {
    let mut parts = vec![];

    for pos_y in 0..engine.len() {
        let mut pos_x = 0;
        let mut chars = vec![];
        let line = &engine[pos_y];

        loop {
            if pos_x == line.len() {
                break;
            }

            let char = line[pos_x];
            if char.is_ascii_digit() {
                chars.push(char);
            } else if !chars.is_empty() {
                // Save previous result and flush
                parts.push(Part::new(
                    chars.iter().collect(),
                    pos_x - chars.len(),
                    pos_y,
                ));
                chars.clear();
            }

            pos_x += 1;
        }

        // Last check for chars buffer
        if !chars.is_empty() {
            parts.push(Part::new(
                chars.iter().collect(),
                pos_x - chars.len(),
                pos_y,
            ));
        }
    }

    parts
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
        assert_eq!(result, 4361);
    }
}

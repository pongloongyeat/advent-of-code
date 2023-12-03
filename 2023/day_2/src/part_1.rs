#[derive(PartialEq, Debug, Clone, Copy)]
pub struct CubeCombination {
    pub reds: u32,
    pub greens: u32,
    pub blues: u32,
}

impl CubeCombination {
    pub fn new(reds: u32, greens: u32, blues: u32) -> CubeCombination {
        CubeCombination {
            reds,
            greens,
            blues,
        }
    }

    fn is_possible(&self, desired: CubeCombination) -> bool {
        self.reds <= desired.reds && self.greens <= desired.greens && self.blues <= desired.blues
    }
}

#[derive(PartialEq, Debug)]
pub struct Game {
    id: u32,
    pub combination: CubeCombination,
}

pub fn process(input: &str, desired: CubeCombination) -> u32 {
    input
        .lines()
        .map(process_line)
        .filter(|game| game.combination.is_possible(desired))
        .map(|game| game.id)
        .sum()
}

pub fn process_line(line: &str) -> Game {
    let (game, rolls) = line.split_once(": ").unwrap();
    let (_, id) = game.split_once(" ").unwrap();
    let id = id.parse::<u32>().unwrap();

    let mut max_reds = 0;
    let mut max_greens = 0;
    let mut max_blues = 0;

    let rolls = rolls.split("; ");

    for roll in rolls {
        let rolled_cubes = roll.split(", ");

        for rolled_cube in rolled_cubes {
            let (num, colour) = rolled_cube.split_once(" ").unwrap();
            let num = num.parse::<u32>().unwrap();
            if colour == "red" {
                if num > max_reds {
                    max_reds = num;
                }

                continue;
            }

            if colour == "green" {
                if num > max_greens {
                    max_greens = num;
                }

                continue;
            }

            if colour == "blue" {
                if num > max_blues {
                    max_blues = num;
                }

                continue;
            }
        }
    }

    Game {
        id,
        combination: CubeCombination {
            reds: max_reds,
            greens: max_greens,
            blues: max_blues,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_process_lines() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(
            input,
            CubeCombination {
                reds: 12,
                greens: 13,
                blues: 14,
            },
        );
        assert_eq!(result, 8);
    }

    #[test]
    fn can_process_line() {
        assert_eq!(
            process_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                combination: CubeCombination {
                    reds: 4,
                    greens: 2,
                    blues: 6
                }
            }
        );
        assert_eq!(
            process_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Game {
                id: 2,
                combination: CubeCombination {
                    reds: 1,
                    greens: 3,
                    blues: 4
                }
            }
        );
        assert_eq!(
            process_line(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            Game {
                id: 3,
                combination: CubeCombination {
                    reds: 20,
                    greens: 13,
                    blues: 6
                }
            }
        );
        assert_eq!(
            process_line(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            Game {
                id: 4,
                combination: CubeCombination {
                    reds: 14,
                    greens: 3,
                    blues: 15
                }
            }
        );
        assert_eq!(
            process_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Game {
                id: 5,
                combination: CubeCombination {
                    reds: 6,
                    greens: 3,
                    blues: 2
                }
            }
        );
    }
}

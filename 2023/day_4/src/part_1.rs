struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(winning_numbers: Vec<u32>, numbers: Vec<u32>) -> Self {
        Self {
            winning_numbers,
            numbers,
        }
    }

    fn points(&self) -> u32 {
        let mut matches = 0;

        for &number in &self.numbers {
            if self.winning_numbers.contains(&number) {
                matches += 1;
            }
        }

        Self::points_for_match(matches)
    }

    fn points_for_match(number_of_matches: u32) -> u32 {
        if number_of_matches == 0 {
            0
        } else {
            2_u32.pow(number_of_matches - 1)
        }
    }
}

pub fn process(input: &str) -> u32 {
    input.lines().map(|line| process_line(line).points()).sum()
}

fn process_line(line: &str) -> Card {
    let (_, numbers) = line.split_once(":").unwrap();
    let (winning_numbers, numbers) = numbers.split_once("|").unwrap();

    Card::new(
        winning_numbers
            .trim()
            .split(" ")
            .filter(|chars| !chars.is_empty())
            .map(|char| char.trim().parse::<u32>().unwrap())
            .collect(),
        numbers
            .trim()
            .split(" ")
            .filter(|chars| !chars.is_empty())
            .map(|char| char.trim().parse::<u32>().unwrap())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn can_process_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process(input);
        assert_eq!(result, 13);
    }
}

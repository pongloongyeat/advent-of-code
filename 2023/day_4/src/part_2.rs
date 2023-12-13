use std::collections::HashMap;

use crate::part_1::{process_line, Card};

pub fn process(input: &str) -> u32 {
    let cards = input.lines().map(process_line);
    let mut cards_map = HashMap::new();

    for card in cards {
        let id = card.id;

        // Process previous copies
        if let Some(&count) = cards_map.get(&id) {
            process_winnings(&card, &mut cards_map, count);
        }

        // Process original
        process_winnings(&card, &mut cards_map, 1);

        // Add current card to stack
        match cards_map.get(&id) {
            Some(&count) => cards_map.insert(id, count + 1),
            None => cards_map.insert(id, 1),
        };
    }

    cards_map.values().sum()
}

fn process_winnings(card: &Card, cards_map: &mut HashMap<u32, u32>, repetitions: u32) {
    let next_id = card.id + 1;
    let matches = card.matches();

    for id in next_id..next_id + matches {
        match cards_map.get(&id) {
            Some(&count) => cards_map.insert(id, count + repetitions),
            None => cards_map.insert(id, repetitions),
        };
    }
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

        assert_eq!(result, 30);
    }
}

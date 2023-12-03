pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(process_line)
        .map(|value| {
            let first = value.0;
            let last = value.1;
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

fn process_line(line: &str) -> (u32, u32) {
    let scan = line
        .chars()
        .enumerate()
        .filter_map(|(index, char)| {
            if char.is_numeric() {
                let num = (&line[index..(index + 1)]).parse::<u32>().unwrap();
                return Some(num);
            }

            let substring = &line[index..];
            if substring.starts_with("one") {
                Some(1)
            } else if substring.starts_with("two") {
                Some(2)
            } else if substring.starts_with("three") {
                Some(3)
            } else if substring.starts_with("four") {
                Some(4)
            } else if substring.starts_with("five") {
                Some(5)
            } else if substring.starts_with("six") {
                Some(6)
            } else if substring.starts_with("seven") {
                Some(7)
            } else if substring.starts_with("eight") {
                Some(8)
            } else if substring.starts_with("nine") {
                Some(9)
            } else {
                None
            }
        })
        .collect::<Vec<u32>>();

    if scan.len() == 1 {
        let &value = scan.first().unwrap();
        (value, value)
    } else {
        let &first = scan.first().unwrap();
        let &last = scan.last().unwrap();
        (first, last)
    }
}

#[cfg(test)]
mod test {

    use super::{process, process_line};

    #[test]
    fn can_process_input() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn can_process_line() {
        assert_eq!(process_line("eightwothree"), (8, 3));
        assert_eq!(process_line("abcone2threexyz"), (1, 3));
        assert_eq!(process_line("xtwone3four"), (2, 4));
        assert_eq!(process_line("4nineeightseven2"), (4, 2));
        assert_eq!(process_line("zoneight234"), (1, 4));
        assert_eq!(process_line("7pqrstsixteen"), (7, 6));
    }
}

pub fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|char| if char.is_numeric() { Some(char) } else { None })
                .collect::<String>()
        })
        .map(|line| {
            let mut it = line.chars();
            let first = it.next().unwrap();
            let last = it.last();

            match last {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<usize>()
            .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn can_process_input() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(input);
        assert_eq!(result, 142);
    }
}

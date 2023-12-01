use std::collections::HashMap;

fn main() {}

fn concat_digits(first: usize, second: usize) -> Option<usize> {
    format!("{first}{second}").parse::<usize>().ok()
}

fn challenge_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_string().parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|digits| !digits.is_empty())
        .filter_map(|digits| concat_digits(*digits.first().unwrap(), *digits.last().unwrap()))
        .sum()
}

fn challenge_2(input: &str) -> usize {
    let tokens_to_numbers: HashMap<&str, usize> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    input
        .lines()
        .map(|line| parse_digits(line, &tokens_to_numbers))
        .filter(|digits| !digits.is_empty())
        .filter_map(|digits| concat_digits(*digits.first().unwrap(), *digits.last().unwrap()))
        .sum()
}

fn parse_digits(line: &str, tokens_to_digits: &HashMap<&str, usize>) -> Vec<usize> {
    (0..line.len()).fold(vec![], |mut acc, start| {
        let numbers = (start..line.len())
            .map(|end| &line[start..=end])
            .filter_map(|slice| tokens_to_digits.get(slice));

        acc.extend(numbers);
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(challenge_1(INPUT), 142);
    }

    #[test]
    fn solves_challenge_1() {
        const INPUT: &str = include_str!("challenge_1_input.txt");

        assert_eq!(challenge_1(INPUT), 54605);
    }

    #[test]
    fn example_2() {
        const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(challenge_2(INPUT), 281);
    }

    #[test]
    fn solves_challenge_2() {
        const INPUT: &str = include_str!("challenge_1_input.txt");
        assert_eq!(challenge_2(INPUT), 55429);
    }
}

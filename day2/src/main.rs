use std::collections::HashMap;

fn main() {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = ();

    fn try_from(color: &str) -> Result<Self, Self::Error> {
        match color {
            "blue" => Ok(Self::Blue),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<HashMap<Color, usize>>,
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.split(':'))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|(id_part, draw_part)| {
            let id = id_part.replace("Game ", "").parse::<usize>().unwrap();
            let draws = draw_part
                .split(';')
                .map(|draw| {
                    draw.split(", ")
                        .map(|draw| draw.split_whitespace())
                        .map(|mut split| {
                            let amount = split.next().unwrap().parse::<usize>().unwrap();
                            let color = split.next().unwrap().try_into().unwrap();

                            (color, amount)
                        })
                        .collect()
                })
                .collect();

            Game { id, draws }
        })
        .collect()
}

fn challenge_1(input: &str, constrains: &HashMap<Color, usize>) -> usize {
    parse_input(input)
        .iter()
        .filter(|game| {
            game.draws.iter().flatten().all(|(color, amount)| {
                constrains
                    .get(color)
                    .map_or(true, |constraint| amount <= constraint)
            })
        })
        .map(|game| game.id)
        .sum()
}

fn challenge_2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|game| {
            game.draws
                .iter()
                .flatten()
                .fold(HashMap::new(), |mut constraints, (&color, &amount)| {
                    if constraints
                        .get(&color)
                        .map_or(true, |&current_constraint| current_constraint < amount)
                    {
                        constraints.insert(color, amount);
                    }
                    constraints
                })
                .iter()
                .map(|(_, &amount)| amount)
                .product::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let constrains = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

        assert_eq!(challenge_1(INPUT, &constrains), 8);
    }

    #[test]
    fn solves_challenge_1() {
        const INPUT: &str = include_str!("input.txt");
        let constrains = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

        assert_eq!(challenge_1(INPUT, &constrains), 2239);
    }

    #[test]
    fn example_2() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(challenge_2(INPUT), 2286);
    }

    #[test]
    fn solves_challenge_2() {
        const INPUT: &str = include_str!("input.txt");
        assert_eq!(challenge_2(INPUT), 83435);
    }
}

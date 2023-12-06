const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

const INPUT: &str = include_str!("input.txt");

fn main() {}

#[derive(Debug)]
struct Race(usize, usize);

impl Race {
    const fn time(&self) -> usize {
        self.0
    }

    const fn distance(&self) -> usize {
        self.1
    }

    const fn simulate(&self, button_pressed_for: usize) -> usize {
        const ACCELERATION: usize = 1;

        let driving_duration = self.time() - button_pressed_for;
        let distance_per_ms = button_pressed_for * ACCELERATION;

        driving_duration * distance_per_ms
    }
}

fn challenge_1(input: &str) -> usize {
    let mut lines = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .filter_map(|number| number.parse::<usize>().ok())
            .collect::<Vec<_>>()
    });

    let races: Vec<Race> = lines
        .next()
        .unwrap()
        .iter()
        .zip(lines.next().unwrap())
        .map(|(&time, distance)| Race(time, distance))
        .collect();

    races
        .iter()
        .map(|race| {
            (0..=race.time())
                .filter(|&ms| race.simulate(ms) > race.distance())
                .count()
        })
        .product()
}

fn challenge_2(input: &str) -> usize {
    let mut numbers = input.lines().filter_map(|line| {
        line.split_whitespace()
            .skip(1)
            .filter(|&char| char != " ")
            .collect::<String>()
            .parse()
            .ok()
    });

    let race = Race(numbers.next().unwrap(), numbers.next().unwrap());

    (0..=(race.time()))
        .filter(|&ms| race.simulate(ms) > race.distance())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solves_example_1() {
        assert_eq!(challenge_1(EXAMPLE), 288);
    }

    #[test]
    fn solves_challenge_1() {
        assert_eq!(challenge_1(INPUT), 2_449_062);
    }

    #[test]
    fn solves_example_2() {
        assert_eq!(challenge_2(EXAMPLE), 71503);
    }

    #[test]
    fn solves_challenge_2() {
        assert_eq!(challenge_2(INPUT), 33_149_631);
    }
}

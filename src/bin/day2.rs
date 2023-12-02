use std::cmp::max;

fn main() {}

#[derive(Debug)]
struct GameRound {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl GameRound {
    fn parse(input: &str) -> Self {
        let mut game_round = Self { red: 0, green: 0, blue: 0 };

        for c in input.split(",") {
            let (number, color) = c.trim().split_once(" ").expect("Failed to split input");
            let number = number.parse::<u32>().expect("Failed to parse number");

            match color {
                "red" => game_round.red = number,
                "blue" => game_round.blue = number,
                "green" => game_round.green = number,
                _ => panic!("Failed to parse game")
            }
        }

        game_round
    }
}

fn parse_game(input: &str) -> Vec<Vec<GameRound>> {
    let lines = input.lines();
    let lines = lines.map(|l| l.split_once(":").unwrap().1);
    let lines = lines.map(|s| { s.split(";") });
    let rounds = lines.map(|l| l.map(|g| GameRound::parse(g)).collect::<Vec<_>>()).collect::<Vec<_>>();
    rounds
}

fn part1(input: &str) -> u32 {
    parse_game(input)
        .iter()
        .enumerate()
        .filter_map(|(id, rounds)| {
            if rounds.iter().all(|round| round.green <= 13 && round.red <= 12 && round.blue <= 14) {
                Some((id + 1) as u32)
            } else {
                None
            }
        }).sum()
}

fn part2(input: &str) -> u32 {
    parse_game(input)
        .iter()
        .map(|rounds| {
            rounds
                .iter()
                .fold(GameRound { blue: 0, red: 0, green: 0 }, |mut tmp_game, g| {
                    tmp_game.green = max(g.green, tmp_game.green);
                    tmp_game.red = max(g.red, tmp_game.red);
                    tmp_game.blue = max(g.blue, tmp_game.blue);
                    tmp_game
                })
        })
        .map(|game_round| game_round.blue * game_round.red * game_round.green)
        .sum()
}


#[cfg(test)]
mod test {
    use crate::{parse_game, part1, part2};

    const EXAMPLE_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_parser() {
        let games = parse_game(EXAMPLE_INPUT);

        assert_eq!(games.len(), 5);
    }

    #[test]
    fn day2_part1_test() {
        // let input = include_str!("../../inputs/day2_input.txt");
        let answer = part1(EXAMPLE_INPUT);

        assert_eq!(8, answer);
    }

    #[test]
    fn day2_part2_test() {
        // let _input = include_str!("../../inputs/day2_input.txt");
        let answer = part2(EXAMPLE_INPUT);

        assert_eq!(2286, answer);
    }
}
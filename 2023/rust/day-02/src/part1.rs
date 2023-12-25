use crate::custom_error::AocError;
#[derive(Debug)]
pub struct Round {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
pub struct Game {
    pub game_number: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    fn is_valid_game(&self, sample_game: &Bag) -> bool {
        let mut valid_game = false;
        let mut valid_round: Vec<bool> = Vec::new();
        for round in &self.rounds {
            if sample_game.red >= round.red
                && sample_game.green >= round.green
                && sample_game.blue >= round.blue
            {valid_round.push(true);
            } else 
            { 
                valid_round.push(false);
            }
        } 
        if valid_round.iter().all(|&x| x == true) {
            valid_game = true;
        }
        valid_game
    }
}

#[derive(Debug)]
pub struct Bag {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let games = parse_games(input); // take file with game data and parse it into a vector
    let test_bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    // get the valid games that could be played with the test
    // bag
    let valid_games: Vec<u32> = get_valid_games(&games, test_bag);
    let sum_games: u32 = valid_games.iter().sum();
    Ok(sum_games.to_string())
}

pub fn parse_games(input: &str) -> Vec<Game> {
    let lines = input.lines();
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        let (game_num, value) = line.split_once(":").unwrap();
        let game_num = game_num.trim().split_once(" ").unwrap().1;
        let value = value.trim();
        let mut game = Game {
            game_number: game_num.parse::<u32>().unwrap(),
            rounds: Vec::new(),
        };
        for round in value.split(";") {
            let mut r = Round {
                red: 0,
                green: 0,
                blue: 0,
            };
            for count_of_color in round.split(",") {
                let count: u8 = count_of_color
                    .trim()
                    .split_once(" ")
                    .unwrap()
                    .0
                    .parse::<u8>()
                    .unwrap();
                let color = count_of_color.trim().split_once(" ").unwrap().1;
                match color.to_lowercase().as_str() {
                    "red" => r.red = count,
                    "blue" => r.blue = count,
                    "green" => r.green = count,
                    _ => panic!("unable to match {color}"),
                };
            }
            game.rounds.push(r);
        }
        games.push(game)
    }
    games
}

fn get_valid_games(games: &[Game], bag: Bag) -> Vec<u32> {
    let mut valid_games: Vec<u32> = Vec::new();
    for game in games {
        if game.is_valid_game(&bag) {
            valid_games.push(game.game_number);
        }
    }
    valid_games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}

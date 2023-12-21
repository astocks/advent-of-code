use crate::custom_error::AocError;

#[derive(Debug)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Game {
    game_number: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid_game(&self, sample_game: &Game) -> bool {
        let mut valid = true;
        todo!("implement is_valid_game");
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    parse_games(input); // take file with game data and parse it into a
                        // vector of games

    Ok("this still isn't finished".to_string())
}

fn parse_games(input: &str) -> Vec<Game> {
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
            let mut r = Round { red: 0, green: 0, blue: 0};
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
    dbg!(&games);
    games
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

use std::collections::HashMap;

use crate::custom_error::AocError;

struct Game {
    game_num: u32,
    colors: HashMap<String, u32>,
}

impl Game {
    fn new(game_num: u32) -> Self {
        Self {
            game_num,
            colors: HashMap::new(),
        }
    }

    fn add_round(&mut self, round: &str) {
        for count_of_color in round.split(",") {
            let count = count_of_color.trim().split_once(" ").unwrap().0;
            let color = count_of_color.trim().split_once(" ").unwrap().1;
            dbg!(count, color);
        }
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
   //this function should split the input into lines, then split on :, and return the parts. 
        let lines = input.lines();
        let mut games = HashMap::new();
        for line in lines {
            let (game_num, value) = line.split_once(":").unwrap();
            let game_num = game_num.trim().split_once(" ").unwrap().1;
            let value = value.trim();
            for round in value.split(";") {
                for count_of_color in round.split(",") {
                    let count = count_of_color.trim().split_once(" ").unwrap().0;
                    let color = count_of_color.trim().split_once(" ").unwrap().1;
                    dbg!(count, color);
                }
            } 
            
            dbg!(game_num, value);
        }
        
    Ok("make it pass".to_string())

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
        assert_eq!("", process(input)?);
        Ok(())
    }
}

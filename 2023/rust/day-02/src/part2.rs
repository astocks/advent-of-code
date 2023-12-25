use crate::custom_error::AocError;
use crate::part1::{parse_games, Bag, Game};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let games = parse_games(_input);
    let max_bags = get_max_bags_for_games(&games);
    let mut power: Vec<u32> = Vec::new();
    for bag in &max_bags {
        power.push(get_power(bag));
    }
    let answer: u32 = power.iter().sum();
    Ok(answer.to_string())
}

fn get_power(sample_bag: &Bag) -> u32 {
    sample_bag.red as u32 * sample_bag.green as u32 * sample_bag.blue as u32
}

fn get_max_bags_for_games(games: &Vec<Game>) -> Vec<Bag>{
    let mut max_bags: Vec<Bag> = Vec::new();
    for game in games {
        let mut max_bag = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };
        for round in &game.rounds {
            if round.red > max_bag.red {
                max_bag.red = round.red;
            }
            if round.green > max_bag.green {
                max_bag.green = round.green;
            }
            if round.blue > max_bag.blue {
                max_bag.blue = round.blue;
            }
        }
        max_bags.push(max_bag);
    }
    max_bags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}

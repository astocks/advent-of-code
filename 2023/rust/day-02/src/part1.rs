
use crate::custom_error::AocError;


#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}
#[derive(Debug)]
struct Cube(Color, u32);

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

#[derive(Debug)]
struct Game {
    game_number: u32,
    rounds: Vec<Round>,
}


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
   //this function should split the input into lines, then split on :, and return the parts. 
        let lines = input.lines();
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
                    cubes: Vec::new(),
                };
                for count_of_color in round.split(",") {
                    let count = count_of_color.trim().split_once(" ").unwrap().0.parse::<u32>().unwrap();
                    let color = count_of_color.trim().split_once(" ").unwrap().1;
                    let cube = match color.to_lowercase().as_str() {
                        "red" => Cube(Color::Red, count),
                        "blue" => Cube(Color::Blue, count),
                        "green" => Cube(Color::Green, count),
                        _ => panic!("unable to match {color}"),
                    };
                    r.cubes.push(cube);
                }
                game.rounds.push(r);
            } 
            
            dbg!(game);
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

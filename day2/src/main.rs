use std::fs;

fn main() {
    let raw_text = fs::read_to_string("./input.txt").unwrap();
    let result = sum_valid_games(raw_text, 12, 13, 14);
    println!("Result: {result}")
}

struct Game {
    pub id: i32,
    pub draws: Vec<(i32, i32, i32)>
}

fn parse_game(game: &str) -> Game {
    // example: Game 1: 10 green, 9 blue, 1 red; 1 red, 7 green; 11 green, 6 blue; 8 blue, 12 green
    let mut game_parts = game.split(": ");
    let id_raw = game_parts.next().unwrap();
    let id = id_raw.replace("Game ", "").parse::<i32>().unwrap();
    let draws_raw = game_parts.next().unwrap();
    let draws_parts = draws_raw.split("; ");
    let mut draws: Vec<(i32, i32, i32)> = vec!();
    for pieces_raw in draws_parts {
        let pieces_parts = pieces_raw.split(", ");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for piece_raw in pieces_parts {
            let mut piece_parts = piece_raw.split(' ');
            let quantity = piece_parts.next().unwrap().parse::<i32>().unwrap();
            let color = piece_parts.next().unwrap();
            match color {
                "red" => red = quantity,
                "green" => green = quantity,
                "blue" => blue = quantity,
                _ => panic!("color {color} not valid")
            }
        }
        draws.push((red, green, blue));
    }
    return Game {
        id,
        draws
    }
}

fn is_game_valid(game: &Game, max_red: i32, max_green: i32, max_blue: i32) -> bool {
    for (red, green, blue) in game.draws.clone() {
        if red > max_red || green > max_green || blue > max_blue {
            return false
        }
    }
    true
}

fn sum_valid_games(game_text: String, max_red: i32, max_green: i32, max_blue: i32) -> i32 {
    let games: Vec<&str> = game_text.lines().collect();
    let mut result = 0;
    for game_text in games {
        let game = parse_game(game_text);
        if is_game_valid(&game, max_red, max_green, max_blue) {
            result += &game.id;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{is_game_valid, parse_game, sum_valid_games};

    #[test]
    fn basic_test() {
        let parsed_game = parse_game("Game 1: 10 green, 9 blue, 1 red; 1 red, 7 green; 11 green, 6 blue; 8 blue, 12 green");
        assert_eq!(1, parsed_game.id);
        assert_eq!(vec!((1, 10, 9), (1, 7, 0), (0, 11, 6), (0, 12, 8)), parsed_game.draws);
        assert_eq!(is_game_valid(&parsed_game, 12, 13, 14), true);
        let example_input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let sum = sum_valid_games(example_input, 12, 13, 14);
        assert_eq!(sum, 8);
    }
}
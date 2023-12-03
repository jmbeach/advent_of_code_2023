fn main() {
    println!("Hello, world!");
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

#[cfg(test)]
mod tests {
    use crate::parse_game;

    #[test]
    fn basic_test() {
        let parsed_game = parse_game("Game 1: 10 green, 9 blue, 1 red; 1 red, 7 green; 11 green, 6 blue; 8 blue, 12 green");
        assert_eq!(1, parsed_game.id);
        assert_eq!(vec!((1, 10, 9), (1, 7, 0), (0, 11, 6), (0, 12, 8)), parsed_game.draws);
    }
}
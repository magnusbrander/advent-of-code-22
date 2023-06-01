use util;

fn main() {
    let lines = util::read_file_lines();

    // Naive solution.
    let naive_points_per_game = get_naive_points_per_game(&lines);
    let naive_total_points: i32 = util::sum_elements( &naive_points_per_game, 0);
    println!("Naive total points {}", naive_total_points);

    // Correct solution.
    let correct_points_per_game = get_correct_points_per_game(&lines);
    let correct_total_points: i32 = correct_points_per_game.iter().sum();
    println!("Correct total points {}", correct_total_points);
}

enum GameShape {
    Rock,
    Paper,
    Scissors,
}

struct GameHands {
    me: GameShape,
    opponent: GameShape,
}

enum GameResult {
    Win,
    Draw,
    Loss,
}

fn get_naive_points_per_game(lines: &Vec<String>) -> Vec<i32> {
    let game_hands = get_naive_game_hands(lines);
    return get_game_points_by_hands(game_hands);
}

fn get_correct_points_per_game(lines: &Vec<String>) -> Vec<i32> {
    let game_hands = get_correct_game_hands(lines);
    return get_game_points_by_hands(game_hands);
}

fn get_game_points_by_hands(game_hands: Vec<GameHands>) -> Vec<i32> {
    let mut points_per_game: Vec<i32> = Vec::new();
    for game_hand in game_hands {
        let mut game_point = 0;
        match game_hand.me {
            GameShape::Rock => {
                match game_hand.opponent {
                    GameShape::Rock => {
                        game_point += 3;
                    }
                    GameShape::Scissors => {
                        game_point += 6;
                    }
                    GameShape::Paper => {}
                }
                game_point += 1;
            }
            GameShape::Paper => {
                match game_hand.opponent {
                    GameShape::Rock => {
                        game_point += 6;
                    }
                    GameShape::Paper => {
                        game_point += 3;
                    }
                    GameShape::Scissors => {}
                }
                game_point += 2;
            }
            GameShape::Scissors => {
                match game_hand.opponent {
                    GameShape::Rock => {}
                    GameShape::Paper => {
                        game_point += 6;
                    }
                    GameShape::Scissors => {
                        game_point += 3;
                    }
                }
                game_point += 3;
            }
        }
        points_per_game.push(game_point);
    }

    return points_per_game;
}

fn get_naive_game_hands(lines: &Vec<String>) -> Vec<GameHands> {
    let mut all_game_hands: Vec<GameHands> = Vec::new();
    for line in lines {
        let shapes: Vec<&str> = line.split(' ').collect();
        let opponent_shape = get_opponent_hand_by_value(shapes[0]);
        let my_shape = get_my_naive_game_shape(shapes[1]);
        let game_hands = GameHands {
            me: my_shape,
            opponent: opponent_shape,
        };
        all_game_hands.push(game_hands);
    }
    return all_game_hands;
}

fn get_correct_game_hands(lines: &Vec<String>) -> Vec<GameHands> {
    let mut all_game_hands: Vec<GameHands> = Vec::new();
    for line in lines {
        let values: Vec<&str> = line.split(' ').collect();
        let opponent_hand = get_opponent_hand_by_value(values[0]);
        let game_result = get_game_result_by_encrypted_value(values[1]);
        let my_hand = get_my_correct_hand_by_game_result(&game_result, &opponent_hand);
        let game_hands = GameHands {
            me: my_hand,
            opponent: opponent_hand,
        };
        all_game_hands.push(game_hands);
    }
    return all_game_hands;
}

fn get_opponent_hand_by_value(shape: &str) -> GameShape {
    if shape == "A" {
        return GameShape::Rock;
    }
    if shape == "B" {
        return GameShape::Paper;
    }
    return GameShape::Scissors;
}

fn get_my_naive_game_shape(shape: &str) -> GameShape {
    if shape == "X" {
        return GameShape::Rock;
    }
    if shape == "Y" {
        return GameShape::Paper;
    }
    return GameShape::Scissors;
}

fn get_game_result_by_encrypted_value(value: &str) -> GameResult {
    if value == "X" {
        return GameResult::Loss;
    }
    if value == "Y" {
        return GameResult::Draw;
    }
    return GameResult::Win;
}

fn get_my_correct_hand_by_game_result(
    game_result: &GameResult,
    opponent_hand: &GameShape,
) -> GameShape {
    match game_result {
        GameResult::Loss => match opponent_hand {
            GameShape::Rock => {
                return GameShape::Scissors;
            }
            GameShape::Paper => {
                return GameShape::Rock;
            }
            GameShape::Scissors => {
                return GameShape::Paper;
            }
        },
        GameResult::Draw => match opponent_hand {
            GameShape::Rock => {
                return GameShape::Rock;
            }
            GameShape::Paper => {
                return GameShape::Paper;
            }
            GameShape::Scissors => {
                return GameShape::Scissors;
            }
        },
        GameResult::Win => match opponent_hand {
            GameShape::Rock => {
                return GameShape::Paper;
            }
            GameShape::Paper => {
                return GameShape::Scissors;
            }
            GameShape::Scissors => {
                return GameShape::Rock;
            }
        },
    }
}

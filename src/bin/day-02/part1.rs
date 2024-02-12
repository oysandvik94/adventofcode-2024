fn main() {
    let input = include_str!("./testinput.txt");
    let answer = part1(input.trim());
    dbg!(answer);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|game| -> u32 { analyze_game(game) })
        .sum()
}

struct Color {
    color: String,
    max: u32,
}
fn analyze_game(game: &str) -> u32 {
    let colors = define_colors();

    let (game_id, game) = game.split_once(':').expect("Two parts of the game");

    match game
        .split(';')
        .any(|round| any_color_break_rules(&colors, round))
    {
        false => get_game_id(game_id),
        true => 0,
    }
}

fn get_game_id(game: &str) -> u32 {
    game
        .replace("Game", "")
        .trim()
        .parse::<u32>()
        .expect("Should be game ID")
}
fn any_color_break_rules(colors: &[Color; 3], round: &str) -> bool {
    colors
        .iter()
        .any(|color| count_color(round, &color.color as &str) > color.max)
}

fn define_colors() -> [Color; 3] {
    let colors: [Color; 3] = [
        Color {
            color: String::from("red"),
            max: 12,
        },
        Color {
            color: String::from("green"),
            max: 13,
        },
        Color {
            color: String::from("blue"),
            max: 14,
        },
    ];
    colors
}

fn count_color(game_round: &str, target_color: &str) -> u32 {
    let found_color: Option<&str> = game_round
        .split(',')
        .find(|color: &&str| color.contains(target_color));

    match found_color {
        Some(found_color) => found_color
            .replace(target_color, "")
            .trim()
            .parse::<u32>()
            .expect("Should be a digit"),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::{analyze_game, count_color, part1};
    use rstest::rstest;

    #[test]
    fn part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let output = part1(input);

        assert_eq!(output, 8);
    }

    #[rstest]
    #[case("3 blue, 4 red", "blue", 3)]
    #[case("3 green, 4 blue, 1 red", "green", 3)]
    #[case("8 green, 6 blue, 20 red", "red", 20)]
    #[case("1 green, 3 red, 6 blue", "green", 1)]
    #[case("6 red, 1 blue, 3 green", "red", 6)]
    fn count_color_test(#[case] input: &str, #[case] color: &str, #[case] expected: u32) {
        assert_eq!(expected, count_color(input, color));
    }

    #[test]
    fn test_analyze_game() {
        assert_eq!(
            analyze_game(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            0
        );
        assert_eq!(
            analyze_game("Game 14: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 8 blue, 9 red"),
            14
        );
        assert_eq!(
            analyze_game(
                "Game 100: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 8 blue, 9 red"
            ),
            100
        );
    }
}

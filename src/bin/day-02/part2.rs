fn main() {
    let input = include_str!("./testinput.txt");
    let answer = part2(input.trim());
    dbg!(answer);
}

fn part2(input: &str) -> u32 {
    input.lines().map(analyze_game).sum()
}

fn analyze_game(game: &str) -> u32 {
    const COLORS: [&str; 3] = ["red", "green", "blue"];
    let game = game.split(':').last().expect("Actual game");

    COLORS
        .iter()
        .map(|color| {
            game.split(';')
                .map(|round| -> u32 { count_color(round, color) })
                .max()
                .expect("Should get max color")
        })
        .product::<u32>()
}

fn count_color(line: &str, target_color: &str) -> u32 {
    let color_count: Option<&str> = line
        .split(',')
        .find(|color: &&str| color.contains(target_color));

    match color_count {
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
    use crate::{count_color, part2};
    use rstest::rstest;

    #[test]
    fn part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let output = part2(input);

        assert_eq!(output, 2286);
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
}

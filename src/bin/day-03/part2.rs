use std::{char, collections::HashSet};

fn main() {
    let input = include_str!("./testinput.txt");
    let answer = part1(input);
    dbg!(answer);
}

trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        let not_numeric = !&self.is_numeric();
        let not_dot: bool = self != &'.';

        not_numeric && not_dot && !&self.is_whitespace()
    }
}

fn part1(input: &str) -> i32 {
    solve_input(input.to_string())
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}
fn generate_boundary(point: Point) -> Vec<Point> {
    Vec::from([
        Point {
            x: point.x - 1,
            y: point.y,
        },
        Point {
            x: point.x - 1,
            y: point.y - 1,
        },
        Point {
            x: point.x + 1,
            y: point.y - 1,
        },
        Point {
            x: point.x,
            y: point.y - 1,
        },
        Point {
            x: point.x,
            y: point.y + 1,
        },
        Point {
            x: point.x - 1,
            y: point.y + 1,
        },
        Point {
            x: point.x + 1,
            y: point.y + 1,
        },
        Point {
            x: point.x + 1,
            y: point.y,
        },
    ])
}
#[derive(Debug)]
struct PartNumber {
    number: i32,
    points: HashSet<Point>,
}

trait PartNumberProduct {
    fn product(&self) -> i32;
}

impl<T> PartNumberProduct for Vec<T>
where
    T: AsRef<PartNumber>,
{
    fn product(&self) -> i32 {
        println!("product");
        self.iter()
            .map(|part| part.as_ref().number)
            .inspect(|f| println!("{f}"))
            .reduce(|acc, number| acc * number)
            .expect("should result in number")
    }
}

impl AsRef<PartNumber> for PartNumber {
    fn as_ref(&self) -> &PartNumber {
        self
    }
}

#[derive(Debug)]
struct Gear {
    points_around_gear: HashSet<Point>,
}

impl Gear {
    fn new(index: Point) -> Gear {
        Gear {
            points_around_gear: generate_boundary(index).into_iter().collect(),
        }
    }
}

fn solve_input(input: String) -> i32 {
    let gears: Vec<Gear> = find_gears(&input);
    let part_numbers: Vec<PartNumber> = find_parts(input);

    dbg!(&gears);
    dbg!(&part_numbers);
    gears
        .iter()
        .map(|gear| {
            let matches: Vec<&PartNumber> = part_numbers
                .iter()
                .filter(|part| does_gear_and_part_intersect(gear, part))
                .collect::<Vec<&PartNumber>>();

            match matches.len() {
                2 => matches.product(),
                _ => 0,
            }
        })
        .sum::<i32>()
}

fn does_gear_and_part_intersect(gear: &Gear, part_number: &PartNumber) -> bool {
    points_intersect(&gear.points_around_gear, &part_number.points)
}

fn points_intersect(first_points: &HashSet<Point>, second_points: &HashSet<Point>) -> bool {
    first_points.intersection(second_points).next().is_some()
}

fn find_parts(input: String) -> Vec<PartNumber> {
    let part_numbers: Vec<PartNumber> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut current_string = String::new();
            let mut number_points = HashSet::new();

            line.chars()
                .enumerate()
                .filter_map(move |(x, character)| match character {
                    c if c.is_ascii_digit() => {
                        println!("string is {current_string}");
                        println!("pushing {character}");
                        current_string.push(character);
                        number_points.insert(Point {
                            x: x as i32,
                            y: y as i32,
                        });

                        let last_character_on_line = line.len() == x + 1;
                        if last_character_on_line {
                            return resolve_number_from_string(
                                &mut current_string,
                                &mut number_points,
                            );
                        }
                        None
                    }
                    _ => {
                        println!("completing number {current_string}");
                        resolve_number_from_string(&mut current_string, &mut number_points)
                    }
                })
        })
        .collect();
    part_numbers
}

fn resolve_number_from_string(
    current_string: &mut String,
    number_points: &mut HashSet<Point>,
) -> Option<PartNumber> {
    if let Ok(parsed_number) = current_string.parse::<i32>() {
        current_string.clear();
        let cloned_points = number_points.clone();
        number_points.clear();
        Some(PartNumber {
            number: parsed_number,
            points: cloned_points,
        })
    } else {
        None
    }
}

fn find_gears(input: &str) -> Vec<Gear> {
    let gears: Vec<Gear> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, character)| match character {
                    '*' => Some(Gear::new(Point {
                        x: x as i32,
                        y: y as i32,
                    })),
                    _ => None,
                })
        })
        .collect();
    gears
}

#[cfg(test)]
mod tests {
    use crate::part1;

    // #[test]
    fn part_one() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let output = part1(input);

        assert_eq!(output, 467835);
    }

    #[test]
    fn ny_test_case() {
        let input = "......
......
120.78
...*..
......
......
";
        assert_eq!(9360, part1(input));
    }
}

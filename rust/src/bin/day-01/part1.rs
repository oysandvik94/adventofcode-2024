fn main() {
    let input = include_str!("./testinput.txt");
    let answer = part1(input);
    dbg!(answer);
}

fn part1(input: &str) -> u32 {
    let parsed_numbers: Vec<Result<u32, _>> = input
        .lines()
        .map(|line| {
            let mut vec = Vec::new();
            for c in line.chars() {
                if c.is_numeric() {
                    vec.push(c);
                }
            }

            if vec.first().is_some() && vec.last().is_some() {
                // println!("adding {} and {}", vec.first().unwrap(), vec.last().unwrap());
                let number_string: String =
                    format!("{}{}", vec.first().unwrap(), vec.last().unwrap());
                let my_number: Result<u32, _> = number_string.parse();
                return my_number;
            }

            return Ok(0b0);
        })
        .collect();

    // Now you have a vector of Results, each containing either a parsed u32 or an error message
    let mut result: u32 = 0;
    for parsed_result in parsed_numbers {
        match parsed_result {
            Ok(parsed) => result += parsed,
            Err(e) => println!("Error parsing: {}", e),
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part_one() {
        let input = "
            1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
            ";

        let output = part1(input);

        assert_eq!(output, 142);
    }
}

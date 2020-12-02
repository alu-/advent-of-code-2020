use regex::Regex;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^(\d+)-(\d+) (\w): (\w*)$").unwrap();
        let captures = pattern.captures(s).unwrap();

        Ok(Password {
            min: captures.get(1).unwrap().as_str().parse().unwrap(),
            max: captures.get(2).unwrap().as_str().parse().unwrap(),
            character: captures.get(3).unwrap().as_str().parse().unwrap(),
            password: captures.get(4).unwrap().as_str().parse().unwrap(),
        })
    }
}

pub fn part1(input: &str) -> Result<usize, String> {
    Ok(input
        .lines()
        .map(|x| x.parse::<Password>().unwrap())
        .filter(|pw| {
            let m = pw.password.chars().filter(|x| *x == pw.character).count();

            return if m < pw.min {
                false
            } else if m > pw.max {
                false
            } else {
                true
            };
        })
        .count())
}

pub fn part2(input: &str) -> Result<usize, String> {
    Ok(input
        .lines()
        .map(|x| x.parse::<Password>().unwrap())
        .filter(|pw| {
            let chars: Vec<char> = pw.password.chars().collect();
            (chars[pw.min - 1] == pw.character) ^ (chars[pw.max - 1] == pw.character)
        })
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 1);
    }
}

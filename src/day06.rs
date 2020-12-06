use itertools::Itertools;

pub fn part1(input: &str) -> Result<usize, String> {
    Ok(input
        .split("\n\n")
        .map(|x| x.replace("\n", "").chars().unique().count())
        .sum())
}

pub fn part2(input: &str) -> Result<usize, String> {
    Ok(input.split("\n\n").fold(0, |acc, x| {
        let answers = x.replace("\n", "");
        let people = x.matches("\n").count() + 1;
        let chars = answers.chars();

        let mut count = 0;
        for char in chars.unique() {
            if x.matches(char).count() == people {
                count += 1;
            }
        }

        acc + count
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 6);
    }
}

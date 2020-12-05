pub fn part1(input: &str) -> Result<usize, String> {
    let lines: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();

    let answer: Vec<&usize> = lines
        .iter()
        .filter(|x| {
            let m = 2020 - *x;
            lines.contains(&m)
        })
        .collect();

    Ok(answer[0] * answer[1])
}

pub fn part2(input: &str) -> Result<i32, String> {
    let lines: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut answer: i32 = 0;
    'outer: for x in &lines {
        for y in &lines {
            let sum = 2020 - x - y;
            if lines.contains(&sum) {
                answer = sum * x * y;
                break 'outer;
            }
        }
    }

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 241861950);
    }
}

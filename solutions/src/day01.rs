pub fn part1(input: &str) -> String {
    let mut answer = String::new();

    'outer_loop: for x in input.lines() {
        for y in input.lines() {
            let sum = x.parse::<i32>().unwrap() + y.parse::<i32>().unwrap();
            if sum == 2020 {
                answer = (x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()).to_string();
                break 'outer_loop;
            }
        }
    }

    return answer;
}

pub fn part2(input: &str) -> String {
    let mut answer = String::new();

    'outer_loop: for x in input.lines() {
        for y in input.lines() {
            for z in input.lines() {
                let sum = x.parse::<i32>().unwrap() + y.parse::<i32>().unwrap() + z.parse::<i32>().unwrap();
                if sum == 2020 {
                    answer = (x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap() * z.parse::<i32>().unwrap()).to_string();
                    break 'outer_loop;
                }
            }
        }
    }

    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "514579");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "241861950");
    }
}

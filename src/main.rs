mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    const INPUT_1: &str = include_str!("../inputs/input1.txt");
    println!("Day 1 - part 1: {}", day01::part1(INPUT_1).unwrap());
    println!("Day 1 - part 2: {}", day01::part2(INPUT_1).unwrap());

    const INPUT_2: &str = include_str!("../inputs/input2.txt");
    println!("Day 2 - part 1: {}", day02::part1(INPUT_2).unwrap());
    println!("Day 2 - part 2: {}", day02::part2(INPUT_2).unwrap());

    const INPUT_3: &str = include_str!("../inputs/input3.txt");
    println!("Day 3 - part 1: {}", day03::part1(INPUT_3).unwrap());
    println!("Day 3 - part 2: {}", day03::part2(INPUT_3).unwrap());

    const INPUT_4: &str = include_str!("../inputs/input4.txt");
    println!("Day 4 - part 1: {}", day04::part1(INPUT_4).unwrap());
    println!("Day 4 - part 2: {}", day04::part2(INPUT_4).unwrap());

    const INPUT_5: &str = include_str!("../inputs/input5.txt");
    println!("Day 5 - part 1: {}", day05::part1(INPUT_5).unwrap());
    println!("Day 5 - part 2: {}", day05::part2(INPUT_5).unwrap());
}

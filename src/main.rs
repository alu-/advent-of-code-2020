mod day01;
mod day02;
mod day03;

fn main() {
    const INPUT_1: &str = include_str!("../inputs/input1.txt");
    println!("Day 1 - part 1: {}", day01::part1(INPUT_1));
    println!("Day 1 - part 2: {}", day01::part2(INPUT_1));

    const INPUT_2: &str = include_str!("../inputs/input2.txt");
    println!("Day 2 - part 1: {}", day02::part1(INPUT_2).unwrap());
    println!("Day 2 - part 2: {}", day02::part2(INPUT_2).unwrap());

    const INPUT_3: &str = include_str!("../inputs/input3.txt");
    println!("Day 3 - part 1: {}", day03::part1(INPUT_3).unwrap());
    println!("Day 3 - part 2: {}", day03::part2(INPUT_3).unwrap());
}

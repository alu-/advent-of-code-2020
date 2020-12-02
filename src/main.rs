mod day01;
mod day02;

fn main() {
    const INPUT_1: &str = include_str!("../inputs/input1.txt");
    println!("Day 1 - part 1: {}", day01::part1(INPUT_1));
    println!("Day 1 - part 2: {}", day01::part2(INPUT_1));

    const INPUT_2: &str = include_str!("../inputs/input2.txt");
    println!("Day 2 - part 1: {}", day02::part1(INPUT_2).unwrap());
    println!("Day 2 - part 2: {}", day02::part2(INPUT_2).unwrap());
}

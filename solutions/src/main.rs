mod day01;

fn main() {
    const INPUT_1: &str = include_str!("./../../inputs/input1.txt");
    println!("Day 1 - part 1: {}", day01::part1(INPUT_1));
    println!("Day 1 - part 2: {}", day01::part2(INPUT_1));
}

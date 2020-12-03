pub fn is_tree_at_coordinates(terrain: &Vec<Vec<char>>, x: &usize, y: &usize) -> bool {
    terrain[*y][*x] == '#'
}

#[derive(Debug)]
struct WrappingStepper {
    current: usize,
    step: usize,
    wrap: usize,
    stop: usize,
    taken: usize,
}

impl WrappingStepper {
    fn new(current: usize, step: usize, wrap: usize, stop: usize) -> WrappingStepper {
        WrappingStepper {
            current,
            step,
            wrap,
            stop,
            taken: 0,
        }
    }
}

impl Iterator for WrappingStepper {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.taken += 1;
        self.current += self.step;
        if self.current >= self.wrap {
            self.current = self.current % self.wrap;
        }

        if self.taken > self.stop {
            None
        } else {
            Some(self.current)
        }
    }
}

fn step(current: usize, step: usize, wrap: usize, stop: usize) -> WrappingStepper {
    WrappingStepper::new(current, step, wrap, stop)
}

pub fn get_slope_coordinates(
    steps_right: usize,
    steps_down: usize,
    columns: usize,
    lines: usize,
) -> Vec<(usize, usize)> {
    let down: Vec<usize> = (steps_down..lines).step_by(steps_down).collect();
    let right: Vec<usize> = step(0, steps_right, columns, down.len()).collect();

    let mut coordinates: Vec<(usize, usize)> = Vec::new();
    for (x, y) in right.iter().zip(&down) {
        coordinates.push((*x, *y));
    }

    coordinates
}

pub fn part1(input: &str) -> Result<usize, String> {
    let terrain: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let lines = terrain.len();
    let columns = terrain.iter().map(|x| x.len()).max().expect("Columns");
    let mut tree_count = 0;
    let slope = get_slope_coordinates(3, 1, columns, lines);

    for (x, y) in slope {
        if is_tree_at_coordinates(&terrain, &x, &y) {
            tree_count += 1
        }
    }

    Ok(tree_count)
}

pub fn part2(input: &str) -> Result<usize, String> {
    let terrain: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let lines = terrain.len();
    let columns = terrain.iter().map(|x| x.len()).max().expect("Columns");

    let mut slopes: Vec<Vec<(usize, usize)>> = Vec::new();
    slopes.push(get_slope_coordinates(1, 1, columns, lines));
    slopes.push(get_slope_coordinates(3, 1, columns, lines));
    slopes.push(get_slope_coordinates(5, 1, columns, lines));
    slopes.push(get_slope_coordinates(7, 1, columns, lines));
    slopes.push(get_slope_coordinates(1, 2, columns, lines));

    let mut trees: Vec<usize> = Vec::new();
    for (index, slope) in slopes.iter().enumerate() {
        trees.insert(index, 0);
        for (x, y) in slope {
            if is_tree_at_coordinates(&terrain, &x, &y) {
                trees[index] += 1;
            }
        }
    }

    Ok(trees.iter().fold(1, |a, b| a * b))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 336);
    }
}

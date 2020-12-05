use itertools::Itertools;

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
    id: usize,
}

fn get_seat_id(row: &usize, column: &usize) -> usize {
    row * 8 + column
}

fn decode_bsp(bsp: &str) -> Seat {
    let row_instructions: Vec<char> = bsp.chars().take(7).collect();
    let mut rows = 0..127;

    for partition in row_instructions {
        match partition {
            'F' => {
                rows.end -= rows.len() / 2 + 1;
            }
            'B' => {
                rows.start += rows.len() / 2 + 1;
            }
            _ => panic!("Unknown binary space partitioning instruction."),
        };
    }

    let column_instructions: Vec<char> = bsp.chars().skip(7).collect();
    let mut columns = 0..7;

    for partition in column_instructions {
        match partition {
            'L' => {
                columns.end -= columns.len() / 2 + 1;
            }
            'R' => {
                columns.start += columns.len() / 2 + 1;
            }
            _ => panic!("Unknown binary space partitioning instruction."),
        };
    }

    Seat {
        row: rows.start,
        column: columns.start,
        id: get_seat_id(&rows.start, &columns.start),
    }
}

pub fn part1(input: &str) -> Result<usize, String> {
    let seat = input
        .lines()
        .map(|x| decode_bsp(&x))
        .fold(0, |max, x| match x.id > max {
            true => x.id,
            false => max,
        });

    return Ok(seat);
}

pub fn part2(input: &str) -> Result<usize, String> {
    let seats: Vec<_> = input
        .lines()
        .map(|x| decode_bsp(&x))
        .sorted_by(|a, b| a.id.cmp(&b.id))
        .scan(None, |state, x| {
            let prev = state.take();
            *state = Some(x.id);
            Some((prev, Some(x.id)))
        })
        .filter_map(|(prev, x)| {
            if prev.and_then(|x| Some(x + 2)) == x {
                x.and_then(|x| Some(x - 1))
            } else {
                None
            }
        })
        .collect();

    return Ok(seats[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(get_seat_id(&44, &5), 357);
        assert_eq!(get_seat_id(&70, &7), 567);
        assert_eq!(get_seat_id(&14, &7), 119);
        assert_eq!(get_seat_id(&102, &4), 820);

        assert_eq!(part1(&"FBFBBFFRLR").unwrap(), 357);
        assert_eq!(part1(&"BFFFBBFRRR").unwrap(), 567);
        assert_eq!(part1(&"FFFBBBFRRR").unwrap(), 119);
        assert_eq!(part1(&"BBFFBBFRLL").unwrap(), 820);
    }
}

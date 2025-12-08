use crate::AdventHashMap;

pub static INPUT: &str = include_str!("../../input/6.txt");
pub static TEST_INPUT: &str = include_str!("../../input/6_test.txt");

pub fn a(input: &str) -> i64 {
    let mut map: AdventHashMap<usize, Vec<_>> = AdventHashMap::default();
    let lines = input.lines();

    for line in lines {
        for (col, value) in line.split_whitespace().enumerate() {
            if let Some(v) = map.get_mut(&col) {
                v.push(value);
            } else {
                map.insert(col, vec![value]);
            }
        }
    }

    let mut sum = 0;
    for values in map.values() {
        let mut res = 0;

        for index in 0..values.len() - 1 {
            let value = values[index].parse::<i64>().unwrap();

            match values[values.len() - 1] {
                "+" => {
                    res += value;
                }
                _ => {
                    if res == 0 {
                        res += value;
                    } else {
                        res *= value;
                    }
                }
            }
        }

        sum += res;
    }

    sum
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 4277556);
    assert_eq!(a(INPUT), 5335495999141);
}

pub fn b(input: &str) -> i64 {
    let mut grid: AdventHashMap<(usize, usize), char> = AdventHashMap::default();
    let mut height = 0;
    let mut width = 0;

    for (row, line) in input.lines().enumerate() {
        height = height.max(row + 1);
        for (col, ch) in line.chars().enumerate() {
            width = width.max(col + 1);
            grid.insert((row, col), ch);
        }
    }

    let mut total_sum = 0;
    let mut current_block_cols = Vec::new();

    for col in 0..width {
        let mut is_separator = true;
        for row in 0..height {
            if let Some(&ch) = grid.get(&(row, col))
                && ch != ' '
            {
                is_separator = false;
                break;
            }
        }

        if is_separator {
            if !current_block_cols.is_empty() {
                total_sum += solve_block(&current_block_cols, height, &grid);
                current_block_cols.clear();
            }
        } else {
            current_block_cols.push(col);
        }
    }

    if !current_block_cols.is_empty() {
        total_sum += solve_block(&current_block_cols, height, &grid);
    }

    total_sum
}

fn solve_block(cols: &[usize], height: usize, grid: &AdventHashMap<(usize, usize), char>) -> i64 {
    let mut op = ' ';
    for &col in cols {
        if let Some(&ch) = grid.get(&(height - 1, col)) {
            op = ch;
            break;
        }
    }

    let mut values = Vec::new();
    for &col in cols {
        let mut num_str = String::new();
        for row in 0..height - 1 {
            if let Some(&ch) = grid.get(&(row, col))
                && ch.is_ascii_digit()
            {
                num_str.push(ch);
            }
        }

        if !num_str.is_empty() {
            let value = num_str.parse::<i64>().unwrap();
            values.push(value);
        }
    }

    match op {
        '+' => values.iter().sum(),
        '*' => values.iter().product(),
        _ => 0,
    }
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 3263827);
    assert_eq!(b(INPUT), 10142723156431);
}

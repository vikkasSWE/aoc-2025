use crate::AdventHashMap;

pub static INPUT: &str = include_str!("../../input/4.txt");
pub static TEST_INPUT: &str = include_str!("../../input/4_test.txt");

pub fn a(input: &str) -> i64 {
    let mut sum = 0;
    let mut map = AdventHashMap::default();

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.trim().chars().enumerate() {
            map.insert((x as i32, y as i32), c);
        }
    }

    let map_clone = map.clone();

    for ((x, y), v) in map_clone.into_iter() {
        let mut count = 0;
        if v == '@' {
            if map.get(&(x - 1, y)).is_some_and(|v| *v == '@') {
                count += 1;
            }
            if map.get(&(x + 1, y)).is_some_and(|v| *v == '@') {
                count += 1;
            }

            if map.get(&(x, y - 1)).is_some_and(|v| *v == '@') {
                count += 1;
            }
            if map.get(&(x, y + 1)).is_some_and(|v| *v == '@') {
                count += 1;
            }

            if map.get(&(x - 1, y - 1)).is_some_and(|v| *v == '@') {
                count += 1;
            }
            if map.get(&(x - 1, y + 1)).is_some_and(|v| *v == '@') {
                count += 1;
            }

            if map.get(&(x + 1, y - 1)).is_some_and(|v| *v == '@') {
                count += 1;
            }
            if map.get(&(x + 1, y + 1)).is_some_and(|v| *v == '@') {
                count += 1;
            }

            if count < 4 {
                sum += 1;
            }
        }
    }

    sum
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 13);
    assert_eq!(a(INPUT), 1540);
}

pub fn b(input: &str) -> i64 {
    let mut sum = 0;
    let mut map = AdventHashMap::default();

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.trim().chars().enumerate() {
            map.insert((x as i32, y as i32), c);
        }
    }

    let mut old_sum = 1;

    while old_sum - sum != 0 {
        let map_clone = map.clone();

        old_sum = sum;

        for ((x, y), v) in map_clone.into_iter() {
            let mut count = 0;
            if v == '@' {
                if map.get(&(x - 1, y)).is_some_and(|v| *v == '@') {
                    count += 1;
                }
                if map.get(&(x + 1, y)).is_some_and(|v| *v == '@') {
                    count += 1;
                }

                if map.get(&(x, y - 1)).is_some_and(|v| *v == '@') {
                    count += 1;
                }
                if map.get(&(x, y + 1)).is_some_and(|v| *v == '@') {
                    count += 1;
                }

                if map.get(&(x - 1, y - 1)).is_some_and(|v| *v == '@') {
                    count += 1;
                }
                if map.get(&(x - 1, y + 1)).is_some_and(|v| *v == '@') {
                    count += 1;
                }

                if map.get(&(x + 1, y - 1)).is_some_and(|v| *v == '@') {
                    count += 1;
                }
                if map.get(&(x + 1, y + 1)).is_some_and(|v| *v == '@') {
                    count += 1;
                }

                if count < 4 {
                    map.remove(&(x, y));
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 43);
    assert_eq!(b(INPUT), 8972);
}

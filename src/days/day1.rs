pub static INPUT: &str = include_str!("../../input/1.txt");
pub static TEST_INPUT: &str = include_str!("../../input/1_test.txt");

pub fn a(input: &str) -> i32 {
    let mut start = 50;
    let mut count = 0;

    for line in input.lines() {
        let dir = line[0..1].parse::<char>().unwrap();
        let rot = line[1..].parse::<i32>().unwrap();

        match dir {
            'L' => {
                start -= rot;
            }
            'R' => {
                start += rot;
            }
            _ => (),
        }

        while start > 99 {
            start -= 100;
        }

        while start < 0 {
            start += 100;
        }

        if start == 0 {
            count += 1;
        }
    }

    count
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 3);
    assert_eq!(a(INPUT), 1135);
}

pub fn b(input: &str) -> i32 {
    let mut start = 50;
    let mut count = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        let dir = line[0..1].parse::<char>().unwrap();
        let rot = line[1..].parse::<i32>().unwrap();

        let step = if dir == 'L' { -1 } else { 1 };
        let mut remaining = rot;

        while remaining > 0 {
            start += step;
            if start == 100 {
                start = 0;
            } else if start == -1 {
                start = 99;
            }

            if start == 0 {
                count += 1;
            }

            remaining -= 1;
        }
    }

    count
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 6);
    assert_eq!(b(INPUT), 6558);
}

pub static INPUT: &str = include_str!("../../input/2.txt");
pub static TEST_INPUT: &str = include_str!("../../input/2_test.txt");

fn is_invalid(id: i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let mid = len / 2;
    let (left, right) = s.split_at(mid);

    left == right
}

pub fn a(input: &str) -> i64 {
    let mut sum = 0;
    for i in input.trim().split(',') {
        let mut range = i.split('-');

        let id1 = range.next().unwrap().trim().parse::<i64>().expect("test");
        let id2 = range.next().unwrap().trim().parse::<i64>().expect("test");

        for id in id1..=id2 {
            if is_invalid(id) {
                sum += id;
            }
        }
    }

    println!("{sum}");

    sum
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1227775554);
    assert_eq!(a(INPUT), 35367539282);
}

fn is_invalid_2(id: i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    for pat_len in 1..=len / 2 {
        if !len.is_multiple_of(pat_len) {
            continue;
        }

        let pattern = &s[0..pat_len];
        let repeats = len / pat_len;

        if pattern.repeat(repeats) == s {
            return true;
        }
    }

    false
}

pub fn b(input: &str) -> i64 {
    let mut sum = 0;
    for i in input.trim().split(',') {
        let mut range = i.split('-');

        let id1 = range.next().unwrap().trim().parse::<i64>().expect("test");
        let id2 = range.next().unwrap().trim().parse::<i64>().expect("test");

        for id in id1..=id2 {
            if is_invalid_2(id) {
                sum += id;
            }
        }
    }

    println!("{sum}");

    sum
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 4174379265);
    assert_eq!(b(INPUT), 45814076230);
}

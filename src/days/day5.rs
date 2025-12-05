pub static INPUT: &str = include_str!("../../input/5.txt");
pub static TEST_INPUT: &str = include_str!("../../input/5_test.txt");

pub fn a(input: &str) -> i64 {
    let mut split = input.trim().split("\r\n\r\n");
    let ranges = split.next().unwrap().split("\r\n").collect::<Vec<_>>();
    let ids = split.next().unwrap().split("\r\n").collect::<Vec<_>>();

    let mut fresh = 0;
    for id in ids {
        let id = id.parse::<i64>().unwrap();

        for range in &ranges {
            let mut split = range.split('-');
            let start = split.next().unwrap().parse::<i64>().unwrap();
            let end = split.next().unwrap().parse::<i64>().unwrap();

            if id >= start && id <= end {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 3);
    assert_eq!(a(INPUT), 615);
}

pub fn b(input: &str) -> usize {
    let mut split = input.trim().split("\r\n\r\n");
    let ranges = split.next().unwrap();

    let mut ranges = ranges
        .lines()
        .map(|l| {
            let mut r = l.split('-');
            let start = r.next().unwrap().parse::<i64>().unwrap();
            let end = r.next().unwrap().parse::<i64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in ranges {
        if let Some((_, merge_end)) = merged.last_mut() {
            if start <= *merge_end + 1 {
                *merge_end = (*merge_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    merged.iter().map(|(s, e)| (e - s + 1) as usize).sum()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 14);
    assert_eq!(b(INPUT), 353716783056994);
}

pub static INPUT: &str = include_str!("../../input/3.txt");
pub static TEST_INPUT: &str = include_str!("../../input/3_test.txt");

pub fn a(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut max = 0;

        let chars = line.chars().collect::<Vec<_>>();
        let chars_clone = line.chars().collect::<Vec<_>>();

        for (i, char) in chars.iter().enumerate() {
            for c in chars_clone.iter().skip(i + 1) {
                let value = format!("{}{}", char, c).parse::<i64>().unwrap();

                if max < value {
                    max = value;
                }
            }
        }

        sum += max;
    }

    sum
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 357);
    assert_eq!(a(INPUT), 17179);
}

pub fn b(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        let seq_len = 12;
        let length = chars.len();

        let mut remove = length - seq_len;
        let mut sequence = Vec::with_capacity(seq_len);

        for &c in &chars {
            while !sequence.is_empty() && remove > 0 && sequence.last().unwrap() < &c {
                sequence.pop();
                remove -= 1;
            }

            sequence.push(c);
        }

        sequence.truncate(seq_len);

        let value = sequence.iter().collect::<String>().parse::<i64>().unwrap();
        sum += value;
    }

    sum
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 3121910778619);
    assert_eq!(b(INPUT), 170025781683941);
}

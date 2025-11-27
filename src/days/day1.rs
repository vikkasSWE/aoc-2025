pub static INPUT: &str = include_str!("../../input/1.txt");
pub static TEST_INPUT: &str = include_str!("../../input/1_test.txt");

pub fn a(_input: &str) -> i32 {
    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 11);
    assert_eq!(a(INPUT), 1938424);
}

pub fn b(_input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 31);
    assert_eq!(b(INPUT), 22014209);
}

use std::iter;

fn pattern(position: usize, width: usize) -> Vec<i32> {
    assert!(position > 0);

    let mut result = Vec::<i32>::new();

    for p in &[0, 1, 0, -1] {
        result.extend(iter::repeat(p).take(position));
    }

    while result.len() < width + 1 {
        result.extend(result.clone());
    }

    result.remove(0);

    result
}

pub fn test() {
    let p = pattern(1, 16);
    println!("{:?}", p);
    let p = pattern(2, 16);
    println!("{:?}", p);
    let p = pattern(3, 16);
    println!("{:?}", p);
}

pub fn part1() {}

pub fn part2() {}

use crate::common::*;
use std::iter;

fn create_pattern(position: usize, width: usize) -> Vec<i32> {
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

fn single_position(position: usize, input: &Vec::<i32>) -> i32 {
    let pattern = create_pattern(position + 1, input.len());
    let mut result = 0;

    for i in 0..input.len() {
        result += input[i] * pattern[i];
    }
    result = result.abs() % 10;

    result
}

fn single_phase(input: &Vec<i32>) -> Vec<i32> {
    let mut output = vec![0; input.len()];

    for position in 0..input.len() {
        output[position] = single_position(position, input);
    }

    output
}

fn fft(input: &Vec<i32>, phases: i32) -> Vec<i32> {
    let mut output = input.clone();
    for _ in 0..phases {
        output = single_phase(&output);
    }
    output
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| i32::from_str_radix(&c.to_string(), 10).expect("Invalid input"))
        .collect::<Vec<i32>>()
}

fn parse_offset(input: &str) -> i32 {
    input
        .chars()
        .take(7)
        .map(|c| i32::from_str_radix(&c.to_string(), 10).expect("Invalid input"))
        .fold(0, |acc, p| acc * 10 + p)
}

pub fn test() {
    let p = create_pattern(1, 16);
    println!("{:?}", p);
    let p = create_pattern(2, 16);
    println!("{:?}", p);
    let p = create_pattern(3, 16);
    println!("{:?}", p);

    let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];

    for _ in 0..4 {
        input = single_phase(&input);

        println!("{:?}", input);
    }

    let result = fft(&parse_input("80871224585914546619083218645595"), 100);
    println!("{:?}", &result[0..8]);
    assert_eq!(&result[0..8], [2, 4, 1, 7, 6, 1, 7, 6]);
    let result = fft(&parse_input("19617804207202209144916044189917"), 100);
    println!("{:?}", &result[0..8]);
    assert_eq!(&result[0..8], [7, 3, 7, 4, 5, 4, 1, 8]);
    let result = fft(&parse_input("69317163492948606335995924319873"), 100);
    println!("{:?}", &result[0..8]);
    assert_eq!(&result[0..8], [5, 2, 4, 3, 2, 1, 3, 3]);
}

pub fn part1() {
    let input = import_lines("src/day16/input.txt");
    let result = fft(&parse_input(&input), 100);
    println!("Part 1 : {:?}", &result[0..8]);
    assert_eq!(&result[0..8], [5, 3, 2, 9, 6, 0, 8, 2]);
}

pub fn part2() {
    let input = import_lines("src/day16/input.txt").repeat(10000);
    let offset = parse_offset(&input);

    println!("{}", offset);
    let result = fft(&parse_input(&input), 1);
    println!("Part 2 : {:?}", &result[0..8]);
}

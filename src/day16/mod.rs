use crate::common::*;

fn multiplier(index: usize, position: usize) -> i32 {
    assert!(position > 0);

    let ix = index % (4 * position);

    if ix >= position - 1 && ix < 2 * position - 1 {
        1
    } else if ix >= 2 * position - 1 && ix < 3 * position - 1 {
        0
    } else if ix >= 3 * position - 1 && ix < 4 * position - 1 {
        -1
    } else {
        0
    }
}

fn fft_single_position(position: usize, input: &Vec<i32>) -> i32 {
    let mut result = 0;

    for i in 0..input.len() {
        result += input[i] * multiplier(i, position + 1);
    }
    result.abs() % 10
}

fn fft_single_phase(input: &Vec<i32>) -> Vec<i32> {
    let mut output = vec![0; input.len()];

    for position in 0..output.len() {
        output[position] = fft_single_position(position, input);
    }

    output
}

fn fft(input: &Vec<i32>, phases: i32) -> Vec<i32> {
    let mut output = input.clone();
    for _ in 0..phases {
        output = fft_single_phase(&output);
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
    let mut result = parse_input("00000010").repeat(16);
    println!("{:?}", &result[0..100]);
    for _ in 0..8 {
        result = fft_single_phase(&result);
        println!("{:?}", &result[0..100]);
    }
}

pub fn part1() {
    let input = import_lines("src/day16/input.txt");
    let result = fft(&parse_input(&input), 100);
    println!("Part 1 : {:?}", &result[0..8]);
    assert_eq!(&result[0..8], [5, 3, 2, 9, 6, 0, 8, 2]);
}

pub fn part2() {
    // let input = import_lines("src/day16/input.txt").repeat(10000);
    // let offset = parse_offset(&input) as usize;

    // println!("{}", offset);
    // let result = fft(&parse_input(&input), 1);
    // println!("Part 2 : {:?}", &result[offset..offset+8]);
}

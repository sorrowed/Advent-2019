fn password_range() -> std::ops::RangeInclusive<i32> {
	145852..=616942
}

fn is_not_decreasing(digits: [i32; 6]) -> bool {
	let mut result = true;
	for ix in 1..6 {
		result &= digits[ix] >= digits[ix - 1];
	}
	result
}

fn analyze_password(mut password: i32) -> ([i32; 6], [i32; 10]) {
	let mut digits = [0; 6];
	let mut histogram: [i32; 10] = [0; 10];

	for ix in 0..6 {
		digits[ix] = password % 10;
		histogram[(password % 10) as usize] += 1;
		password /= 10;
	}
	digits.reverse();
	(digits, histogram)
}

fn is_valid_first(password: i32) -> bool {

	let (digits, histogram) = analyze_password(password);

	is_not_decreasing(digits)
		&& histogram
			.iter()
			.filter(|&a| a >= &2)
			.count() > 0
}

fn is_valid_second(password: i32) -> bool {

	let (digits, histogram) = analyze_password(password);

	is_not_decreasing(digits)
		&& histogram
			.iter()
			.filter(|&a| a == &2)
			.count() > 0
}

pub fn part1() {
	let count = password_range()
		.filter(|&a| is_valid_first(a))
		.collect::<Vec<_>>()
		.len();

	println!("Valid passwords: {}", count);
}

pub fn part2() {
	let count = password_range()
		.filter(|&a| is_valid_second(a))
		.collect::<Vec<_>>()
		.len();

	println!("Valid passwords: {}", count);
}

struct Move {
	m: char,
	c: i32,
}

#[derive(Clone)]
struct Coordinate {
	x: i32,
	y: i32,
}

pub fn test() {
	let wire1 = "R8,U5,L5,D3";
	let wire2 = "U7,R6,D4,L4";

	let moves = Vec::<Move>::new();
	let mut coordinates1 = Vec::<Coordinate>::new();

	let mut pos = Coordinate { x: 0, y: 0 };
	for m in moves {
		match m.m {
			'U' => {
				pos.y += m.c;
			}
			'R' => {
				pos.x += m.c;
			}
			'D' => {
				pos.y -= m.c;
			}
			'L' => {
				pos.x -= m.c;
			}
			_ => {}
		}
		coordinates1.push(pos.clone());
	}
}

pub fn part1() {}

pub fn part2() {}

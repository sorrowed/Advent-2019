use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug,Clone, Hash)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl std::cmp::PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl std::cmp::Eq for Vector {}

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn path(&self, other: &Vector) -> Path {
        let x = (other.x - self.x) as f32;
        let y = (other.y - self.y) as f32;

        Path {
            target: other.clone(),
            angle: angle_in_degrees(y.atan2(x)),
            distance: x.hypot(y),
        }
    }

    pub fn magnitude(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

pub struct Path {
    pub target: Vector,
    pub angle: f32,
    pub distance: f32,
}

// Path is considered equal when they have the same angle
impl std::cmp::PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        return self.angle == other.angle;
    }
}

// Path is compared based on distance
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "T:{} A:{},D:{}", self.target, self.angle, self.distance)
    }
}

/*
	Convert from radians to degrees in coordinate system used by asteroids (0 degrees points up, positive rotate clockwise)
*/
fn angle_in_degrees(angle: f32) -> f32 {
    // Bring in range [0 ~ 360] degrees
    let r = if angle.to_degrees() < 0f32 {
        360f32 + angle.to_degrees()
    } else {
        angle.to_degrees()
    };

    // And offset with 90 degrees to make up/north 0 degrees, clamp to [0 ~ 360> degrees
    (r + 90f32) % 360f32
}

pub fn import(name: &str) -> Vec<i64> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        vec.push(line.unwrap().parse::<i64>().unwrap())
    }
    return vec;
}

pub fn import_lines(name: &str) -> String {
    fs::read_to_string(name).expect("Error parsing file")
}

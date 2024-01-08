use std::ops::RangeInclusive;
use advent_of_code::get_nums;
advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    dx:f64,
    dy:f64,
    dz:f64,
}

impl Hail {
    fn pos(&self, axis: char) -> f64 {
        match axis { 'x' => self.x, 'y' => self.y, 'z' => self.z, _ => unreachable!() }
    }
    fn vel(&self, axis: char) -> f64 {
        match axis { 'x' => self.dx, 'y' => self.dy, 'z' => self.dz, _ => unreachable!() }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones: Vec<Hail> = input.lines().map(|line| {
        let nums = get_nums(line);
        Hail{x: nums[0] as f64, y: nums[1] as f64, z: nums[2] as f64,
            dx: nums[3] as f64, dy: nums[4] as f64, dz: nums[5] as f64}
    }).collect();


    let mut count = 0;
    let lo = 200000000000000.;
    let hi = 400000000000000.;
    for i in 0..stones.len() {
        for j in i+1..stones.len() {
            let a = stones[i];
            let b = stones[j];
            let denom = a.dx * b.dy - a.dy * b.dx;
            if denom == 0. {
                // parallel paths
                continue;
            }
            let g: f64 = (a.dx * a.y + a.dy * b.x - a.dy * a.x - b.y * a.dx) / denom;
            let f: f64 = (b.x + b.dx * g - a.x) / a.dx;
            if f < 0. || g < 0. {
                // only forward
                continue
            }
            let p = [a.x + a.dx * f, a.y + a.dy * f];
            if lo <= p[0] && p[0] <= hi && lo <= p[1] && p[1] <= hi {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<i64> {
    let stones: Vec<Hail> = input.lines().map(|line| {
        let nums = get_nums(line);
        Hail{x: nums[0] as f64, y: nums[1] as f64, z: nums[2] as f64,
            dx: nums[3] as f64, dy: nums[4] as f64, dz: nums[5] as f64}
    }).collect();

    const BRUTE_RANGE: RangeInclusive<i64> = -1000..=1000;

    // init all possible velocities
    let mut possible_x_vel: Vec<i64> = BRUTE_RANGE.collect();
    let mut possible_y_vel: Vec<i64> = BRUTE_RANGE.collect();
    let mut possible_z_vel: Vec<i64> = BRUTE_RANGE.collect();

    while possible_x_vel.len() != 1 || possible_y_vel.len() != 1 || possible_z_vel.len() != 1 {
        for i in 0..stones.len() {
            for j in i + 1..stones.len() {
                let a = stones[i];
                let b = stones[j];
                let process = |possible: &mut Vec<i64>, axis: char| {
                    let pos = (a.pos(axis) as i64, b.pos(axis) as i64);
                    let vel = (a.vel(axis) as i64, b.vel(axis) as i64);

                    // only want stones with matching velocities
                    if vel.0 != vel.1 {
                        return;
                    }

                    let dist = (pos.0 - pos.1).abs();
                        // rock is moving with velocity dx'.
                        // pair of hailstones are moving at dx. dist b/w them is a constant integer.
                        // (dx' - dx) * (t2 - t1) = x2 - x1
                        // (x2 - x1) / (dx' - dx) = t2 - t1 => assume time delta is integer
                    possible.retain(|v| v != &vel.0 && dist % (v - vel.0) == 0);
                };

                process(&mut possible_x_vel, 'x');
                process(&mut possible_y_vel, 'y');
                process(&mut possible_z_vel, 'z');
            }
        }
    }

    let a = stones[0];
    let b = stones[1];
    let (xv, yv, zv) = (
        possible_x_vel[0] as f64,
        possible_y_vel[0] as f64,
        possible_z_vel[0] as f64,
    );

    // the line made by subtracting the rock's velocities from a stone's velocities
    // passes through the origin of the rock's trajectory
    let ma = (a.dy - yv) / (a.dx - xv);
    let mb = (b.dy - yv) / (b.dx - xv);

    let ca = a.y - ma * a.x;
    let cb = b.y - mb * b.x;

    // velocity-adjusted lines cross at rock's origin
    // x * ma + ca = x * mb + cb
    // x * ma - x * mb = cb - ca
    let x = (cb - ca) / (ma - mb);
    let y = ma * x + ca;

    // time = dist / velocity
    let t = (a.x - x) / (xv - a.dx);
    let z = a.z + (a.dz - zv) * t;

    ((x + y + z) as i64).into()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // set lo to 7. and hi to 27.
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}

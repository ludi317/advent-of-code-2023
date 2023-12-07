mod day;
pub mod template;

pub use day::*;

// parses a sequence of *non-negative* numbers
pub fn get_nums(s: &str) -> Vec<isize> {
    let mut nums: Vec<isize> = vec![];
    let mut num: isize = 0;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + c.to_digit(10).unwrap() as isize;
            num_found = true;
        } else if num_found {
            nums.push(num);
            num = 0;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num);
    }
    nums
}


// concatenates all *non-negative* numbers on a line
pub fn get_nums_as_1(s: &str) -> isize {
    let mut num: isize = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + c.to_digit(10).unwrap() as isize;
        }
    }
    num
}

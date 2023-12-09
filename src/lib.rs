mod day;
pub mod template;

pub use day::*;

// parses a sequence of numbers
pub fn get_nums(s: &str) -> Vec<isize> {
    let mut nums: Vec<isize> = vec![];
    let mut num: isize = 0;
    let mut sign = 1;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + c.to_digit(10).unwrap() as isize;
            num_found = true;
        } else if c == '-' {
            sign = -1;
        } else if num_found {
            nums.push(num * sign);
            num = 0;
            sign = 1;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num * sign);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nums() {
        let s = "4 27 78 180 375 742  0";
        let v = vec![4, 27, 78, 180, 375, 742, 0];
        assert_eq!(get_nums(s), v);
    }

    #[test]
    fn test_get_nums_neg() {
        let s = "14 23 30 32 26 9 -22 -70";
        let v = vec![14, 23, 30, 32, 26, 9, -22, -70];
        assert_eq!(get_nums(s), v);
    }
}

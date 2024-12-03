advent_of_code::solution!(2);

fn safe_or_unsafe(input: &str) -> i32 {
    let mut iter = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .peekable();
    let mut max_ind = 0;
    let mut min_ind = 0;
    let mut res = 0;
    while let Some(cur) = iter.next() {
        if let Some(next) = iter.peek() {
            let diff = cur - next;
            if diff.abs() > 3 {
                res += 1;
            };
            if diff == 0 {
                res += 1;
            }
            if diff.is_positive() {
                max_ind = 1;
            }
            if diff.is_negative() {
                min_ind = 1;
            }
            if max_ind + min_ind == 2 {
                res += 1;
                min_ind = 0;
                max_ind = 0;
            }
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let res = input.lines().filter(|x| safe_or_unsafe(x) > 0).count();
    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let res = input.lines().filter(|x| safe_or_unsafe(x) <= 1).count();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_or_unsafe_example_1() {
        let result = safe_or_unsafe("7 6 4 2 1");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_safe_or_unsafe_example_3() {
        let result = safe_or_unsafe("1 3 2 4 5");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_safe_or_unsafe_example_2() {
        let result = safe_or_unsafe("8 6 4 4 1");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_safe_or_unsafe_example_4() {
        let result = safe_or_unsafe("1 2 7 8 9");
        assert!(result > 1);
    }

    #[test]
    fn test_safe_or_unsafe_example_5() {
        let result = safe_or_unsafe("4 3 7 8 9");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

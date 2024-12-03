advent_of_code::solution!(1);
use std::convert::TryInto;

fn create_iterator(input: &str, index: i32) -> impl Iterator<Item = i32> + '_ {
    input
        .split_whitespace()
        .enumerate()
        .filter(move |(x, _)| TryInto::<i32>::try_into(*x).unwrap_or_default() % 2 == index)
        .map(|(_, x)| x.parse::<i32>().unwrap())
}

fn find_count(iterator: Vec<i32>, num: &i32) -> i32 {
    iterator
        .into_iter()
        .filter(|x| x == num)
        .count()
        .try_into()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut list_1: Vec<i32> = create_iterator(input, 0).collect::<Vec<i32>>();
    let mut list_2: Vec<i32> = create_iterator(input, 1).collect::<Vec<i32>>();
    list_1.sort();
    list_2.sort();

    let diff: i32 = list_1
        .iter()
        .zip(list_2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Some(diff)
}

pub fn part_two(input: &str) -> Option<i32> {
    let list_2: Vec<i32> = create_iterator(input, 1).collect();
    let value: i32 = create_iterator(input, 0)
        .map(|x| x * find_count(list_2.clone(), &x))
        .sum();
    Some(value)
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]

    fn test_create_list() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result: Vec<i32> = create_iterator(input, 0).collect();
        assert_eq!(vec![3, 4, 2, 1, 3, 3], result);
    }

    #[test]
    fn test_find_count() {
        let input = vec![3, 4, 2, 1, 3, 3];
        let result = find_count(input, &3);
        assert_eq!(3, result);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

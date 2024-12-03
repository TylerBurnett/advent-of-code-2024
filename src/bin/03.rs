use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut accumulator = 0;
    for (_, [str1, str2]) in re.captures_iter(input).map(|c| c.extract()) {
        let n1 = str1.parse::<i32>().unwrap();
        let n2 = str2.parse::<i32>().unwrap();

        accumulator += n1 * n2;
    }

    Some(accumulator)
}

pub fn part_two(input: &str) -> Option<i32> {
    let re = Regex::new(r"(?:(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    let mut accumulator = 0;
    let mut should_capture = true;
    for groups in re.captures_iter(input) {
        let values: Vec<&str> = groups.iter().filter_map(|f| { f } ).map(|f| f.as_str()).collect();

        if values.len() == 2 {
            should_capture = values[1] == "do()";
            continue;
        }

        if should_capture {
            let n1 = values[1].parse::<i32>().unwrap();
            let n2 = values[2].parse::<i32>().unwrap();
            accumulator += n1 * n2;
        }
    }

    Some(accumulator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(183669043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(59097164));
    }
}

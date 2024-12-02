advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut accumulator = 0;
    for report in input.lines() {
        let mut safe = true;
        let levels: Vec<&str> = report.split(' ').collect();

        let mut previous = levels[0].parse::<i32>().unwrap();
        let is_positive = (previous - levels[1].parse::<i32>().unwrap()) > 0;

        for i in 1..levels.len() {
            let current = levels[i].parse::<i32>().unwrap();

            let delta = previous - current;
            let delta_normalized = delta.abs();

            if delta_normalized > 3 || delta_normalized < 1 || is_positive != (delta > 0) {
                safe = false;
                break;
            }

            previous = current;
        }

        if safe {
            accumulator += 1;
        }
    }

    Some(accumulator)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut accumulator = 0;
    for report in input.lines() {
        let mut failures = 0;
        let levels: Vec<&str> = report.split(' ').collect();

        let mut previous = levels[0].parse::<i32>().unwrap();
        let is_positive = (previous - levels[1].parse::<i32>().unwrap()) > 0;

        for i in 1..levels.len() {
            let current = levels[i].parse::<i32>().unwrap();

            let delta = previous - current;
            let delta_normalized = delta.abs();

            if delta_normalized > 3 || delta_normalized < 1 || is_positive != (delta > 0) {
                failures += 1;

                if failures > 1 {
                    break;
                }
                
            }

            previous = current;
        }

        if failures <= 1 {
            accumulator += 1;
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
        assert_eq!(result, Some(663));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(692));
    }
}

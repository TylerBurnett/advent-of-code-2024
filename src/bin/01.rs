use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();


    for item in input.lines() {
        let pair = item.split("   ").collect::<Vec<&str>>();

        list1.push(pair[0].parse::<i32>().unwrap());
        list2.push(pair[1].parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut accumulator: i32 = 0;
    for (index, li) in list1.into_iter().enumerate() {
        accumulator += (li - list2[index]).abs();
    }

    Some(accumulator)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2_hashed: HashMap<i32, i32> = HashMap::new();

    for item in input.lines() {
        let number_pair = item.split("   ").collect::<Vec<&str>>();

        list1.push(number_pair[0].parse::<i32>().unwrap());

        let item1 = number_pair[1].parse::<i32>().unwrap();
        match list2_hashed.get(&item1) {
            None => {
                list2_hashed.insert(item1, 1);
            }
            Some(oldValue) => {
                list2_hashed.insert(item1, oldValue + 1);
            }
        }
    }

    let result = list1.into_iter().map(|item| {
        let multiplier = list2_hashed.get(&item).cloned().unwrap_or(0);
        item * multiplier
    }).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2769675));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24643097));
    }
}

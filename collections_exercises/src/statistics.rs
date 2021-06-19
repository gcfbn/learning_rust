use std::collections::HashMap;
use std::cmp::Ordering;

pub fn mean(list: &[i32]) -> f32 {
    let sum = list.iter().sum::<i32>() as f32;
    sum / list.len() as f32
}

pub fn median(list: &mut [i32]) -> f32 {
    list.sort_unstable();

    if list.len() % 2 == 1 {
        list[list.len() / 2] as f32
    } else {
        (list[list.len() / 2] + list[list.len() / 2 - 1]) as f32 / 2.0
    }
}

pub fn mode(list: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max_occurrences = 0;
    let mut result: Vec<i32> = Vec::new();

    for (key, value) in map {
        match value.cmp(&max_occurrences) {
            Ordering::Greater => {
                max_occurrences = value;
                result.clear();
                result.push(*key)
            }
            Ordering::Equal => result.push(*key),
            Ordering::Less => ()
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let list = [-2, 2, 10, 0];
        assert_eq!(mean(&list), 2.5 );
    }
}

use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f32 {
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    return sum as f32 / list.len() as f32;
}

pub fn median(list: &mut [i32]) -> f32 {
    list.sort();

    return if list.len() % 2 == 1 {
        list[list.len() / 2] as f32
    } else {
        (list[list.len() / 2] + list[list.len() / 2 - 1]) as f32 / 2 as f32
    };
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
        if value > max_occurrences {
            max_occurrences = value;
            result.clear();
            result.push(*key);
        } else if value == max_occurrences {
            result.push(*key);
        }
    }

    result
}
pub fn average(list: &[i32]) -> f32 {
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    return sum as f32 / list.len() as f32
}
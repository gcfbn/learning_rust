mod statistics;

fn main() {
    let mut list = [1, 2, 10, 10, 25, -65, 13, 55, 25];
    println!("Mean: {}", statistics::mean(&list));
    println!("Median: {}", statistics::median(&mut list));
    println!("Mode: {:?}", statistics::mode(&list));
}

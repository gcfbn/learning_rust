mod statistics;
mod pig_latin;

fn main() {
    let mut list = [1, 2, 10, 10, 25, -65, 13, 55, 25];
    println!("Mean: {}", statistics::mean(&list));
    println!("Median: {}", statistics::median(&mut list));
    println!("Mode: {:?}", statistics::mode(&list));

    let text = "food snap guide fun swimming love a interesting pillow ice cream hair eat music";

    println!("Pig latin: {}", pig_latin::convert(&text));
}

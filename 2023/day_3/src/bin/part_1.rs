use day_3::part_1::process;

fn main() {
    let file = include_str!("part_1.txt");
    let result = process(file);
    println!("{}", result);
}

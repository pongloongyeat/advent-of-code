use day_3::part_2::process;

fn main() {
    let file = include_str!("part_2.txt");
    let result = process(file);
    println!("{}", result);
}

use day_2::part_1::{process, CubeCombination};

fn main() {
    let file = include_str!("part_1.txt");
    let result = process(file, CubeCombination::new(12, 13, 14));
    println!("{}", result);
}

use aoc_2024_day_11::stones::Stones;

fn main() {
    let input_str = include_str!("../../../input/example1.txt");
    let stones = Stones::from(input_str);
    println!("{}", stones);

    let stones = stones.blink();
    println!("{}", stones);
}

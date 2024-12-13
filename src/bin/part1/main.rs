use aoc_2024_day_11::stones::Stones;

fn main() {
    let input_str = include_str!("../../../input/example2.txt");
    let mut stones = Stones::from(input_str);
    println!("{}", stones);

    for _ in 0..6 {
        stones = stones.blink();
        println!("{}", stones);
    }

    println!("{}", stones.0.len());
}

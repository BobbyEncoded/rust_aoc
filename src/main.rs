mod general;
mod day7;
mod day8;
mod day9;
mod day1_2022;
mod day2_2022;
mod day3_2022;

const TESTFILENAME : &str = "Advent2022D3.txt";

fn main() {
    let file_contents = general::pull_file_contents(std::path::Path::new(TESTFILENAME));

    println!("{:?}", day3_2022::part2(day3_2022::parse(&file_contents)));
}
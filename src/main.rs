mod general;
mod day7;
mod day8;
mod day9;

const TESTFILENAME : &str = "Advent2021D9Test.txt";

fn main() {
    let file_contents = general::pull_file_contents(std::path::Path::new(TESTFILENAME));

    println!("{}", day9::run(&file_contents));
}
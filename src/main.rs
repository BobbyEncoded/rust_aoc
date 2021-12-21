mod general;
mod day7;
mod day8;

const TESTFILENAME : &str = "Advent2021D8.txt";

fn main() {
    let file_contents = general::pull_file_contents(std::path::Path::new(TESTFILENAME));

    println!("{}", day8::run(&file_contents));
}
mod general;
mod day7;

const TESTFILENAME : &str = "Advent2021D7.txt";

fn main() {
    let file_contents = general::pull_file_contents(std::path::Path::new(TESTFILENAME));

    println!("{}", day7::run(&file_contents));
}
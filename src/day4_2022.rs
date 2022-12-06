use std::collections::HashSet;
use std::iter::FromIterator;

pub fn parse (initial_contents : &String) -> Vec<Vec<char>> {
    initial_contents.lines().map(|input| input.chars().collect()).collect()
}

pub fn part1 (entry_data : Vec<Vec<char>>) -> i64 {
    entry_data.iter().map(|line| {
        let mut first_compartment = line.clone();
        let second_compartment = first_compartment.split_off(first_compartment.len() / 2);
        let first_hash : HashSet<char> = HashSet::from_iter(first_compartment);
        let second_hash : HashSet<char> = HashSet::from_iter(second_compartment);
        let intersect : Vec<&char> = first_hash.intersection(&second_hash).collect();
        let sum : u32 = intersect.iter().map(|item| {
                //println!("{}", item);
                match **item as u32 {
                    val if val < 93 => (val - 65) + 27,
                    val if val >= 93 => val - 96,
                    _ => 0,
                }
            }).sum();
        sum as i64
    }).sum()
}

pub fn part2 (entry_data : Vec<Vec<char>>) -> i64 {
    let hashed_data : Vec<HashSet<char>> = entry_data.iter().map(|line| HashSet::from_iter(line.to_owned())).collect();
    let windows : Vec<&[HashSet<char>]> = hashed_data.chunks(3).collect();
    let intersect_outer : Vec<Vec<char>> = 
        windows.iter().map(|share| {
            share.iter().cloned().reduce(|prev, next| HashSet::from_iter(prev.intersection(&next).map(|a| *a))).unwrap()
        }).map(|set| Vec::from_iter(set)).collect();
    intersect_outer.iter().map(|intersect| {
        let sum : u32 = intersect.iter().map(|item| {
                    //println!("{}", item);
                    match *item as u32 {
                        val if val < 93 => (val - 65) + 27,
                        val if val >= 93 => val - 96,
                        _ => 0,
                    }
                }).sum();
        sum as i64
    }).sum()
}
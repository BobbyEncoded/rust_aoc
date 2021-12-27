use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub fn parse (initial_contents : &String) -> HashMap<(i32, i32), u32> {
    let lines = initial_contents
        .lines()
        .map(|x| x.chars().enumerate().collect::<Vec<(usize, char)>>())
        .enumerate()
        .collect::<Vec<(usize, Vec<(usize, char)>)>>();
    let mut coordinates: HashMap<(i32, i32), u32> = HashMap::new();
    lines
        .iter()
        .for_each(|(x, y)| {
            y
            .iter()
            .for_each(|(y, c)| {
                coordinates
                .insert( (*x as i32, *y as i32), c.to_digit(10).expect("Character wasn't a num"));
            });
        });
    coordinates
}

pub fn part1 (mut input_map : HashMap<(i32, i32), u32>) -> u32 {
    let (mut minx, mut miny, mut maxx, mut maxy) = (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN);
    for keyval in &mut input_map {
        let ((x, y), _) = keyval;
        if x < &minx {
            minx = *x;
        };
        if x > &maxx {
            maxx = *x;
        };
        if y < &miny {
            miny = *y;
        };
        if y > &maxy {
            maxy = *y;
        };
    }
    for n in minx-1..maxx+1 {
        input_map.insert((n, miny-1), 10);
        input_map.insert((n, maxy+1), 10);
    }
    for n in miny..maxy {
        input_map.insert((minx-1, n), 10);
        input_map.insert((maxx+1, n), 10);
    }
    //The input map should now be prepared for searching.
    let mut sum = 0u32;
    for x in minx..maxx {
        for y in miny..maxy {
            let top =
                match &input_map.entry((x, y + 1)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            let bot =
                match &input_map.entry((x, y - 1)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            let right =
                match &input_map.entry((x + 1, y)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            let left =
                match &input_map.entry((x - 1, y)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            let cur_val =
                match &input_map.entry((x, y)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            if cur_val < top && cur_val < bot && cur_val < right && cur_val < left {
                sum = sum + cur_val + 1;
            }
            else {
                continue;
            }
        }
    }
    sum
}

pub fn run (initial_contents : &String) -> String {
    let initial_map = parse(initial_contents);
    format!("{}",part1(initial_map))
}
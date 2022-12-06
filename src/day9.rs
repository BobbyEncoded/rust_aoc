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
    for n in minx-1..=maxx+1 {
        input_map.insert((n, miny-1), 10);
        input_map.insert((n, maxy+1), 10);
    }
    for n in miny..=maxy {
        input_map.insert((minx-1, n), 10);
        input_map.insert((maxx+1, n), 10);
    }
    //The input map should now be prepared for searching.
    let mut sum = 0u32;
    for x in minx..=maxx {
        for y in miny..=maxy {
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

pub fn part2 (mut input_map : HashMap<(i32, i32), u32>) -> u32 {
    //Convert input_map to more suitable format for finding basins.

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
    for n in minx-1..=maxx+1 {
        input_map.insert((n, miny-1), 10);
        input_map.insert((n, maxy+1), 10);
    }
    for n in miny..=maxy {
        input_map.insert((minx-1, n), 10);
        input_map.insert((maxx+1, n), 10);
    }
    //Create the basin map based off the input map.
    let mut basin_map : HashMap<(i32, i32), (u32, u32)> = input_map.iter().map(|((x, y), val)| ((*x, *y), (*val, 0u32))).collect();
    //The input map should now be prepared for searching.
    let mut current_basin_index= 0u32;
    for x in minx..=maxx {
        for y in miny..=maxy {
            let top =
                match &basin_map.entry((x, y + 1)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            let bot =
                match &basin_map.entry((x, y - 1)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            let right =
                match &basin_map.entry((x + 1, y)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            let left =
                match &basin_map.entry((x - 1, y)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            let cur_val =
                match &basin_map.entry((x, y)) {
                Occupied(entry) => entry.get().clone(),
                Vacant(_) => unimplemented!(),
                };
            if cur_val.0 < top.0 && cur_val.0 < bot.0 && cur_val.0 < right.0 && cur_val.0 < left.0 {
                current_basin_index += 1;
                basin_map.entry((x, y)).or_insert((cur_val.0, current_basin_index));
            }
            else {
                continue;
            }
        }
    }
    //All the low points in the basin should now have been assigned an index.  We will need to start spreading them using a similar formula to loop through the cells and check whether the new map is different than the old map.
    fn update_map (minx : &i32, miny : &i32, maxx : &i32, maxy : &i32, map_to_update : &mut HashMap<(i32, i32), (u32, u32)>) {
        for x in *minx..=*maxx {
            for y in *miny..=*maxy {
                let top =
                    match &map_to_update.entry((x, y + 1)) {
                        Occupied(entry) => entry.get().clone(),
                        Vacant(_) => unimplemented!(),
                    };
                let bot =
                    match &map_to_update.entry((x, y - 1)) {
                        Occupied(entry) => entry.get().clone(),
                        Vacant(_) => unimplemented!(),
                    };
                let right =
                    match &map_to_update.entry((x + 1, y)) {
                        Occupied(entry) => entry.get().clone(),
                        Vacant(_) => unimplemented!(),
                    };
                let left =
                    match &map_to_update.entry((x - 1, y)) {
                        Occupied(entry) => entry.get().clone(),
                        Vacant(_) => unimplemented!(),
                    };
                let mut cur_val =
                    match map_to_update.entry((x, y)) {
                        Occupied(mut entry) => *entry.get_mut(),
                        Vacant(_) => unimplemented!(),
                    };
                if cur_val.0 < 9 {
                    if top.1 != 0 {
                        cur_val.1 = top.1;
                    } else if bot.1 != 0 {
                        cur_val.1 = bot.1;
                    } else if left.1 != 0 {
                        cur_val.1 = left.1;
                    } else if right.1 != 0 {
                        cur_val.1 = right.1;
                    }
                    continue;
                }
                else {
                    continue;
                }
            }
        }
    }

    for iteration in 0..100 {
        update_map(&minx, &miny, &maxx, &maxy, &mut basin_map);
        println!("Iteration: {}", iteration);
        for ((x, y), (val, index)) in &basin_map {
            println!("X: {}, Y: {}, Val: {}, Index: {}", x, y, val, index);
        }
    }
    //Map should now be fully updated.  Now we'll need to find the largest basins, and get their counts.
    //We'll need a function which will check each cell in the original map, and create a bin for the index of the bin, and then add 1 to it whenever it sees a new cell.  At the end, we should have a hashmap with all the bins.
    let mut basin_index_counts : HashMap<u32, u32> = HashMap::new();
    for x in minx..=maxx {
        for y in miny..=maxy {
            let (_, index) = 
                match &basin_map.entry((x, y)) {
                    Occupied(entry) => *entry.get(),
                    Vacant(_) => unimplemented!(),
                };
            *basin_index_counts.entry(index).or_insert(1) += 1;
        }
    }
    //basin_index_counts should now have proper counts.  We now need to find the 3 largest indices and multiply them together.
    let (mut largest, mut larger, mut large) = (0, 0, 0);
    for (_, value) in basin_index_counts {
        if value > large {
            if value > larger {
                if value > largest {
                    large = larger;
                    larger = largest;
                    largest = value;
                } else {
                    large = larger;
                    larger = value;
                }
            } else {
                large = value;
            }
        } else {
            continue;
        }
    }
    largest * larger * large
}

pub fn run (initial_contents : &String) -> String {
    let initial_map = parse(initial_contents);
    format!("{}",part2(initial_map))
}
pub fn run (initial_contents : &String) -> String {
    let initial_ints = parse(initial_contents);
    let min_int = *initial_ints.iter().min().unwrap();
    let max_int = *initial_ints.iter().max().unwrap();

    let fuel_range : Vec<i32> = (min_int..=max_int).collect();

    let map_fuel_and_position = |input_position : &i32| -> (i32, u64) {
        let fuel_cost : u64 = initial_ints
            .iter()
            .map(|val| _part_2_fuel_consumption((val-input_position).abs()))
            .sum();
        //println!("Current fuel cost: {}", fuel_cost);
        (*input_position, fuel_cost)
    };

    fn _part_2_fuel_consumption (input_fuel_cost : i32) -> u64 {
        ((input_fuel_cost * (input_fuel_cost + 1)) / 2) as u64
    }

    fn _part_1_fuel_consumption (input_fuel_cost : i32) -> u64 {
        input_fuel_cost as u64
    }

    let costs : Vec<(i32, u64)> = fuel_range.iter().map(map_fuel_and_position).collect();

    let (_, min_fuel) = costs.iter()
        .fold((0i32, u64::MAX), |(min_pos, min_fuel), (pos, fuel)| {
            if *fuel < min_fuel {
                (*pos, *fuel)
            } else {
                (min_pos, min_fuel)
            }
        });
    min_fuel.to_string()
}

fn parse (initial_contents : &String) -> Vec<i32> {
    initial_contents
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
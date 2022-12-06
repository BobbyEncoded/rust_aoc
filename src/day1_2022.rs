pub fn parse (initial_contents : &String) -> Vec<Vec<i64>> {
    let mut pancake_stack: Vec<Vec<i64>> = vec![vec![]];
    let mut_pancake_stack = &mut pancake_stack;
    {
        for line in initial_contents.lines() {
            if line.is_empty() {
               mut_pancake_stack.push(vec![]); 
            } else {
                let calories : i64 = line.parse().unwrap();
                let last_stack = mut_pancake_stack.last_mut().unwrap();
                last_stack.push(calories);
            }
        }
    }
    mut_pancake_stack.to_vec()
}

pub fn part1 (entry_data : Vec<Vec<i64>>)-> i64 {
    fn sum_vector (input_vec : Vec<i64>) -> i64 {
       input_vec.iter().sum()
    }
    let stacked_calories : Vec<i64> = entry_data.iter().map(|stack| sum_vector(stack.to_vec())).collect();
    *stacked_calories.iter().max().unwrap()
}

pub fn part2 (entry_data : Vec<Vec<i64>>)-> i64 {
    fn sum_vector (input_vec : Vec<i64>) -> i64 {
       input_vec.iter().sum()
    }
    let stacked_calories : Vec<i64> = entry_data.iter().map(|stack| sum_vector(stack.to_vec())).collect();
    let max1 = *stacked_calories.iter().max().clone().unwrap();
    let mut stacked_calories_2 = stacked_calories.clone();
    let index2 = stacked_calories_2.iter().position(|x| *x == max1).unwrap();
    stacked_calories_2.remove(index2);
    let max2 = *stacked_calories_2.iter().max().clone().unwrap();
    let mut stacked_calories_3 = stacked_calories_2.clone();
    let index3 = stacked_calories_3.iter().position(|x| *x == max2).unwrap();
    stacked_calories_3.remove(index3);
    let max3 = *stacked_calories_3.iter().max().clone().unwrap();
    max1 + max2 + max3
}
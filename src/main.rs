use std::io;
use rand::Rng;

// if position is out of bounds, dont consider it
fn get_next_position(position: (i32, i32)) -> (i32, i32) {
    let (x, y) = position;
    let mut possible_positions: Vec<(i32, i32)> = Vec::new();
    let mut next_x;
    let mut next_y;

    let coefficients = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (x_coefficient, y_coefficient) in coefficients.iter() {
        next_x = x + x_coefficient;
        next_y = y + y_coefficient;
        if next_x >= 0 && next_x < 3 && next_y >= 0 && next_y < 3 {
            possible_positions.push((next_x, next_y));
        }
    }
    
    // choose a random position from the possible positions
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible_positions.len());

    return possible_positions[index];
}

fn main() {
    let board = [[true,false,false], [false,false,false], [false,false,true]];
    // get user input, a number from 2 to 8
    let mut input = String::new();
    println!("Enter a number from 2 to 8: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: usize = input.trim().parse().expect("Please type a number!");
    // check if the number is valid
    if input < 2 || input > 8 {
        println!("Please enter a number between 2 and 8!");
        return;
    }
    
    let initial_position:(i32, i32);

    match input{
        2 => initial_position = (0,1),
        3 => initial_position = (0,2),
        4 => initial_position = (1,0),
        5 => initial_position = (1,1),
        6 => initial_position = (1,2),
        7 => initial_position = (2,0),
        8 => initial_position = (2,1),
        _ => {
            println!("Please enter a number between 2 and 8!");
            return;
        }
    }

    
    let mut count_top = 0;
    let mut count_bottom = 0;
    let mut jumps_mean = 0.0;
    let mut center_mean = 0.0;

    for _ in 0..1000000 {
        let mut current_position = initial_position;
        let mut jumps = 0;
        let mut centers = 0;
        while !board[current_position.0 as usize][current_position.1 as usize] {
            current_position = get_next_position(current_position);
            if current_position == (1,1) {
                centers += 1;
            }
            jumps += 1;
        }
        jumps_mean += jumps as f64;
        center_mean += centers as f64;
        if current_position.0 == 0 {
            count_top += 1;
        } else {
            count_bottom += 1;
        }
    }

    jumps_mean /= 1000000.0;
    center_mean /= 1000000.0;


    println!("Top: {}", count_top);
    println!("Bottom: {}", count_bottom);

    println!("Top as percentage: {}", (count_top as f64 / 1000000.0) * 100.0);
    println!("Bottom as percentage: {}", (count_bottom as f64 / 1000000.0) * 100.0);
    println!("Jumps mean: {}", jumps_mean);
    println!("Center mean: {}", center_mean);

    


}

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BinaryHeap;

fn original_fuel_consumption(positions: &Vec<u32>, assembly_point: i32) -> u32 {
    let mut fuel = 0 as u32;

    for crab in positions {
        let mut crab = *crab as i32;
        crab = crab - assembly_point;
        if crab < 0 {
            crab = 0 - crab;
        }
        fuel += crab as u32;
    }

    fuel
}

fn corrected_fuel_consumption(positions: &Vec<u32>, assembly_point: i32) -> u32 {
    let mut fuel = 0 as u32;

    for crab in positions {
        let mut crab = *crab as i32;
        crab = crab - assembly_point;
        if crab < 0 {
            crab = 0 - crab;
        }
        crab = ( crab * (crab + 1) ) / 2;
        fuel += crab as u32;
    }

    fuel
}

fn find_optimum_naive_corrected(positions: &Vec<u32>) -> (u32, u32) {
    let mut optimum = (0 as u32, 1000000000 as u32);
    let mut i = 0 as u32;
    for crab in positions {
        let fuel = corrected_fuel_consumption(positions, i as i32);
        if fuel < optimum.1 {
            optimum = (i, fuel);
        }
        i += 1;
    }
    optimum
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::from("");
    reader.read_line(&mut line).unwrap();
    line = String::from(line.trim());

    let mut positions: BinaryHeap<u32> = BinaryHeap::new();
    let mut average: f64 = 0.0;

    for mut element in line.split(',') {
        element = element.trim();
        let element: u32 = element.parse().unwrap();    // if this fails might as well panic
        positions.push(element);
        average += element as f64;
    }
    average /= positions.len() as f64;
    if (average - average.floor()) < 0.5 {
        average = average.floor();
    } else {
        average = average.ceil();
    }
    let average = average as u32;

    println!(" ----------------------");
    println!("    =============");
    println!("    =============");
    println!("    =============");
    println!(" ----------------------");

    let positions = positions.into_sorted_vec();

//    for (i, crab) in &mut positions.iter().enumerate() {
//        println!("{} --- {}", i, crab);
//    }

    let mut median: u32 = 0;
    println!("If we start counting the sorted crabs at 0, the median approach suggests that:");
    if positions.len() % 2 == 1 {
        let pivot = ( positions.len() + 1 ) / 2 - 1;
        println!("we should align at the position of the {}. crab", pivot);
        median = positions[pivot];
    } else {
        let pivot = positions.len() / 2;
        println!("we should align on the average postitions of the {}. and {}. crabs", pivot - 1, pivot);
        median = ( positions[pivot - 1] + positions[pivot] ) / 2;
    }

    println!("This means position {}", median);

    let mut fuel_consumption = original_fuel_consumption(&positions, median as i32);

    println!("This results in a total fuel consumption of {}", fuel_consumption);

    println!(" ----------------------");

    fuel_consumption = corrected_fuel_consumption(&positions, average as i32);

    println!("On the other hand:");
    println!("the average approach suggests we should gather at position {}", average);
    println!("which results in a fuel consumption of {}", fuel_consumption);

    println!(" ----------------------");

    println!("Exhaustively verifying this now...");

    let naive_solution = find_optimum_naive_corrected(&positions);

    println!("Which tells us that the optimal position is actually {}, which requires {} units of fuel", naive_solution.0, naive_solution.1);

}

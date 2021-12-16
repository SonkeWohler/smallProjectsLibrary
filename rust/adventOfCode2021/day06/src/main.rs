use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

struct Population {
    population: VecDeque<u64>,
}

impl Population {
    fn create(input: Vec<u64>) -> Population {
        Population {
            population: VecDeque::from(input),
        }
    }

    fn step(&mut self) {
        self.population.rotate_left(1);
        let mut seven_days_to_go = match self.population.remove(6) {
            None => panic!("VecDeque is not as long as it should be"),
            Some(value) => value,
        };
        match self.population.get(7) {
            None => panic!("VecDeque is not as long as it should be"),
            Some(value) => seven_days_to_go += value,
        };
        self.population.insert(6, seven_days_to_go);
    }

    fn total_fish(&self) -> u64 {
        let mut total_fish = 0;
        for i in &self.population {
            total_fish += i;
        }
        total_fish
    }

}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::from("");
    reader.read_line(&mut line).unwrap();
    line = String::from(line.trim());
    println!("{}", line);

    let mut population: Vec<u64> = vec![0; 9];

    for fish in line.split(',') {
        let index: usize = fish.parse().unwrap();
        population[index] += 1;
    }

    println!("{:?}", population);

    let mut population = Population::create(population);

    let max_days = 256;
    for _i in 0..max_days {
        population.step();
        println!("{:?}", population.population);
    }

    println!("In total there are {} fish after {} days", population.total_fish(), max_days);

}

use std::time::{Instant};
use std::hash::Hash;
use std::collections::HashMap;

extern crate crossbeam;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Fish {
    timer: usize
}

impl Fish {
    fn new(timer: usize) -> Fish {
        Fish { timer }
    }

    pub fn parse_input(input: String) -> Vec<Fish> {
        let separator = ",";

        println! {">>> Starting to parse input {} <<<", input}
        input.lines()
            .next()
            .unwrap()
            .split(separator)
            .map(|fish| Fish::new(fish.parse().unwrap()))
            .collect()
    }

    pub fn sim(fishes: Vec<Fish>, days: usize) -> usize {
        let new_fish = 8;
        let reset_fish = 6;
        let reproduce_fish = 0;

        let mut fish_pop = HashMap::new();

        for fish in fishes {
            *fish_pop.entry(fish).or_insert(0) += 1;
        }
        let starting_size: usize = fish_pop.values().sum();
        println!{"Starting fish pop: {} {:?}", starting_size, fish_pop};

        let now = Instant::now();
        for _day in 0..days {
            let mut new_fish_pop: HashMap<Fish, usize> = HashMap::new();
            for fish in &fish_pop {
                if fish.0.timer == reproduce_fish {
                    println!{"Day {} fish reproduced: {}", _day, *fish.1};
                    // Reproduce
                    // Get number of currently reset fish (age==6) and add number of reproducing fish
                    *new_fish_pop.entry(Fish::new(reset_fish)).or_insert(0) += *fish.1;
                    // Get number of new fish (age==8) and add number of reproducing fish
                    *new_fish_pop.entry(Fish::new(new_fish)).or_insert(0) += *fish.1;
                    continue
                }
                // Get number of fish with (age - 1) and add number of fish with target age
                *new_fish_pop.entry(Fish::new(fish.0.timer - 1)).or_insert(0) += *fish.1;
            }
            let fish_pop_size: usize = new_fish_pop.values().sum();
            println!{"Fish pop after day {}: {} {:?}", _day, fish_pop_size, new_fish_pop};
            fish_pop = new_fish_pop;
        }
        let duration = now.elapsed().as_secs();
        let result = fish_pop.values().sum();

        println!{"Evolution at day {} with {} fishes took {} secs", days, result, duration};
        result
    }
}

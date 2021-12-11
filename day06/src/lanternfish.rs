use std::fmt;
use std::fs;
use std::thread;
use std::time::{Instant};

extern crate crossbeam;

// #[derive(Clone)]
pub struct Population {
    pub fishes: Vec<usize>,
    pub current_day: usize
}

impl Population {
    fn new(fishes: Vec<usize>, current_day: usize) -> Population {
        Population {
            fishes: fishes,
            current_day: current_day,
        }
    }

    pub fn from_file(filename: &str) -> Population {
        let separator = ",";
        let starting_day = 0;

        println! {">>> Starting to parse file {} <<<", filename}
        let raw_data = fs::read_to_string(filename).unwrap();
        let raw_fishes: Vec<&str> = raw_data.split(separator).collect();

        Population::new(
            raw_fishes.iter().map(|f| f.parse::<usize>().unwrap()).collect(),
            starting_day
        )
    }

    pub fn evolute(&mut self) {
        let now = Instant::now();
        self.current_day += 1;
        let new_fish = 8;
        let reset_fish = 6;
        for i in 0..self.fishes.len() {
            if self.fishes[i] == 0 {
                // Reproduce
                self.fishes[i] = reset_fish;
                self.fishes.push(new_fish);
            } else {
                self.fishes[i] -= 1;
            }
        }
        println!{"Evolution of day {} with {} fishes took {} secs", self.current_day, self.size(), now.elapsed().as_secs()};
    }

    // pub fn evolute_concurrently(&self) -> Population {
    //     // Update the day
    //     let new_day = self.current_day + 1;

    //     // Prepare multithreading
    //     let cores = 24; // 24 cores
    //     let chunk_size = (self.size() / cores) + 1;

    //     println!{"Splitting population ({}) into {} chunks of size {}", self.size(), cores, chunk_size};
    //     // let chunks = self.fishes.chunks(chunk_size);
    //     // println!{"Chunks: {:?}", chunks};
        
    //     // Spawn threads and start measuring duration
    //     let mut results = Vec::new();
    //     let now = Instant::now();
    //     let _ = crossbeam::scope(|scope| {
    //         for slice in self.fishes.chunks_mut(chunk_size) {
    //             let mut result = Vec::new();
    //             scope.spawn( move |_| result = Population::evolute_population(slice.to_vec()));
    //         }
    //     });

    //     // for chunk in chunks {
    //     //     threads.push(thread::spawn(|| {
    //     //         Population::evolute_population(chunk.to_vec())
    //     //     }));

    //     // }

    //     // Wait for threads to finish and collect result populations in single vec
    //     let mut new_pop = Vec::new();
    //     // for thread in threads {
    //     //     new_pop.append(&mut thread.join().unwrap());
    //     // }

    //     println!{
    //         "Concurrent evolution of day {} with {} fishes took {} secs",
    //         new_day, new_pop.len(), now.elapsed().as_secs()
    //     };

    //     Population::new(new_pop, new_day)
    // }

    fn evolute_population(init_pop: Vec<usize>) -> Vec<usize> {
        // Constants for fish evolution
        let new_fish = 8;
        let reset_fish = 6;
        // Fish evolution
        let mut new_pop = Vec::new();
        for i in 0..init_pop.len() {
            if init_pop[i] == 0 {
                // Reproduce
                new_pop.push(reset_fish);
                new_pop.push(new_fish);
            } else {
                // Age
                new_pop.push(init_pop[i] - 1);
            }
        }
        new_pop
    }


    pub fn size(&self) -> usize {
        self.fishes.len()
    }

    pub fn sum(&self) -> usize {
        self.fishes.iter().fold(0, |sum, i| sum + i)
    }
}

impl fmt::Display for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.current_day {
            0 => write!{f, "Initial state: ",}.unwrap(),
            _ => write!{f, "After {:2} days: ", self.current_day}.unwrap(),
        };
        let fish_strings: Vec<String> = self.fishes.iter().map(|f| f.to_string()).collect(); 
        write!{f, "{}", fish_strings.join(",")}
    }
}

// struct Lanternfish {
//     age: usize,
// }

// impl Lanternfish {
//     fn new(age: usize) -> Lanternfish {
//         Lanternfish {
//             age: age,
//         }
//     }
// }

// impl fmt::Display for Lanternfish {

// }

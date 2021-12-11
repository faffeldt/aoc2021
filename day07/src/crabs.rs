use std::hash::Hash;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Crab {
    h_pos: usize,
}

impl Crab {
    fn new(h_pos: usize) -> Crab {
        Crab { 
            h_pos: h_pos,
        }
    }

    pub fn parse_input(input: String) -> Vec<Crab> {
        let separator = ",";

        println! {">>> Starting to parse input {} <<<", input}
        input.lines()
            .next()
            .unwrap()
            .split(separator)
            .map(|crab| Crab::new(crab.parse().unwrap()))
            .collect()
    }

    fn fuel_cost(&self, target_h_pos: usize) -> isize {
        let result = (self.h_pos as isize - target_h_pos as isize).abs();
        // println!{"{} -> {}: {}", self.h_pos, target_h_pos, result};
        result
    }

    pub fn calculate_fuel_cost(crabs: Vec<Crab>, target_h_pos: usize) -> isize {
        crabs.iter()
            .map(|c| c.fuel_cost(target_h_pos))
            .fold(0, |i, sum| sum + i)
    }

    pub fn find_cheapest_alignment_point(crabs: Vec<Crab>) -> (isize, usize) {
        let mut min_h_pos = 999999999;
        let mut min_fuel = 999999999;

        let min_pos = crabs.iter().min_by_key(|c| c.h_pos).unwrap().h_pos;
        let max_pos = crabs.iter().max_by_key(|c| c.h_pos).unwrap().h_pos;

        // println!{"Trying positions between {} and {}", min_pos, max_pos}
        for tmp_hpos in min_pos..max_pos+1 {
            let tmp_fuel = Crab::calculate_fuel_cost(crabs.clone(), tmp_hpos);
            // println!{"Fuel cost at {}: {} (min_fuel={}, min_h_pos={})", tmp_hpos, tmp_fuel, min_fuel, min_h_pos};
            if tmp_fuel < min_fuel {
                min_fuel = tmp_fuel;
                min_h_pos = tmp_hpos;
            }
        }

        (min_fuel, min_h_pos)
    }

    fn fuel_cost_correct(&self, target_h_pos: usize) -> isize {
        // 16 -> 5
        // 15: 11
        // 14: 10
        // 13:  9
        // ...
        //  5:  1
        // (n * (n + 1)) / 2
        let diff = (self.h_pos as isize - target_h_pos as isize).abs();
        let result = (diff * (diff + 1)) / 2;
        // println!{"{} -> {}: {}", self.h_pos, target_h_pos, result};
        result
    }

    pub fn calculate_fuel_cost_correct(crabs: Vec<Crab>, target_h_pos: usize) -> isize {
        crabs.iter()
            .map(|c| c.fuel_cost_correct(target_h_pos))
            .fold(0, |i, sum| sum + i)
    }

    pub fn find_cheapest_alignment_point_correct(crabs: Vec<Crab>) -> (isize, usize) {
        let mut min_h_pos = 999999999;
        let mut min_fuel = 999999999;

        let min_pos = crabs.iter().min_by_key(|c| c.h_pos).unwrap().h_pos;
        let max_pos = crabs.iter().max_by_key(|c| c.h_pos).unwrap().h_pos;

        // println!{"Trying positions between {} and {}", min_pos, max_pos}
        for tmp_hpos in min_pos..max_pos+1 {
            let tmp_fuel = Crab::calculate_fuel_cost_correct(crabs.clone(), tmp_hpos);
            // println!{"Fuel cost at {}: {} (min_fuel={}, min_h_pos={})", tmp_hpos, tmp_fuel, min_fuel, min_h_pos};
            if tmp_fuel < min_fuel {
                min_fuel = tmp_fuel;
                min_h_pos = tmp_hpos;
            }
        }

        (min_fuel, min_h_pos)
    }
}

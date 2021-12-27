use std::fmt;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct PolymerTemplate {
    pub states: Vec<Vec<char>>,
    pub insertion_rules: HashMap<(char, char), char>,
}

impl PolymerTemplate {
    pub fn new(
        initial_state: Vec<char>,
        insertion_rules: HashMap<(char, char), char>
    ) -> PolymerTemplate {
        PolymerTemplate {
            states: vec![initial_state],
            insertion_rules: insertion_rules,
        }
    }

    pub fn parse_input(input: &str) -> PolymerTemplate {
        println! {">>> Starting to parse input <<<"};

        let mut insertion_rules: HashMap<(char, char), char> = HashMap::new();
        let mut initial_state: Vec<char> = Vec::new();
        input.lines().for_each(|l| {
            if l.len() < 4 {
                // Ignore split line
            } else if l.contains("->") {
                // lines containing insertion rules like:
                // CH -> B
                let raw_instr: Vec<&str> = l
                    .split(" -> ")
                    .collect::<Vec<&str>>();
                let mut pattern = raw_instr[0].chars();
                let insertion_character = raw_instr[1].chars().next().unwrap();
                insertion_rules
                    .entry((pattern.next().unwrap(), pattern.next().unwrap()))
                    .or_insert(insertion_character);
            } else {
                // Base template
                initial_state = l.to_string().chars().collect();
            }
        });
        
        PolymerTemplate::new(initial_state, insertion_rules)
    }

    pub fn build_polymer(&mut self) {
        // println! {"DEBUG> Start building polymer"};
        let last_state = self.states.last().unwrap();
        let mut new_state: Vec<char> = Vec::new();

        last_state.windows(2).for_each(
            |window| {
                let first = window[0];
                let second = window[1];
                // new_state.push(first);

                match self.insertion_rules.get(&(first, second)) {
                    Some(i) => {
                        new_state.push(first);
                        new_state.push(*i);
                    },
                    None    => new_state.push(first),
                }
            }
        );
        new_state.push(*last_state.last().unwrap());
        // self.states.push();
        self.states = vec![new_state];
        // println! {"DEBUG> Finished building polymer\n\t{}", self.states[0].iter().collect::<String>()};
    }

    pub fn build_polymer_n_times(&mut self, n: usize) {
        println!{"DEBUG> Starting to perform {} steps", n};
        let now = Instant::now();
        for i in 0..n {
            let now2 = Instant::now();
            self.build_polymer();
            println!{"DEBUG> Finished step {} with {} chars in polymer in {} secs (Total: {} secs)",
                      i, self.states.last().unwrap().len(), now2.elapsed().as_secs(), now.elapsed().as_secs()};
        }
        println!{"DEBUG> Finished performing {} steps in {} sec", n, now.elapsed().as_secs()};
    }

    pub fn score_of_last(&self) -> usize {
        let last_polymer = self.states.last().unwrap();
        let mut counts: HashMap<char, usize> = HashMap::new();
        last_polymer.iter()
            .for_each(|c| *counts.entry(*c).or_insert(1) += 1);
        
        println!{"DEBUG> Counts of last: {:?}", counts};
        let mut min_count = 999999;
        let mut max_count = 0;
        for (_, count) in counts {
            if count < min_count {min_count = count};
            if count > max_count {max_count = count};
        }
        max_count - min_count
    }
}

impl fmt::Display for PolymerTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln! {f, "PolymerTemplate insertion rules: {:?} and states:", self.insertion_rules};
        for (i, state) in self.states.iter().enumerate() {
            writeln!{f, "After step {}: {}", i, state.into_iter().collect::<String>()};
        }
        write!{f, ""}
    }
}

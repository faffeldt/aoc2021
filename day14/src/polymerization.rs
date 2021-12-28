use std::collections::BTreeMap;
use std::fmt;
use std::mem;
use std::time::{Duration, Instant};

pub struct PolymerTemplate {
    pub counts: BTreeMap<char, usize>,
    pub pairs: BTreeMap<(char, char), usize>,
    pub insertion_rules: BTreeMap<(char, char), char>,
}

impl PolymerTemplate {
    pub fn new(
        counts: BTreeMap<char, usize>,
        pairs: BTreeMap<(char, char), usize>,
        insertion_rules: BTreeMap<(char, char), char>,
    ) -> PolymerTemplate {
        PolymerTemplate {
            counts: counts,
            pairs: pairs,
            insertion_rules: insertion_rules,
        }
    }

    pub fn parse_input(input: &str) -> PolymerTemplate {
        println! {">>> Starting to parse input <<<"};

        let mut insertion_rules: BTreeMap<(char, char), char> = BTreeMap::new();
        let mut counts: BTreeMap<char, usize> = BTreeMap::new();
        let mut pairs: BTreeMap<(char, char), usize> = BTreeMap::new();

        input.lines().for_each(|l| {
            if l.len() < 4 {
                // Ignore split line
            } else if l.contains("->") {
                // lines containing insertion rules like:
                // CH -> B
                let raw_instr: Vec<&str> = l.split(" -> ").collect::<Vec<&str>>();
                let mut pattern = raw_instr[0].chars();
                let insertion_character = raw_instr[1].chars().next().unwrap();
                insertion_rules
                    .entry((pattern.next().unwrap(), pattern.next().unwrap()))
                    .or_insert(insertion_character);
            } else {
                // Base template
                let chars = l.to_string().chars().collect::<Vec<char>>();
                chars
                    .iter()
                    .for_each(|c| *counts.entry(*c).or_insert(0) += 1);
                chars
                    .windows(2)
                    .for_each(|w| *pairs.entry((w[0], w[1])).or_insert(0) += 1);
            }
        });

        PolymerTemplate::new(counts, pairs, insertion_rules)
    }

    pub fn build_polymer(&mut self) {
        println! {"DEBUG> Start building polymer"};
        let mut new_state: BTreeMap<(char, char), usize> = BTreeMap::new();
        for ((c1, c2), num) in &self.pairs {
            let ic = self.insertion_rules.get(&(*c1, *c2)).unwrap();
            // Update the counts of the cells in polymer with inserted char
            *self.counts.entry(*ic).or_insert(0) += num;

            // Update the number of pairs in new state
            *new_state.entry((*c1, *ic)).or_insert(0) += num;
            *new_state.entry((*ic, *c2)).or_insert(0) += num;

            // println!{
            //     "DEBUG>\tPair={:?} -> ({}, {}) and ({}, {})\n\tCounter: {:?}",
            //     ((c1, c2), num), c1, ic, ic, c2,
            //     new_state,
            // };
        }
        self.pairs = new_state;
    }

    pub fn build_polymer_n_times(&mut self, n: usize) {
        println! {"DEBUG> Starting to perform {} steps", n};
        let now = Instant::now();
        for i in 0..n {
            self.build_polymer();
            println! {"DEBUG> Polymer after step {}: counts={:?} pairs={:?}", i, self.counts, self.pairs};
        }
        println! {"DEBUG> Finished performing {} steps in {} sec", n, now.elapsed().as_secs()};
    }

    pub fn score_of_last(&self) -> usize {
        let mut min_count = usize::MAX;
        let mut max_count = 0;

        for (_, count) in &self.counts {
            if *count < min_count {
                min_count = *count
            };
            if *count > max_count {
                max_count = *count
            };
        }
        max_count - min_count
    }
}

impl fmt::Display for PolymerTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln! {f,
        "PolymerTemplate counts: {:?}\n\tstates: {:?}\n\tinsertion rules: {:?}",
        self.counts, self.pairs, self.insertion_rules};
        // for (i, state) in self.states.iter().enumerate() {
        //     writeln!{f, "After step {}: {}", i, state.into_iter().collect::<String>()};
        // }
        write! {f, ""}
    }
}

use std::collections::BTreeMap;
use std::fmt;
use std::mem;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct RiskFactorMap {
    pub cells: BTreeMap<(usize, usize), usize>,
}

impl RiskFactorMap {
    pub fn new() -> RiskFactorMap {
        RiskFactorMap {
            cells: BTreeMap::new(),
        }
    }

    pub fn parse_input(input: &str) -> RiskFactorMap {
        RiskFactorMap::new()
    }
}

impl std::cmp::PartialEq for RiskFactorMap {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}

#[cfg(test)]
mod tests {
    use crate::chiton::RiskFactorMap;
    use std::fs;
    #[test]
    fn test_parse_input() {
        let input_file = ".\\solution_input.txt";
        let input = fs::read_to_string(&input_file)
            .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
        let caves = RiskFactorMap::parse_input(&input);
        let expected_caves = RiskFactorMap::new();
        assert_eq!(caves, expected_caves);
    }
}

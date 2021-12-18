use std::collections::{HashMap, HashSet};
use std::fmt;


//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
// ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// |-------------- all ten unique signal patterns -------------| -- four digit output value -- |
//  acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Measurement {
    unique_signal_patterns: Vec<String>,
    output_values: Vec<String>,
}

impl Measurement {
    fn new(unique_signal_patterns: Vec<String>, output_values: Vec<String>) -> Measurement {
        Measurement { 
            unique_signal_patterns: unique_signal_patterns,
            output_values: output_values,
        }
    }

    pub fn parse_input(input: String) -> Vec<Measurement> {
        let signal_sep = " ";
        let output_sep = " | ";

        println! {">>> Starting to parse input <<<"}
        let mut result: Vec<Measurement> = Vec::new();
        let lines = input.lines();
        for line in lines {
            let raw_segments: Vec<&str> = line.split(output_sep).collect();
            let raw_signal_patterns: Vec<&str> = raw_segments[0].split(signal_sep).collect();
            let raw_output_values: Vec<&str> = raw_segments[1].split(signal_sep).collect();
            result.push(
                Measurement::new(
                    raw_signal_patterns.iter().map(|p| String::from(*p)).collect(),
                    raw_output_values.iter().map(|p| String::from(*p)).collect()
                )
            )
        }
        result
    }

    pub fn count_1478_in_output(&self) -> usize {
        // // Find number of output values corresponding to "1" (2 bits, min)
        let num_1_entries: Vec<String> = self.output_values.clone().into_iter()
            .filter(|v| v.len() == 2)
            .collect();
        let num_1 = num_1_entries.len();
        
        // Find number of output values corresponding to "4" (4 bits)
        let num_4_entries: Vec<String> = self.output_values.clone().into_iter()
            .filter(|v| v.len() == 4)
            .collect();
        let num_4 = num_4_entries.len();
        
        // Find number of output values corresponding to "7" (3 bits)
        let num_7_entries: Vec<String> = self.output_values.clone().into_iter()
            .filter(|v| v.len() == 3)
            .collect();
        let num_7 = num_7_entries.len();
        
        // Find number of output values corresponding to "8" (7 bits, max)
        let num_8_entries: Vec<String> = self.output_values.clone().into_iter()
            .filter(|v| v.len() == 7)
            .collect();
        let num_8 = num_8_entries.len();

        println!{
            "DEBUG> Num1: {} {:?}; Num4: {} {:?}; Num7: {} {:?}; Num8: {} {:?}",
            num_1, num_1_entries, num_4, num_4_entries, num_7, num_7_entries, num_8, num_8_entries
        }

        num_1 + num_4 + num_7 + num_8
    }

    pub fn decode_output(&self) -> u32 {
        // Find one and four representation in unique values
        let unique_patterns = self.unique_signal_patterns.clone();
        let one = unique_patterns.iter()
            .find(|s| s.len() == 2).unwrap();
        let four = unique_patterns.iter()
            .find(|s| s.len() == 4).unwrap();
        
        // Easy numbers:
        //   1:      4:      7:      8:   
        //  ....    ....    aaaa    aaaa  
        // .    c  b    c  .    c  b    c 
        // .    c  b    c  .    c  b    c 
        // ....    dddd     ....    dddd  
        // .    f  .    f  .    f  e    f 
        // .    f  .    f  .    f  e    f 
        //  ....    ....    ....    gggg  
        
        // Hard numbers:
        //   0:      2:      3:      5:      6:      9:
        //  aaaa    aaaa    aaaa    aaaa    aaaa    aaaa
        // b    c  .    c  .    c  b    .  b    .  b    c
        // b    c  .    c  .    c  b    .  b    .  b    c
        // ....    dddd    dddd     dddd    dddd    dddd
        // e    f  e    .  .    f  .    f  e    f  .    f
        // e    f  e    .  .    f  .    f  e    f  .    f
        //  gggg    gggg    gggg    gggg    gggg    gggg
        //
        //  6,2,3   5,1,2   5,2,3   5,1,3   6,1,3   6,2,4
        let decoded_output: Vec<u32> = self.output_values.iter()
            .map(|o| match o.len() {
                // Simple bits because unique length for 1, 4, 7, 8 (part A)
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                // Complex matching for rest of numbers based on length of input
                // and how many other characters are contained
                len => match (
                    len,
                    o.chars().filter(|&b| one.contains(b)).count(),
                    o.chars().filter(|&b| four.contains(b)).count(),
                ) {
                    (6, 2, 3) => 0,
                    (5, 1, 2) => 2,
                    (5, 2, 3) => 3,
                    (5, 1, 3) => 5,
                    (6, 1, 3) => 6,
                    (6, 2, 4) => 9,
                    _ => unreachable!(),
                },
            }).collect();
        // println!{"DEBUG> Input: {} -> {:?}", self, decoded_output};
        // Add output numbers together with decreasing power of 10
        decoded_output.iter().enumerate()
            .fold(0, |acc, (i, n)| acc + n * 10_u32.pow(3 - i as u32))
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "{} | {}", self.unique_signal_patterns.join(" "), self.output_values.join(" ")}
    }
}

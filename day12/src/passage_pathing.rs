use std::collections::{BTreeMap, HashMap};
use std::fmt;

pub struct CaveMap<'a> {
    connections: BTreeMap<&'a str, Vec<&'a str>>,
}

// Map a -> [b, c], b -> [d, end], ....
impl<'a> CaveMap<'a> {
    pub fn new<'b>(connections: BTreeMap<&'b str, Vec<&'b str>>) -> CaveMap<'b> {
        CaveMap {
            connections: connections,
        }
    }

    pub fn parse_input(input: &'a str) -> BTreeMap<&'a str, Vec<&'a str>> {
        println! {">>> Starting to parse input <<<"};

        let raw_connections = input
            .lines()
            .map(|l| l.split("-").collect())
            .collect::<Vec<Vec<&str>>>();

        let mut connections: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for rc in &raw_connections {
            let begin = rc[0];
            let end = rc[1];

            // Create bidirectional connections between caves
            connections.entry(begin).or_insert(vec![]).push(end);
            connections.entry(end).or_insert(vec![]).push(begin);
        }

        connections
    }

    pub fn find_all_paths(
        &self,
        start: &str,
        end: &str,
        max_visits_to_small_caves: usize,
    ) -> Vec<Vec<&str>> {
        println! {"DEBUG> Starting to find all paths"}
        let mut paths: Vec<Vec<&str>> = vec![vec!["start"]];
        let mut continue_pathing = true;
        let mut iteration = 0;

        while continue_pathing {
            iteration += 1;
            println! {"DEBUG> Starting pathing iteration {} with {} paths", iteration, paths.len()};
            let mut new_paths = Vec::new();
            let mut tmp_continue_pathing = false;
            // Iterate all paths and branch them out if "end" is not reached yet
            for path in paths {
                let last_cave = path.last().unwrap();
                let path_ended = *last_cave == end;
                if path_ended {
                    // Just add finished path to new_paths
                    new_paths.push(path.to_vec());
                } else {
                    // If not finished get all possible connections of last cave visited
                    let connected_caves = self.connections.get(last_cave).unwrap();
                    for next_cave in connected_caves {
                        // Create a new path for all possible new connections
                        // but only if not cave is big or not visited yet
                        let is_big_cave = next_cave
                            .chars()
                            .fold(true, |acc, c| acc && c.is_uppercase());
                        let small_cave_visits_unfiltered =
                            path.iter().fold(HashMap::<&str, usize>::new(), |mut m, x| {
                                *m.entry(x).or_default() += 1;
                                m
                            });
                        let small_cave_visits = small_cave_visits_unfiltered
                            .into_iter()
                            .filter(|(k, v)|
                                // Not start or end cave
                                *k != start && *k != end &&
                                // Is small cave
                                k.chars().fold(true, |acc, c| acc && c.is_lowercase()) &&
                                // Has been visited more than max_visits_to_small_caves
                                *v >= max_visits_to_small_caves)
                            .collect::<Vec<(&str, usize)>>();
                        let path_it = is_big_cave
                            || *next_cave == end
                            || !path.contains(next_cave)
                            || small_cave_visits.len() == 0;
                        // println!{"DEBUG> Path {:?} last={} next={} is_big={} small_cave_visits={:?} -> path_it={}", path, last_cave, next_cave, is_big_cave, small_cave_visits, path_it};
                        if *next_cave != start && path_it {
                            let mut path_copy = path.to_vec();
                            path_copy.push(next_cave);
                            new_paths.push(path_copy);
                            tmp_continue_pathing |= true;
                        }
                    }
                }
            }
            continue_pathing = tmp_continue_pathing;
            paths = new_paths;
        }

        println! {"DEBUG> Finished pathing. Current paths:"};
        for p in &paths {
            println! {"\t{:?}", p}
        }
        paths
    }
}

impl fmt::Display for CaveMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "Connections: {:?}", self.connections}
    }
}

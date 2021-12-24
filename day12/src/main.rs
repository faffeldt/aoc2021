use std::env;
use std::fs;
mod passage_pathing;
use crate::passage_pathing::CaveMap;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a_small();
    local_test_a_big();
    solution_a();
    local_test_b_small();
    local_test_b_big();
    solution_b();
}

fn local_test_a_small() {
    let input_file = ".\\test_input_small.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let connections = CaveMap::parse_input(&input);
    let cave_map = CaveMap::new(connections);
    println! {"TEST A Small> Input:\n{}", cave_map};

    let mut paths = cave_map.find_all_paths("start", "end", 1);

    // Map of connections:
    //      start
    //      /   \
    //  c--A-----b--d
    //      \   /
    //       end
    let mut expected_paths = vec![
        vec!["start", "A", "b", "A", "c", "A", "end"],
        vec!["start", "A", "b", "A", "end"],
        vec!["start", "A", "b", "end"],
        vec!["start", "A", "c", "A", "b", "A", "end"],
        vec!["start", "A", "c", "A", "b", "end"],
        vec!["start", "A", "c", "A", "end"],
        vec!["start", "A", "end"],
        vec!["start", "b", "A", "c", "A", "end"],
        vec!["start", "b", "A", "end"],
        vec!["start", "b", "end"],
    ];
    assert_eq! {expected_paths.len(), paths.len(), "TEST A Small> Number of Paths mismatches"}
    assert_eq! {expected_paths.sort_unstable(), paths.sort_unstable(), "TEST A Small> Paths mismatch"}
}

fn local_test_a_big() {
    let input_file = ".\\test_input_big.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let connections = CaveMap::parse_input(&input);
    let cave_map = CaveMap::new(connections);
    println! {"TEST A Big> Input:\n{}", cave_map};

    let mut paths = cave_map.find_all_paths("start", "end", 1);

    let mut expected_paths = vec![
        vec!["start", "HN", "dc", "HN", "end"],
        vec!["start", "HN", "dc", "HN", "kj", "HN", "end"],
        vec!["start", "HN", "dc", "end"],
        vec!["start", "HN", "dc", "kj", "HN", "end"],
        vec!["start", "HN", "end"],
        vec!["start", "HN", "kj", "HN", "dc", "HN", "end"],
        vec!["start", "HN", "kj", "HN", "dc", "end"],
        vec!["start", "HN", "kj", "HN", "end"],
        vec!["start", "HN", "kj", "dc", "HN", "end"],
        vec!["start", "HN", "kj", "dc", "end"],
        vec!["start", "dc", "HN", "end"],
        vec!["start", "dc", "HN", "kj", "HN", "end"],
        vec!["start", "dc", "end"],
        vec!["start", "dc", "kj", "HN", "end"],
        vec!["start", "kj", "HN", "dc", "HN", "end"],
        vec!["start", "kj", "HN", "dc", "end"],
        vec!["start", "kj", "HN", "end"],
        vec!["start", "kj", "dc", "HN", "end"],
        vec!["start", "kj", "dc", "end"],
    ];
    assert_eq! {expected_paths.len(), paths.len(), "TEST A Big> Number of Paths mismatches"}
    assert_eq! {expected_paths.sort_unstable(), paths.sort_unstable(), "TEST A Big> Paths mismatch"}
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));

    let connections = CaveMap::parse_input(&input);
    let cave_map = CaveMap::new(connections);
    println! {"SOLUTION A Big> Input:\n{}", cave_map};

    let paths = cave_map.find_all_paths("start", "end", 1);
    println! {"Solution A> Number of paths: {}", paths.len()};
}

fn local_test_b_small() {
    let input_file = ".\\test_input_small.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let connections = CaveMap::parse_input(&input);
    let cave_map = CaveMap::new(connections);
    println! {"TEST B Small> Input:\n{}", cave_map};

    let mut paths = cave_map.find_all_paths("start", "end", 2);

    // Map of connections:
    //      start
    //      /   \
    //  c--A-----b--d
    //      \   /
    //       end
    let mut expected_paths = vec![
        vec!["start", "A", "b", "A", "b", "A", "c", "A", "end"],
        vec!["start", "A", "b", "A", "b", "A", "end"],
        vec!["start", "A", "b", "A", "b", "end"],
        vec!["start", "A", "b", "A", "c", "A", "b", "A", "end"],
        vec!["start", "A", "b", "A", "c", "A", "b", "end"],
        vec!["start", "A", "b", "A", "c", "A", "c", "A", "end"],
        vec!["start", "A", "b", "A", "c", "A", "end"],
        vec!["start", "A", "b", "A", "end"],
        vec!["start", "A", "b", "d", "b", "A", "c", "A", "end"],
        vec!["start", "A", "b", "d", "b", "A", "end"],
        vec!["start", "A", "b", "d", "b", "end"],
        vec!["start", "A", "b", "end"],
        vec!["start", "A", "c", "A", "b", "A", "b", "A", "end"],
        vec!["start", "A", "c", "A", "b", "A", "b", "end"],
        vec!["start", "A", "c", "A", "b", "A", "c", "A", "end"],
        vec!["start", "A", "c", "A", "b", "A", "end"],
        vec!["start", "A", "c", "A", "b", "d", "b", "A", "end"],
        vec!["start", "A", "c", "A", "b", "d", "b", "end"],
        vec!["start", "A", "c", "A", "b", "end"],
        vec!["start", "A", "c", "A", "c", "A", "b", "A", "end"],
        vec!["start", "A", "c", "A", "c", "A", "b", "end"],
        vec!["start", "A", "c", "A", "c", "A", "end"],
        vec!["start", "A", "c", "A", "end"],
        vec!["start", "A", "end"],
        vec!["start", "b", "A", "b", "A", "c", "A", "end"],
        vec!["start", "b", "A", "b", "A", "end"],
        vec!["start", "b", "A", "b", "end"],
        vec!["start", "b", "A", "c", "A", "b", "A", "end"],
        vec!["start", "b", "A", "c", "A", "b", "end"],
        vec!["start", "b", "A", "c", "A", "c", "A", "end"],
        vec!["start", "b", "A", "c", "A", "end"],
        vec!["start", "b", "A", "end"],
        vec!["start", "b", "d", "b", "A", "c", "A", "end"],
        vec!["start", "b", "d", "b", "A", "end"],
        vec!["start", "b", "d", "b", "end"],
        vec!["start", "b", "end"],
    ];

    let too_much_paths = paths
        .iter()
        .filter(|p| !expected_paths.contains(p))
        .collect::<Vec<_>>();
    println! {"TEST B Small> Too much paths:"};
    for tmp in too_much_paths {
        println! {"\t{:?}", tmp};
    }

    let expected_num_paths = 36;
    assert_eq! {expected_num_paths, expected_paths.len(), "TEST B Small> Expected Number of Paths mismatches calculated length"}
    assert_eq! {expected_paths.len(), paths.len(), "TEST B Small> Number of Paths mismatches"}
    assert_eq! {expected_paths.sort_unstable(), paths.sort_unstable(), "TEST B Small> Paths mismatch"}
    println! {"TEST B Small> Finished"};
}

fn local_test_b_big() {
    let input_file = ".\\test_input_big.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let connections = CaveMap::parse_input(&input);
    let cave_map = CaveMap::new(connections);
    println! {"TEST B Big> Input:\n{}", cave_map};

    let paths = cave_map.find_all_paths("start", "end", 2);

    let expected_num_paths = 103;
    assert_eq! {expected_num_paths, paths.len(), "TEST B Big> Number of Paths mismatches"}
    println! {"TEST B Big> Finished"};
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let connections = CaveMap::parse_input(&input);
    let cave_map = CaveMap::new(connections);
    println! {"SOLUTION B> Input:\n{}", cave_map};

    let paths = cave_map.find_all_paths("start", "end", 2);
    println! {"Solution B> Number of paths: {}", paths.len()};
}

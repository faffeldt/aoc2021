#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs;

    #[test]
    fn local_test() {
        let filepath = Path::new("../test_input.txt");
        let items = fs::read_to_string(filepath);
        println!("Test input: {:?}", items)
    }

    #[test]
    fn solution() {
        let filepath = Path::new("../solution_input.txt");
        let items = fs::read_to_string(filepath);
        println!("Solution input: {:?}", items)
    }
}

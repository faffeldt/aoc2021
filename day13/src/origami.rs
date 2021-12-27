use std::fmt;

pub struct Origami {
    points: Vec<(usize, usize)>,
    instructions: Vec<(usize, String, usize)>,
    width: usize,
    height: usize,
}

impl Origami {
    pub fn new(
        points: Vec<(usize, usize)>,
        instructions: Vec<(usize, String, usize)>,
        width: usize,
        height: usize,
    ) -> Origami {
        Origami {
            points: points,
            instructions: instructions,
            width: width,
            height: height,
        }
    }

    pub fn parse_input(input: &str) -> Origami {
        println! {">>> Starting to parse input <<<"};

        let mut points: Vec<(usize, usize)> = Vec::new();
        let mut instructions: Vec<(usize, String, usize)> = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut instruction_counter = 0;
        input.lines().for_each(|l| {
            if l.contains(",") {
                let coords: Vec<&str> = l.split(",").collect();
                let x = coords[0].parse::<usize>().unwrap();
                let y = coords[1].parse::<usize>().unwrap();
                if x > width {
                    width = x
                };
                if y > height {
                    height = y
                };
                points.push((x, y));
            } else if l.contains("fold") {
                let raw_instr: Vec<&str> = l
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .split("=")
                    .collect();
                instructions.push((
                    instruction_counter,
                    String::from(raw_instr[0]),
                    raw_instr[1].parse::<usize>().unwrap(),
                ));
                instruction_counter += 1;
            } else {
                // Ignore split
            }
        });
        instructions.reverse();
        // points.sort_unstable();
        points.sort_unstable_by_key(|p| p.0);
        points.sort_unstable_by_key(|p| p.1);
        Origami::new(points, instructions, width, height)
    }

    pub fn num_of_points(&self) -> usize {
        self.points.len()
    }

    pub fn draw(&self) {
        let mut map: Vec<Vec<&str>> = vec![vec!["."; self.width + 1]; self.height + 1];
        for (x, y) in &self.points {
            map[*y][*x] = "#";
        }

        for row in map {
            print! {"\t"};
            for point in row {
                print! {"{}", point};
            }
            println! {""};
        }
    }

    pub fn fold(&mut self) {
        println! {"DEBUG> Starting to fold"};
        let (instr_number, fold_axis, fold_coord) = self.instructions.pop().unwrap();
        let mut folded_points: Vec<(usize, usize)> = Vec::new();
        for (px, py) in &self.points {
            match fold_axis.as_str() {
                "x" => {
                    if *px < fold_coord {
                        folded_points.push((*px, *py));
                    } else if *px > fold_coord {
                        let new_point = (fold_coord - (*px - fold_coord), *py);
                        if !folded_points.contains(&new_point) {
                            folded_points.push(new_point);
                        }
                    }
                    self.width = fold_coord - 1;
                }
                "y" => {
                    if *py < fold_coord {
                        folded_points.push((*px, *py));
                    } else if *py > fold_coord {
                        let new_point = (*px, fold_coord - (*py - fold_coord));
                        if !folded_points.contains(&new_point) {
                            folded_points.push(new_point);
                        }
                    }
                    self.height = fold_coord - 1;
                }
                _ => unreachable! {"Unknown fold axis: {}", fold_axis},
            }
        }
        folded_points.sort_unstable();
        folded_points.dedup();
        self.points = folded_points;
        println! {"DEBUG> Finished fold state:"};
        // self.draw();
    }

    pub fn execute_all_folds(&mut self) {
        while !self.instructions.is_empty() {
            self.fold();
        }
    }
}

impl fmt::Display for Origami {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "Origami with {} points and {} instructions (w: {}, h: {})\n\tPoints: {:?}\n\tInstr: {:?}",
        self.points.len(), self.instructions.len(), self.width, self.height, self.points, self.instructions}
    }
}

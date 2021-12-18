use std::fmt;


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct HeightMap {
    heightmap: Vec<usize>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn new(heightmap: Vec<usize>, width: usize, height: usize) -> HeightMap {
        HeightMap { 
            heightmap: heightmap,
            width: width,
            height: height,
        }
    }

    pub fn parse_input(input: String) -> HeightMap {
        println! {">>> Starting to parse input <<<"}

        let mut heightmap: Vec<usize> = Vec::new();
        let lines = input.lines();
        let height = lines.clone().count();
        let mut width = 0;
        for line in lines {
            if width == 0 {
                width = line.len();
            }
            heightmap.append(&mut line.chars().map(|h| h.to_digit(10).unwrap().try_into().unwrap()).collect());
        }

        HeightMap::new(heightmap, width, height)
    }

    fn get_point_at(&self, x: usize, y: usize) -> usize {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.heightmap[y * self.width + x]
        } else {
            panic!{"Cannot access point at x: {} y: {} (w: {}, h: {})", x, y, self.width, self.height}
        }
    }

    fn get_adjacent_coordinates(&self, x: usize, y: usize) -> Vec<(usize, usize, usize)> {
        let mut adjacent_coordinates = Vec::new();
        if x > 0 {
            // Check left
            adjacent_coordinates.push((x - 1, y, self.get_point_at(x - 1, y)));
        }
        if x < self.width - 1 {
            // Check right
            adjacent_coordinates.push((x + 1, y, self.get_point_at(x + 1, y)));
        }
        if y > 0 {
            // Check above
            adjacent_coordinates.push((x, y - 1, self.get_point_at(x, y - 1)));
        }
        if y < self.height - 1 {
            // Check below
            adjacent_coordinates.push((x, y + 1, self.get_point_at(x, y + 1)));
        }
        adjacent_coordinates
    }

    pub fn low_points(&self) -> Vec<usize> {
        let mut low_points: Vec<usize> = Vec::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let current = self.heightmap[y * self.width + x];
                let adjacent_points = self.get_adjacent_coordinates(x, y);
                let adjacent_lower_points: Vec<_> = adjacent_points
                    .iter()
                    .filter(|(_, _, p)| *p <= current)
                    .collect();

                if adjacent_lower_points.len() == 0 {
                    // println!{"DEBUG> Found low point at x:{} y:{} with value {}", x, y, current};
                    low_points.push(current);
                }
            }
        }
        low_points
    }

    pub fn risk_of_height(height: usize) -> usize {
        1 + height
    }

    pub fn total_risk(&self) -> usize {
        self.low_points().iter()
            .map(|&p| HeightMap::risk_of_height(p))
            .fold(0, |acc, r| acc + r)
    }

    pub fn basins(&self) -> Vec<usize> {
        let mut basins: Vec<(usize, usize)> = Vec::new();

        // Iterate all points and cluster them in basins
        for y in 0..self.height {
            for x in 0..self.width {
                let adjacent_points = vec![
                    0,
                ];
            }
        }

        // println!{"DEBUG> All points: {:?}", all_points.collect::<Vec<(usize, usize)>>()};
        vec![]
    }
}

impl fmt::Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "Heightmap with {} measurements (w: {}, h: {}):\n", self.heightmap.len(), self.width, self.height};
        for row in self.heightmap.chunks(self.width) {
            for &cell in row {
                if cell == 9 {
                    write!{f, "#"};
                } else {
                    write!{f, "{}", cell};
                }
            }
            write!{f, "\n"};
        }
        write!{f, "=======================\n"}
    }
}

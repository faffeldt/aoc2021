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

    pub fn low_points(&self) -> Vec<usize> {
        let mut low_points: Vec<usize> = Vec::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let current = self.heightmap[y * self.width + x];
                let mut is_low_point = true;

                if x > 0 {
                    // Check left
                    is_low_point &= current < self.heightmap[y * self.width + (x - 1)]
                }
                
                if x < self.width - 1 {
                    // Check right
                    is_low_point &= current < self.heightmap[y * self.width + (x + 1)]
                }

                if y > 0 {
                    // Check above
                    is_low_point &= current < self.heightmap[(y - 1) * self.width + x]
                }

                if y < self.height - 1 {
                    // Check below
                    is_low_point &= current < self.heightmap[(y + 1) * self.width + x]
                }
                
                if is_low_point {
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
}

impl fmt::Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "Heightmap with {} measurements (w: {}, h: {}):\n", self.heightmap.len(), self.width, self.height};
        for row in self.heightmap.chunks(self.width) {
            write!{f, "{:?}\n", row};
        }
        write!{f, "=======================\n"}
    }
}

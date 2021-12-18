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

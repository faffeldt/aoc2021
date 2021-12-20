use std::fmt;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct OctopusField {
    octopuses: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    flashes: usize,
    step: usize,
}

impl OctopusField {
    fn new(octopuses: Vec<Vec<usize>>, height: usize, width: usize) -> OctopusField {
        OctopusField {
            octopuses: octopuses,
            height: height,
            width: width,
            flashes: 0,
            step: 0,
        }
    }

    pub fn parse_input(input: String) -> OctopusField {
        println! {">>> Starting to parse input <<<"}

        let octopuses = input
            .lines()
            .map(|l| {
                l.chars()
                    // .inspect(|o| println!{"DEBUG> parsing: {}", o})
                    .map(|o| o.to_digit(10).unwrap().try_into().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        let height = octopuses.len();
        let width = octopuses[0].len();
        OctopusField::new(octopuses, height, width)
    }

    pub fn get_octopuses(&self) -> Vec<Vec<usize>> {
        self.octopuses.clone()
    }

    pub fn get_flashes(&self) -> usize {
        self.flashes
    }

    pub fn get_step(&self) -> usize {
        self.step
    }

    fn all_flashed(&self) -> bool {
        let mut all_flashed = true;
        for y in 0..self.height {
            for x in 0..self.width {
                all_flashed &= self.octopuses[y][x] == 0;
                if !all_flashed {
                    break;
                }
            }
            if !all_flashed {
                break;
            }
        }
        all_flashed
    }

    pub fn step(&mut self) {
        self.step += 1;

        // First: Increase energy level of every octopus
        let mut coords_to_flash: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                self.octopuses[y][x] += 1;
                if self.octopuses[y][x] > 9 {
                    coords_to_flash.push((x, y));
                }
            }
        }

        // Second: Propagate flashes to all neighbours (incl. diagonally)
        //         one octopus can be activated more than once per step
        // Position modifier (dx, dy)
        let neighbour_modifier = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        while coords_to_flash.len() > 0 {
            // println!{"DEBUG> Coords to flash: {:?}", coords_to_flash};
            let mut new_coords_to_flash: Vec<(usize, usize)> = Vec::new();

            for (x, y) in coords_to_flash {
                // Check if neighbour of flashed coord flash as well
                for (dx, dy) in &neighbour_modifier {
                    let (nx, ny) = ((x as isize + *dx) as usize, (y as isize + *dy) as usize);
                    // Check that neighbour has not flashed already
                    if nx < self.width && ny < self.height && self.octopuses[ny][nx] < 10 {
                        self.octopuses[ny][nx] += 1;
                        // If neighbour just flashed, then add to coords to flash in next step
                        if self.octopuses[ny][nx] > 9 {
                            new_coords_to_flash.push((nx, ny));
                        }
                    }
                }
            }
            coords_to_flash = new_coords_to_flash;
        }

        // Third: Reset flashed octopuses and count flashes
        for y in 0..self.height {
            for x in 0..self.width {
                if self.octopuses[y][x] > 9 {
                    self.flashes += 1;
                    self.octopuses[y][x] = 0;
                }
            }
        }
    }

    pub fn step_x_times(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn step_until_all_flash(&mut self) {
        while !self.all_flashed() {
            self.step();
        }
    }
}

impl fmt::Display for OctopusField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "Octopusfield (w: {}, h: {}, step: {}, flashes: {}):\n", self.width, self.height, self.step, self.flashes};
        for row in &self.octopuses {
            for octopus in row {
                if *octopus == 0 {
                    write! {f, "*"};
                } else if *octopus > 9 {
                    write! {f, "#"};
                } else {
                    write! {f, "{}", octopus};
                }
            }
            write! {f, "\n"};
        }
        write! {f, ""}
    }
}

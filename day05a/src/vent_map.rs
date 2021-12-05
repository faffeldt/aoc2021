use std::fmt;
use std::fs;

pub struct VentMap {
    lines: Vec<Line>,
    width: usize,
    height: usize,
}

impl VentMap {
    fn new(lines: Vec<Line>, width: usize, height: usize) -> VentMap {
        VentMap {
            lines: lines,
            width: width,
            height: height,
        }
    }

    pub fn from_file(filename: &str) -> VentMap {
        let newline = "\r\n";
        let coord_sep = ",";
        let point_sep = " -> ";

        println! {">>> Starting to parse file {} <<<", filename}
        let raw_data = fs::read_to_string(filename).unwrap();
        let raw_lines: Vec<&str> = raw_data.split(newline).collect();
        let mut lines = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for raw_line in raw_lines {
            let raw_points: Vec<&str> = raw_line
                .split(point_sep)
                .collect();
            let mut coords = Vec::new();
            for raw_coord in raw_points {
                let raw_coords: Vec<&str> = raw_coord
                    .split(coord_sep)
                    .collect();
                let point = Point::new(raw_coords[0].parse::<usize>().unwrap(), raw_coords[1].parse::<usize>().unwrap());
                coords.push(point);
                if point.x > max_x {
                    max_x = point.x;
                }
                if point.y > max_y {
                    max_y = point.y;
                }
            }
            lines.push(Line::new(coords[0], coords[1]));
        }

        VentMap::new(lines, max_x, max_y)
    }

    pub fn number_of_overlapping_lines(&self) -> usize {
        let mut num_overlapping_lines = 0;
        for row in self.create_usage_grid() {
            for cell in row {
                if cell > 1 {
                    num_overlapping_lines += 1;
                }
            }
        }
        num_overlapping_lines
    }

    fn create_usage_grid(&self) -> Vec<Vec<usize>> {
        let mut grid: Vec<Vec<usize>> = vec!{vec!{0;self.width+1};self.height+1};
        for line in &self.lines {
            let points_on_line = line.get_points_on_line();
            for point in points_on_line {
                grid[point.y][point.x] += 1;
            }
        }
        grid
    }

    pub fn draw(&self) {
        println!{"> Full grid:\n"};
        for row in self.create_usage_grid() {
            for cell in row {
                print!{"{}", cell};
            }
            println!{""};
        }
        
    }
}

impl fmt::Display for VentMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!{f, "> VentMap:"};
        for line in &self.lines {
            writeln!{f, "{}", line};
        }
        write!{f, ""}
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line {
            start: start,
            end: end,
        }
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn get_points_on_line(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        if self.is_straight() {
            let (dx, dy) = self.start.abs_diff(self.end);
            let x_mod = dx.signum();
            let y_mod = dy.signum();
            for step in 0..std::cmp::max(dx.abs(), dy.abs())+1 {
                let new_x = self.start.x as isize + (step * x_mod);
                let new_y = self.start.y as isize + (step * y_mod);
                // println!{"DEBUG> Point on line: start={} end={} step={}:\n\tdx={}, x_mod={}, start_x={}, dy={}, y_mod={}, start_y={}", self.start, self.end, step, dx, x_mod, self.start.x as isize, dy, y_mod, self.start.y as isize}
                points.push(Point::new(new_x as usize, new_y as usize));
            }
            // println!{"DEBUG> Points on line: start={} end={}:\n\t{:?}", self.start, self.end, points}

        } else {
            // Not implemented yet
        }
        points
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "{} -> {}", self.start, self.end}
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {x: x, y: y}
    }

    fn abs_diff(&self, other: Point) -> (isize, isize) {
        (
            other.x as isize - self.x as isize,
            other.y as isize - self.y as isize 
        )
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "[{},{}]", self.x, self.y}
    }
}

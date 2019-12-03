use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::io;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vec2 (isize, isize);

impl Vec2 {
    fn distance(&self) -> isize {
        self.0.abs() + self.1.abs()
    }
}

impl Ord for Vec2 {
    fn cmp(&self, other: &Vec2) -> Ordering {
        other.distance().cmp(&self.distance())
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Vec2) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Crossing (Vec2, usize);

impl Ord for Crossing {
    fn cmp(&self, other: &Crossing) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Crossing {
    fn partial_cmp(&self, other: &Crossing) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct Grid {
    cells: HashMap<Vec2, HashMap<u8, usize>>,
    intersections: BinaryHeap<Vec2>,
    crossings: BinaryHeap<Crossing>,
    paths: u8
}

impl Grid {
    fn new() -> Self {
        Default::default()
    }
    
    fn closest_crossing(&self) -> Option<Vec2> {
        self.intersections.peek()
            .map(|c| *c)
    }

    fn shortest_crossing(&self) -> Option<Crossing> {
        self.crossings.peek()
            .map(|c| *c)
    }

    fn add_path<P: AsRef<[Step]>>(&mut self, path: P) {
        let mut cursor = Vec2(0, 0);
        let mut steps = 0;

        for step in path.as_ref() {
            for i in 0..step.distance() {
                match step {
                    Step::Up(_) => cursor.1 += 1,
                    Step::Right(_) => cursor.0 += 1,
                    Step::Down(_) => cursor.1 -= 1,
                    Step::Left(_) => cursor.0 -= 1
                }

                steps += 1;

                if cursor == Vec2(0, 0) { continue; }
                    
                self.set(cursor, self.paths, steps);
            }
        }

        self.paths += 1;
    }

    fn set(&mut self, point: Vec2, path_id: u8, steps: usize) {
        if !self.cells.contains_key(&point) {
            self.cells.insert(point, Default::default());
        }

        let cell = self.cells.get_mut(&point).unwrap();
        
        if cell.len() - (if cell.contains_key(&path_id) { 1 } else { 0 }) > 0 {
            self.intersections.push(point);

            let other_steps = cell.get(&(path_id - 1)).unwrap();
            let combined_steps = other_steps + steps;
            self.crossings.push(Crossing(point, combined_steps));
        }

        cell.insert(path_id, steps);
    }
}

///////////////////////////////////////////////////////////////////////////////

enum Step {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize)
}

impl Step {
    fn distance(&self) -> usize {
        *(match self {
            Step::Up(d) => d,
            Step::Right(d) => d,
            Step::Down(d) => d,
            Step::Left(d) => d
        })
    }
}

///////////////////////////////////////////////////////////////////////////////

fn parse(mut line: String) -> Vec<Step> {
    let mut steps = Vec::with_capacity(100);
    let mut value = String::new();

    line.push(',');

    for c in line.chars() {
        match c {
            ',' => {
                let int = value[1..].trim().parse().unwrap();
                let step = match value.get(0..1).unwrap() {
                    "U" => Step::Up,
                    "R" => Step::Right,
                    "D" => Step::Down,
                    "L" => Step::Left,
                    _ => panic!("unknown step type")
                }(int);

                value.clear();
                steps.push(step);
            },

            _ => value.push(c)
        }
    }

    steps
}

///////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut line1 = String::with_capacity(512);
    let mut line2 = String::with_capacity(512);

    io::stdin().read_line(&mut line1).unwrap();
    io::stdin().read_line(&mut line2).unwrap();
    
    let mut grid = Grid::new();
    grid.add_path(parse(line1));
    grid.add_path(parse(line2));

    println!(
        "\nclosest crossing distance: {:?}",
        grid.closest_crossing().unwrap().distance()
    );
    println!(
        "\n best crossing steps: {:?}",
        grid.shortest_crossing().unwrap().1
    );
}

#[cfg(test)]
mod tests_day3 {
    use super::*;

    #[test]
    fn test0() {
        let mut grid = Grid::new();
        grid.add_path(
            parse("R8,U5,L5,D3".to_string())
        );
        grid.add_path(
            parse("U7,R6,D4,L4".to_string())
        );
        assert_eq!(grid.closest_crossing().unwrap().distance(), 6);
    }

    #[test]
    fn test1() {
        let mut grid = Grid::new();
        grid.add_path(
            parse("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string())
        );
        grid.add_path(
            parse("U62,R66,U55,R34,D71,R55,D58,R83".to_string())
        );
        assert_eq!(grid.closest_crossing().unwrap().distance(), 159);
    }

    #[test]
    fn test2() {
        let mut grid = Grid::new();
        grid.add_path(
            parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string())
        );
        grid.add_path(
            parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string())
        );
        assert_eq!(grid.closest_crossing().unwrap().distance(), 135);
    }

    #[test]
    fn test3() {
        let mut grid = Grid::new();
        grid.add_path(
            parse("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string())
        );
        grid.add_path(
            parse("U62,R66,U55,R34,D71,R55,D58,R83".to_string())
        );
        assert_eq!(grid.shortest_crossing().unwrap().1, 610);
    }

    #[test]
    fn test4() {
        let mut grid = Grid::new();
        grid.add_path(
            parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string())
        );
        grid.add_path(
            parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string())
        );
        assert_eq!(grid.shortest_crossing().unwrap().1, 410);
    }
}

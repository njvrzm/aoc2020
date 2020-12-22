pub mod day_seventeen {
    use std::collections::{HashMap, HashSet};
    use std::str::FromStr;
    use std::string::ParseError;

    type Point = (i32, i32, i32, i32);

    #[derive(Debug)]
    struct World {
        alive: HashSet<Point>,
        neighbors: HashMap<Point, i32>,
    }
    impl FromStr for World {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let alive = HashSet::new();
            let neighbors : HashMap<Point, i32> = HashMap::new();
            let mut world = World{alive, neighbors};
            for (i, line) in s.lines().enumerate() {
                for (j, char) in line.chars().enumerate() {
                    if char == '#' {
                        world.on(&(i as i32, j as i32, 0, 0));
                    }
                }
            }
            Ok(world)
        }
    }
    impl World {
        fn on(&mut self, p: &Point) {
            self.alive.insert(p.clone());
            for point in neighbors(p) {
                *self.neighbors.entry(point).or_insert(0) += 1
            }
        }
        fn off(&mut self, p: &Point) {
            self.alive.remove(p);
            for point in neighbors(p) {
                *self.neighbors.get_mut(&point).expect("Missing count for off operation") -= 1
            }
        }
        fn step(&mut self) {
            let mut on = Vec::new();
            let mut off = Vec::new();
            for (place, count) in self.neighbors.iter() {
                match (self.alive.contains(place), count) {
                    (true, &i) if i<2 || i>3 => off.push(place.clone()),
                    (false, &i) if i == 3 => on.push(place.clone()),
                    _ => ()
                }
            }
            on.iter().for_each(|p| self.on(p));
            off.iter().for_each(|p| self.off(p));
        }
    }
    fn neighbors((x, y, z, w): &Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                            points.push((x + dx, y + dy, z + dz, w + dw));
                        }
                    }
                }
            }
        }
        points
    }

    pub fn run_one(input: String) {
        let mut world = World::from_str(&input).unwrap();
        for _ in 0..6 {
            world.step();
        }
        println!("Total on: {}", world.alive.len());
    }
}
pub mod day_eleven {
    use std::str::FromStr;
    use itertools::{Itertools};
    use std::ops::{Index, IndexMut};
    use std::fmt;
    use std::string::ParseError;

    #[derive(Debug)]
    #[derive(Clone)]
    enum State {
        Empty,
        Occupied,
        Floor,
    }
    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                State::Empty => 'L',
                State::Occupied=> '#',
                State::Floor => '.',
            })
        }
    }
    struct Row {
        row: Vec<State>
    }
    struct Field {
        field: Vec<Row>
    }
    struct World {
        this: Field,
        that: Field,
        which: bool,
        width: usize,
        height: usize,
        farlook: bool,
    }

    impl FromStr for State {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "L" => State::Empty,
                "#" => State::Occupied,
                _ => State::Floor,
            })
        }
    }
    impl FromStr for Row {
        type Err = ParseError;
        fn from_str(line: &str) -> Result<Self, Self::Err> {
            Ok(Self{row: line.chars().into_iter().map(|c| State::from_str(&c.to_string()).unwrap()).collect_vec()})
        }
    }
    impl Index<usize> for Row {
        type Output = State;
        fn index(&self, index: usize) -> &Self::Output {
            &self.row[index]

        }
    }
    impl IndexMut<usize> for Row {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.row[index]
        }
    }
    impl fmt::Display for Row {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.row.iter().map(|st| format!("{}", st)).join(""))
        }
    }
    impl fmt::Display for Field {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.field.iter().map(|r| format!("{}", r)).join("\n"))
        }
    }
    impl Index<usize> for Field {
        type Output = Row;
        fn index(&self, index: usize) -> &Self::Output {
            &self.field[index]
        }
    }
    impl IndexMut<usize> for Field {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.field[index]
        }
    }

    impl FromStr for Field {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Field{field: s.lines().map(|l| Row::from_str(l).unwrap()).collect_vec()})
        }
    }
    impl Field {
        fn len(&self) -> usize {
            self.field.len()
        }
    }
    impl Row {
        fn len(&self) -> usize {
            self.row.len()
        }
    }
    impl FromStr for World {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let this = Field::from_str(s).unwrap();
            let that = Field::from_str(s).unwrap();
            let width = this.len();
            let height = this[0].len();
            let which = true;
            Ok(Self{this, that, which, width, height, farlook: false})
        }
    }
    impl Index<(usize,usize)> for World {
        type Output = State;

        fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
            match self.which {
                true => &self.this[x][y],
                false => &self.that[x][y],
            }
        }
    }
    impl IndexMut<(usize, usize)> for World  {
        fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
            match self.which {
                true => &mut self.that[x][y],
                false => &mut self.this[x][y],
            }
        }
    }
    impl World {
        fn neighbor_count(&self, x:usize, y:usize) -> u8 {
            match self.farlook {
                true => self.neighbor_count_far(x, y),
                false => self.neighbor_count_near(x, y),
            }
        }
        fn neighbor_count_near(&self, x:usize, y:usize) -> u8 {
            let mut count= 0;
            let ox = x as isize;
            let oy = y as isize;
            for dx in -1..=1 {
                let nx = ox + dx;
                if nx < 0 || nx as usize > self.width - 1 {
                    continue
                }
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue
                    }
                    let ny = oy + dy;
                    if ny < 0 || ny as usize > self.height - 1 {
                        continue
                    }
                    count += match self[(nx as usize, ny as usize)] {
                        State::Occupied => 1,
                        _ => 0,
                    }
                }
            }
            count
        }
        fn neighbor_count_far(&self, x:usize, y:usize) -> u8 {
            let mut count= 0;
            for dx in -1..=1 as isize {
                for dy in -1..=1 as isize {
                    if dx == 0 && dy == 0 {
                        continue
                    }
                    let mut ox = x as isize;
                    let mut oy = y as isize;
                    loop {
                        ox += dx;
                        oy += dy;
                        if ox < 0 || ox + 1 > self.width as isize|| oy < 0 || oy + 1 > self.height as isize{break}
                        match self[(ox as usize, oy as usize)] {
                            State::Occupied => {count += 1;break}
                            State::Empty => {break},
                            State::Floor => (),
                        }
                    }
                }
            }
            count
        }
        fn step(&mut self) -> bool {
            let mut changed = false;
            for i in 0..self.width {
                for j in 0..self.height {
                    match (self.farlook, &self[(i,j)], self.neighbor_count(i, j)) {
                        (_, State::Empty, 0) => {self[(i,j)] = State::Occupied; changed = true},
                        (_, State::Empty, _) => {self[(i,j)] = State::Empty},
                        (false, State::Occupied, c) if c >= 4 => {self[(i,j)] = State::Empty; changed = true},
                        (true, State::Occupied, c) if c >= 5 => {self[(i,j)] = State::Empty; changed = true},
                        (_, State::Occupied, _) => {self[(i,j)] = State::Occupied},
                        (_, State::Floor, _) => (),
                    };
                }
            }
            self.which = !self.which;
            changed
        }
        fn total_occupied(&self) -> u32 {
            let mut c = 0;
            for i in 0..self.width {
                for j in 0..self.height {
                    c += match self[(i, j)] {
                        State::Occupied => 1,
                        _ => 0,
                    }
                }
            }
            c
        }
        #[allow(dead_code)]
        fn show(&self) {
            for j in 0..self.height {
                let mut line: Vec<String> = Vec::with_capacity(self.width);
                for i in 0..self.width {
                    line.push(match self[(i,j)] {
                        State::Floor => String::from("."),
                        _ => self.neighbor_count(i, j).to_string(),
                    })
                }
                println!("{}", line.join(""))
            }
        }
    }

    pub fn run_one(input: String) {
        let mut world = World::from_str(&input).unwrap();
        while world.step() {
        }
        println!("Total occupied seats: {}", world.total_occupied())
        // println!("{}", match world.which {true => world.this, false => world.that});
    }
    pub fn run_two(input: String) {
        let mut world = World::from_str(&input).unwrap();
        world.farlook = true;
        while world.step() {
            println!("{}", match &world.which {true=>&world.that, false=>&world.this})
        }
        println!("Total occupied seats: {}", world.total_occupied())
        // println!("{}", match world.which {true => world.this, false => world.that});
    }
}
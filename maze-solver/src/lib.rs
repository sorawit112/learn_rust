use std::collections::{HashSet, VecDeque};
use std::fmt;

//  --> y
//  | ['S', '.', '#', '#', '#'], [0][*] [x][y]
//  x ['#', '.', '#', '.', '.'], [1][*]
//    ['#', '.', '.', '.', '#'], [2][*]
//    ['#', '#', '#', '.', '#'], [3][*]
//    ['#', '#', '#', 'E', '#']

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32
}

// Implement Display trait for Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn neighbor(&self) -> Vec<Point>{
        vec![
            Point{x: self.x,     y: self.y + 1}, // right
            Point{x: self.x + 1, y: self.y},     // down
            Point{x: self.x,     y: self.y - 1}, // left
            Point{x: self.x - 1, y: self.y}      // up
        ]
    }    
}

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    // Your code here
    if maze.is_empty() || maze[0].is_empty() {
        return vec![];
    }
    let rows = maze.len() as i32;
    let cols = maze[0].len() as i32;

    let start = Point{x: start.0 as i32, y: start.1 as i32};
    let end = Point{x: end.0 as i32, y: end.1 as i32};

    let mut visited:HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point,Vec<Point>)> = VecDeque::new();

    visited.insert(start);
    queue.push_back((start, vec![start]));

    while let Some((current, path)) = queue.pop_front() {
        // check goal reach condition
        if current == end {
            return path.into_iter()
                    .map(|p|(p.x as usize, p.y as usize))
                    .collect();
        }
        
        println!("current: {}, neighbor: {:?}",current, current.neighbor());

        for next in current.neighbor(){
            // skip out of scope 
            if next.x < 0 || next.x >= rows || next.y < 0 || next.y >= cols {
                continue;
            }

            // skip wall
            if maze[next.x as usize][next.y as usize] == '#' {
                continue;
            }

            // skip already visited
            if visited.contains(&next) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(next);

            visited.insert(next);
            queue.push_back((next, new_path));
        }
    }

    vec![]
}

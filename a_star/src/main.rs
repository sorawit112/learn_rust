use image::{RgbImage, Rgb};
use rand::Rng;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    position: (i32, i32),
    cost: i32,
    priority: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority).then_with(|| self.cost.cmp(&other.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: i32,
    height: i32,
    obstacles: HashSet<(i32, i32)>,
}

impl Grid {
    fn is_within_bounds(&self, position: (i32, i32)) -> bool {
        position.0 >= 0 && position.0 < self.width && position.1 >= 0 && position.1 < self.height
    }

    fn is_passable(&self, position: (i32, i32)) -> bool {
        !self.obstacles.contains(&position)
    }

    fn neighbors(&self, position: (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for delta in &deltas {
            let new_position = (position.0 + delta.0, position.1 + delta.1);
            if self.is_within_bounds(new_position) && self.is_passable(new_position) {
                neighbors.push(new_position);
            }
        }
        neighbors
    }
}

fn heuristic(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn a_star_search(grid: &Grid, start: (i32, i32), goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = std::collections::HashMap::new();
    let mut cost_so_far = std::collections::HashMap::new();

    open_set.push(Node { position: start, cost: 0, priority: heuristic(start, goal) });
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if current.position == goal {
            let mut path = Vec::new();
            let mut current_pos = current.position;
            while let Some(&Some(prev)) = came_from.get(&current_pos) {
                path.push(current_pos);
                current_pos = prev;
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for next in grid.neighbors(current.position) {
            let new_cost = cost_so_far[&current.position] + 1;
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(next, goal);
                open_set.push(Node { position: next, cost: new_cost, priority });
                came_from.insert(next, Some(current.position));
            }
        }
    }

    None
}

fn generate_random_obstacles(grid_size: i32, density: f32) -> HashSet<(i32, i32)> {
    let num_obstacles = (grid_size * grid_size) as f32 * density;
    let mut obstacles = HashSet::new();
    let mut rng = rand::thread_rng();

    while obstacles.len() < num_obstacles as usize {
        let x = rng.gen_range(0..grid_size);
        let y = rng.gen_range(0..grid_size);
        obstacles.insert((x, y));
    }

    obstacles
}

fn genenerate_random_vector(grid_size: i32) -> (i32,i32) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0..grid_size);
    let y = rng.gen_range(0..grid_size);

    (x,y)
}

fn visualize_grid(grid: &Grid, start: &(i32, i32), goal: &(i32, i32), path: &[(i32, i32)], n: i32) {
    let scale = 20;
    let width = grid.width * scale;
    let height = grid.height * scale;

    let mut img = RgbImage::new(width as u32, height as u32);

    // Draw free space (white) and obstacles (black)
    for y in 0..grid.height {
        for x in 0..grid.width {
            let color = if grid.obstacles.contains(&(x, y)) {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            };
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel((x * scale + dx) as u32, (y * scale + dy) as u32, color);
                }
            }
        }
    }

    // Draw robot path (blue)
    for &(x, y) in path.iter() {
        for dy in 0..scale {
            for dx in 0..scale {
                img.put_pixel((x * scale + dx) as u32, (y * scale + dy) as u32, Rgb([128, 128, 128]));
            }
        }
    }

    // Draw start position (green)
    for dy in 0..scale {
        for dx in 0..scale {
            img.put_pixel((start.1 * scale + dx) as u32, (start.1 * scale + dy) as u32, Rgb([0, 255, 0]));
        }
    }
    

    // Draw goal position (red)
    for dy in 0..scale {
        for dx in 0..scale {
            img.put_pixel((goal.0 * scale + dx) as u32, (goal.1 * scale + dy) as u32, Rgb([255, 0, 0]));
        }
    }
    
    img.save(format!("output_image/a*_result_{n}.png")).unwrap();

    
}


fn main() {

    let grid_size = 20;
    let density = 0.1;
    let obstacles = generate_random_obstacles(grid_size, density);

    let grid = Grid {
        width: grid_size,
        height: grid_size,
        obstacles,
    };

    // Create a directory to store frames
    fs::create_dir_all("output_image").unwrap();

    for i in 0..10 {
        let start = genenerate_random_vector(grid_size);
        let goal = genenerate_random_vector(grid_size);

        if let Some(path) = a_star_search(&grid, start, goal) {
            visualize_grid(&grid, &start, &goal, &path, i);
            println!("Path_{} found: {:?}", i, path);
        } else {
            println!("No path found from {:?}->{:?}", start, goal);
        }
    }

}
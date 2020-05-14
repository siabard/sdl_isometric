use std::collections::HashSet;

/// A* Node
#[derive(Eq, PartialEq, Debug)]
pub struct Node {
    pub f: u32,
    pub g: u32,
    pub h: u32,
    pub x: usize,
    pub y: usize,
    pub previous: Option<(usize, usize)>,
    pub blocked: bool,
}

/// A* Search Algorithm
pub struct AStar {
    cols: usize,
    rows: usize,
    grid: Vec<Node>,
}

impl AStar {
    pub fn new(cols: usize, rows: usize) -> AStar {
        let mut grid = vec![];
        for r in 0..rows {
            for c in 0..cols {
                grid.push(Node {
                    f: 0,
                    g: 0,
                    h: 0,
                    x: c,
                    y: r,
                    previous: None,
                    blocked: false,
                });
            }
        }
        AStar { cols, rows, grid }
    }

    pub fn run(&mut self, src: (usize, usize), tgt: (usize, usize)) -> Vec<(usize, usize)> {
        let mut openset: Vec<(usize, usize)> = vec![]; // Index만 가지고 있으면 됨
        let mut closedset: HashSet<(usize, usize)> = HashSet::new(); // index만 가지고 있으면 됨

        let c: u32 = 10; // 이동 코스트

        openset.push(src);

        let mut path: Vec<(usize, usize)> = vec![];
        let mut done: bool = false;
        while !done && !openset.is_empty() {
            // current is lowest F-score in openset
            let mut winner = 0;
            for i in 0..openset.len() {
                let i_f = self.get_node_f(openset[i]);
                let winner_f = self.get_node_f(openset[winner]);
                if i_f < winner_f {
                    winner = i;
                }
            }

            let current = openset[winner];

            // current == goal done!

            if current.0 == tgt.0 && current.1 == tgt.1 {
                // Done!
                let mut steps = current;
                path.push(current);
                while self.get_node_previous(steps).is_some() {
                    path.push(self.get_node_previous(steps).unwrap());
                    steps = self.get_node_previous(steps).unwrap();
                }

                path.reverse();
                println!("{:?}", path);
                done = true;
            } else {
                closedset.insert(current);
                openset.remove(winner);

                // calcualate neighbors
                let mut neighbors: Vec<(usize, usize)> = vec![];
                if current.0 > 0 && !self.get_node_blocked((current.0 - 1, current.1)) {
                    neighbors.push((current.0 - 1, current.1));
                }

                if current.0 < self.cols - 1 && !self.get_node_blocked((current.0 + 1, current.1)) {
                    neighbors.push((current.0 + 1, current.1));
                }

                if current.1 > 0 && !self.get_node_blocked((current.0, current.1 - 1)) {
                    neighbors.push((current.0, current.1 - 1));
                }

                if current.1 < self.rows - 1 && !self.get_node_blocked((current.0, current.1 + 1)) {
                    neighbors.push((current.0, current.1 + 1));
                }

                for neighbor in neighbors {
                    let cost = self.get_node_g(current) + c;
                    let neighbor_g = self.get_node_g(neighbor);
                    if openset.contains(&neighbor) && cost < neighbor_g {
                        // remove neighbor from open, because new path is better
                        openset.retain(|&x| x.0 == neighbor.0 && x.1 == neighbor.1);
                    }

                    if closedset.contains(&neighbor) && cost < neighbor_g {
                        // remove neighbor from closed
                        closedset.remove(&neighbor);
                    }

                    if !openset.contains(&neighbor) && !closedset.contains(&neighbor) {
                        // set g(neighbor) to cost

                        self.set_node_g(neighbor, cost);
                        let node_g = self.get_node_g(neighbor);
                        let node_h = heuristic(neighbor, tgt);

                        self.set_node_f(neighbor, node_g + node_h);

                        // add neighbor to OPEN
                        openset.push(neighbor);

                        // set priority queue rank to g(neighbor) + h(neighbor)
                        // 위에서 F값을 설정하였으므로 패스

                        // set neighbor's parent to current
                        self.set_node_previous(neighbor, current);
                    }
                }
            }
        }

        // Find the path
        println!("DONE!");
        path
    }

    fn get_node_blocked(&mut self, (x, y): (usize, usize)) -> bool {
        self.grid[y * self.cols + x].blocked
    }

    fn set_node_blocked(&mut self, (x, y): (usize, usize), blocked: bool) {
        self.grid[y * self.cols + x].blocked = blocked;
    }

    fn get_node_previous(&mut self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        self.grid[y * self.cols + x].previous
    }

    fn set_node_previous(&mut self, (x, y): (usize, usize), pos: (usize, usize)) {
        self.grid[y * self.cols + x].previous = Some(pos);
    }

    fn get_node_f(&mut self, (x, y): (usize, usize)) -> u32 {
        self.grid[y * self.cols + x].f
    }

    fn get_node_g(&mut self, (x, y): (usize, usize)) -> u32 {
        self.grid[y * self.cols + x].g
    }

    fn get_node_h(&mut self, (x, y): (usize, usize)) -> u32 {
        self.grid[y * self.cols + x].h
    }

    fn set_node_f(&mut self, (x, y): (usize, usize), f: u32) {
        self.grid[y * self.cols + x].f = f;
    }

    fn set_node_g(&mut self, (x, y): (usize, usize), g: u32) {
        self.grid[y * self.cols + x].g = g;
    }

    fn set_node_h(&mut self, (x, y): (usize, usize), h: u32) {
        self.grid[y * self.cols + x].h = h;
    }
}

fn heuristic((sx, sy): (usize, usize), (ex, ey): (usize, usize)) -> u32 {
    ((sx as i32 - ex as i32).abs() + (sy as i32 - ey as i32).abs()) as u32
}

#[cfg(test)]
mod tests {
    use super::AStar;

    #[test]
    fn a_star_works() {
        let mut astar = AStar::new(25, 25);

        let result = astar.run((0, 0), (24, 24));
        assert_ne!(result, vec![]);
    }
}

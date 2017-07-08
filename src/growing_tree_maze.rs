extern crate rand;
use rand::{thread_rng, Rng};

#[derive(PartialEq)]
pub enum CellType {
    Wall,
    Empty,
    ExposedUndertermined,
    UnexposedUndertermined,
}

pub struct GrowingTreeMaze {
    /// Maze grid width
    width: usize,

    /// Maze grid height
    height: usize,

    /// Maze grid
    grid: Vec<Vec<CellType>>,
}

impl GrowingTreeMaze {
    pub fn new(width: usize, height: usize) -> GrowingTreeMaze {
        let mut v: Vec<Vec<CellType>> = Vec::with_capacity(height);

        for _ in 0..height {
            let mut v2: Vec<CellType> = Vec::with_capacity(width);

            for _ in 0..width {
                v2.push(CellType::UnexposedUndertermined);
            }

            v.push(v2);
        }

        GrowingTreeMaze {
            width: width,
            height: height,
            grid: v,
        }
    }

    /// Make the cell at y,x a space.
    /// Update the frontier and field accordingly.
    /// Note: this does not remove the current cell from frontier, it only adds new cells.
    fn carve(&mut self, frontier: &mut Vec<(usize, usize)>, y: usize, x: usize) {
        let mut extra: Vec<(usize, usize)> = vec![];
        self.grid[y][x] = CellType::Empty;

        if x > 0 && self.grid[y][x - 1] == CellType::UnexposedUndertermined {
            self.grid[y][x - 1] = CellType::ExposedUndertermined;
            extra.push((y, x - 1));
        }

        if x < self.width - 1 && self.grid[y][x + 1] == CellType::UnexposedUndertermined {
            self.grid[y][x + 1] = CellType::ExposedUndertermined;
            extra.push((y, x + 1));
        }

        if y > 0 && self.grid[y - 1][x] == CellType::UnexposedUndertermined {
            self.grid[y - 1][x] = CellType::ExposedUndertermined;
            extra.push((y - 1, x));
        }

        if y < self.height - 1 && self.grid[y + 1][x] == CellType::UnexposedUndertermined {
            self.grid[y + 1][x] = CellType::ExposedUndertermined;
            extra.push((y + 1, x));
        }

        thread_rng().shuffle(&mut extra);
        frontier.append(&mut extra);
    }

    /// Make the cell at y,x a wall.
    fn harden(&mut self, y: usize, x: usize) {
        self.grid[y][x] = CellType::Wall;
    }

    /// Test the cell at y,x: can this cell become a space?
    /// True indicates it should become a space,
    /// False indicates it should become a wall.
    fn check(&self, y: usize, x: usize, nodiagonals: bool) -> bool {
        let mut edgestate = 0;

        if x > 0 && self.grid[y][x - 1] == CellType::Empty {
            edgestate += 1;
        }

        if x < self.width - 1 && self.grid[y][x + 1] == CellType::Empty {
            edgestate += 2;
        }

        if y > 0 && self.grid[y - 1][x] == CellType::Empty {
            edgestate += 4;
        }

        if y < self.height - 1 && self.grid[y + 1][x] == CellType::Empty {
            edgestate += 8;
        }

        if nodiagonals {
            // if this would make a diagonal connecition, forbid it
            // the following steps make the test a bit more complicated and are not necessary,
            // but without them the mazes don't look as good
            match edgestate {
                1 => {
                    if x < self.width - 1 {
                        if y > 0 && self.grid[y - 1][x + 1] == CellType::Empty {
                            return false;
                        }

                        if y < self.height - 1 && self.grid[y + 1][x + 1] == CellType::Empty {
                            return false;
                        }
                    }

                    true
                }
                2 => {
                    if x > 0 {
                        if y > 0 && self.grid[y - 1][x - 1] == CellType::Empty {
                            return false;
                        }

                        if y < self.height - 1 && self.grid[y + 1][x - 1] == CellType::Empty {
                            return false;
                        }
                    }

                    true
                }
                4 => {
                    if y < self.height - 1 {
                        if x > 0 && self.grid[y + 1][x - 1] == CellType::Empty {
                            return false;
                        }

                        if x < self.width - 1 && self.grid[y + 1][x + 1] == CellType::Empty {
                            return false;
                        }
                    }

                    true
                }
                8 => {
                    if y > 0 {
                        if x > 0 && self.grid[y - 1][x - 1] == CellType::Empty {
                            return false;
                        }

                        if x < self.width - 1 && self.grid[y - 1][x + 1] == CellType::Empty {
                            return false;
                        }
                    }

                    true
                }
                _ => false,
            }
        } else {
            return [1, 2, 4, 8].contains(&edgestate);
        }
    }

    /// parameter branchrate:
    /// zero is unbiased, positive will make branches more frequent, 
    /// negative will cause long passages
    /// this controls the position in the list chosen: 
    /// positive makes the start of the list more likely,
    /// negative makes the end of the list more likely
    /// large negative values make the original point obvious
    /// try values between -10, 10
    pub fn generate(&mut self, x_start: usize, y_start: usize, branchrate: f64) {
        // list of coordinates of exposed but undetermined cells.
        let mut frontier: Vec<(usize, usize)> = vec![];
        self.carve(&mut frontier, y_start, x_start);

        while frontier.len() > 0 {
            // select a random edge
            let mut pos = rand::random::<f64>();
            pos = pos.powf((-branchrate).exp());

            let idx = (pos * (frontier.len() as f64)) as usize;
            let (y, x) = frontier[idx];

            if self.check(y, x, true) {
                self.carve(&mut frontier, y, x);
            } else {
                self.harden(y, x);
            }

            frontier.remove(idx);
        }

        // set unexposed cells to be walls
        for i in 0..self.height {
            for j in 0..self.width {
                if self.grid[i][j] == CellType::UnexposedUndertermined {
                    self.grid[i][j] = CellType::Wall;
                }
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {

                match self.grid[i][j] {
                    CellType::Empty => print!(" "),
                    CellType::ExposedUndertermined => print!(","),
                    CellType::UnexposedUndertermined => print!("?"),
                    CellType::Wall => print!("#"),
                }
            }

            println!();
        }
    }

    /// returns (height, width)
    pub fn get_size(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    /// get cell type by position
    pub fn get_cell(&self, x: usize, y: usize) -> &CellType {
        &self.grid[y][x]
    }
}

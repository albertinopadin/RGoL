use crate::life_cell::LifeCell;
use rand::Rng;

pub struct Grid {
    x_cells: u32,
    y_cells: u32,
    generation: u64,
    // TODO: Figure out how to use an array here instead of a Vec:
    // cells: [LifeCell],
    cells: Vec<LifeCell>,
}

impl Grid {
    fn new(x_cells: u32, y_cells: u32) -> Self {
        Self {
            x_cells,
            y_cells,
            generation: 0,
            cells: Vec::with_capacity((x_cells * y_cells) as usize),
        }
    }

    fn set_neighbors(&mut self) {
        let xc = self.x_cells;
        let yc = self.y_cells;
        for x in 0..xc {
            for y in 0..yc {
                let neighbor_idxs = self.get_neighbor_indices_for_cell(x, y);
                self.cells[(x + y * self.x_cells) as usize].neighbor_indices = neighbor_idxs;
            }
        }
    }

    fn get_neighbor_indices_for_cell(&self, x: u32, y: u32) -> Vec<usize> {
        let mut neighbor_idxs: Vec<usize> = vec![];

        let left_x: i64 = (x - 1) as i64;
        let right_x: i64 = (x + 1) as i64;
        let top_y: i64 = (y - 1) as i64;
        let bottom_y: i64 = (y + 1) as i64;

        if left_x > -1 {
            neighbor_idxs.push((left_x + (y * self.x_cells) as i64) as usize);
        }

        if left_x > -1 && top_y > -1 {
            neighbor_idxs.push((left_x + top_y * (self.x_cells as i64)) as usize);
        }

        if top_y > -1 {
            neighbor_idxs.push((x as i64 + top_y * self.x_cells as i64) as usize);
        }

        if right_x < self.x_cells as i64 && top_y > -1 {
            neighbor_idxs.push((right_x + top_y * self.x_cells as i64) as usize);
        }

        if right_x < self.x_cells as i64 {
            neighbor_idxs.push((right_x + (y * self.x_cells) as i64) as usize);
        }

        if right_x < self.x_cells as i64 && bottom_y < self.y_cells as i64 {
            neighbor_idxs.push((right_x + bottom_y * self.x_cells as i64) as usize);
        }

        if bottom_y < self.y_cells as i64 {
            neighbor_idxs.push((x as i64 + bottom_y * self.x_cells as i64) as usize);
        }

        if left_x > -1 && bottom_y < self.y_cells as i64 {
            neighbor_idxs.push((left_x + bottom_y * self.x_cells as i64) as usize);
        }

        return neighbor_idxs;
    }

    fn update(&mut self) -> u64 {
        for x in 0..self.x_cells {
            for y in 0..self.y_cells {
                let neighbor_idxs =
                    self.cells[(x + y * self.x_cells) as usize].get_neighbor_indices();
                let live_neighbors: u8 = neighbor_idxs
                    .iter()
                    .filter(|nidx| self.cells[**nidx].alive)
                    .collect::<Vec<&usize>>()
                    .len() as u8;
                self.cells[(x + y * self.x_cells) as usize].prepare_update(live_neighbors);
            }
        }

        for x in 0..self.x_cells {
            for y in 0..self.y_cells {
                self.cells[(x + y * self.x_cells) as usize].update();
            }
        }

        self.generation += 1;
        self.generation
    }

    fn reset(&mut self) {
        for x in 0..self.x_cells {
            for y in 0..self.y_cells {
                self.cells[(x + y * self.x_cells) as usize].make_dead();
            }
        }

        self.generation = 0;
    }

    fn randomize(&mut self, live_probability: f64) {
        self.reset();

        let mut rng = rand::thread_rng();
        for x in 0..self.x_cells {
            for y in 0..self.y_cells {
                if rng.gen::<f64>() <= live_probability {
                    self.cells[(x + y * self.x_cells) as usize].make_live();
                }
            }
        }
    }
}

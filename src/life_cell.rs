
pub struct Point2D {
    x: f64,
    y: f64,
}

pub struct LifeCell {
    alive: bool,
    position: Point2D,
    // pub neighbors: Vec<&'a LifeCell<'a>>,
    pub neighbor_indices: Vec<usize>,
    current_state: bool,
    next_state: bool,
}

impl LifeCell {
    fn new(alive: bool, position: Point2D) -> Self {
        Self {
            alive,
            position,
            neighbor_indices: vec![],
            current_state: alive,
            next_state: alive,
        }
    }

    fn set_state(&mut self, live: bool) {
        self.alive = live;
        self.current_state = live;
        self.next_state = live;
    }

    pub fn make_live(&mut self) {
        self.set_state(true);
        // TODO: Change appearance
    }

    pub fn make_dead(&mut self) {
        self.set_state(false);
        // TODO: Change appearance
    }

    fn needs_update(&self) -> bool {
        self.next_state != self.current_state
    }

    pub fn get_neighbor_indices(&self) -> &Vec<usize> {
        &self.neighbor_indices
    }

    pub fn prepare_update(&mut self, live_neighbors: u8) {
        // This kinda smells (collecting into a Vec of references)...
        // let live_neighbors: u8 = self.neighbors.iter().filter(|n| n.alive).collect::<Vec<&&LifeCell>>().len() as u8;
        if !(!self.current_state && live_neighbors < 3) {
            self.next_state = (self.current_state && live_neighbors == 2) || (live_neighbors == 3);
        }
    }

    pub fn update(&mut self) {
        // TODO: Perhaps we can use match here?
        if self.needs_update() {
            if self.next_state {
                self.make_live();
            } else {
                self.make_dead();
            }
        }
    }
}
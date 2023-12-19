
struct Point2D {
    x: f64,
    y: f64,
}

struct Cell {
    alive: bool,
    position: Point2D,
    neighbors: Vec<Cell>,
    current_state: bool,
    next_state: bool,
}

impl Cell {
    fn new(alive: bool, position: Point2D) -> Self {
        Self {
            alive,
            position,
            neighbors: Vec::new(),
            current_state: alive,
            next_state: alive,
        }
    }

    fn set_state(&mut self, live: bool) {
        self.alive = live;
        self.current_state = live;
        self.next_state = live;
    }

    fn make_live(&mut self) {
        self.set_state(true);
        // TODO: Change appearance
    }

    fn make_dead(&mut self) {
        self.set_state(false);
        // TODO: Change appearance
    }

    fn needs_update(&self) -> bool {
        self.next_state != self.current_state
    }

    fn prepare_update(&mut self) {
        let live_neighbors: u8 = self.neighbors.iter().filter(|n| n.alive).collect().len();
        if !(!self.current_state && live_neighbors < 3) {
            self.next_state = (self.current_state && live_neighbors == 2) || (live_neighbors == 3);
        }
    }

    fn update(&mut self) {
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
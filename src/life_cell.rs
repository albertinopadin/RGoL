use graphics::color::{BLACK, GREEN};
use graphics::types::Color;
use graphics::Rectangle;

pub struct LifeCell {
    pub(crate) alive: bool,
    pub(crate) rect: Rectangle,
    pub(crate) corners: [f64; 4],
    // pub neighbors: Vec<&'a LifeCell<'a>>,
    pub(crate) neighbor_indices: Vec<usize>,
    current_state: bool,
    next_state: bool,
    alive_color: [f32; 4],
    dead_color: [f32; 4],
}

impl LifeCell {
    pub(crate) fn new(alive: bool, corners: [f64; 4]) -> Self {
        Self {
            alive,
            rect: Rectangle::new(if alive { GREEN } else { BLACK }),
            corners,
            neighbor_indices: vec![],
            current_state: alive,
            next_state: alive,
            alive_color: GREEN,
            dead_color: BLACK,
        }
    }

    fn set_state(&mut self, live: bool) {
        self.alive = live;
        self.current_state = live;
        self.next_state = live;
    }

    pub fn make_live(&mut self) {
        self.set_state(true);
        self.rect.color = self.alive_color;
    }

    pub fn make_dead(&mut self) {
        self.set_state(false);
        self.rect.color = self.dead_color;
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

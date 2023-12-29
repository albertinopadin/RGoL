use crate::app::{run_loop, App};
use crate::life_cell::LifeCell;
use opengl_graphics::GlGraphics;
use piston_window::*;

mod app;
mod grid;
mod life_cell;

fn get_num_alive(cells: &Vec<LifeCell>) -> usize {
    cells
        .iter()
        .filter(|c| c.alive)
        .collect::<Vec<&LifeCell>>()
        .len()
}

fn main() {
    let opengl = OpenGL::V4_1;

    let mut window: PistonWindow = WindowSettings::new("RGoL", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let gl = GlGraphics::new(opengl);

    let x_cells = 100;
    let y_cells = 100;
    let mut app = App::new(gl, x_cells, y_cells);
    let live_prob: f64 = 0.50;
    println!(
        "Num alive before randomize: {}",
        get_num_alive(&app.grid.cells)
    );
    app.grid.randomize(live_prob);
    println!(
        "Num alive after randomize: {}",
        get_num_alive(&app.grid.cells)
    );
    run_loop(&mut app, &mut window);
}

use opengl_graphics::*;
use piston_window::*;
mod grid;
mod life_cell;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct App {
    pub(crate) gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, g| {
            g.clear_color(BLACK);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}
}

fn run_loop(app: &mut App, w: &mut PistonWindow) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(w) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

fn main() {
    let opengl = OpenGL::V4_1;

    let mut window: PistonWindow = WindowSettings::new("RGoL", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let gl = GlGraphics::new(opengl);

    let mut app = App { gl };
    run_loop(&mut app, &mut window);
}

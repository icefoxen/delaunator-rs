extern crate delaunator;
extern crate ggez;
extern crate rand;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

struct State {
    mesh: graphics::Mesh,
}

impl State {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        const NUM_POINTS: usize = 100;
        const BOUNDS: f64 = 500.0;

        let points: Vec<_> = (0..NUM_POINTS)
            .map(|_| {
                let x = rand::random::<f64>() * BOUNDS;
                let y = rand::random::<f64>() * BOUNDS;
                delaunator::Point { x, y }
            }).collect();
        let triangulation = delaunator::triangulate(&points)
            .expect("Could not find a triangulation?  Maybe try again.");
        let triangle_points: Vec<_> = triangulation
            .triangles
            .iter()
            .map(|i| na::Point2::new(points[*i].x as f32, points[*i].y as f32))
            .collect();
        let triangles = triangle_points.chunks(3);
        let triangles = triangle_points.chunks(3);
        let mut mb = graphics::MeshBuilder::new();
        for triangle in triangles {
            let r = rand::random::<f32>();
            let g = rand::random::<f32>();
            let b = rand::random::<f32>();
            let color = graphics::Color::new(r, g, b, 1.0);
            mb.polygon(graphics::DrawMode::Fill, triangle, color);
        }
        let mesh = mb.build(ctx)?;
        Ok(Self { mesh })
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::draw(
            ctx,
            &self.mesh,
            (na::Point2::new(50.0, 50.0), graphics::WHITE),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let cb = ggez::ContextBuilder::new("draw_triangles", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Delaunator!"));
    let (mut ctx, mut ev) = cb.build().unwrap();
    let state = &mut State::new(&mut ctx).unwrap();
    event::run(&mut ctx, &mut ev, state).unwrap();
}

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
        const NUM_POINTS: usize = 20;
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
        // let mesh = ggez::graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::Fill,
        //     na::Point2::new(0.0, 0.0),
        //     100.0,
        //     10.0,
        // )?;
        // let mesh = graphics::Mesh::from_triangles(ctx, &triangle_points)?;
        let mesh =
            graphics::Mesh::new_polyline(ctx, graphics::DrawMode::Line(2.0), &triangle_points)?;
        Ok(Self { mesh })
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::draw(ctx, &self.mesh, na::Point2::new(50.0, 50.0), 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let cb = ggez::ContextBuilder::new("draw_triangles", "ggez");
    let ctx = &mut cb.build().unwrap();
    let state = &mut State::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

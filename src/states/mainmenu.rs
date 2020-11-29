use ggez::{event::EventHandler, graphics, Context};
use graphics::DrawParam;

use super::game::Resources;

pub struct MainMenuState {
    pub resources: &'static Resources,
}

impl MainMenuState {
    pub fn new(ctx: &mut Context, resources: &'static mut Resources) -> Self {
        Self { resources }
    }
}

impl EventHandler for MainMenuState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT),
            graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: 50.0,
                h: 50.0,
            },
            graphics::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        )?;

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        graphics::draw(ctx, &mesh, DrawParam::new())?;

        graphics::present(ctx)?;
        Ok(())
    }
}

use ggez::{Context, graphics::DrawParam};

pub mod background;
pub mod button;
pub mod character;
pub mod gamescreen;
pub mod mainmenuscreen;
pub mod stackcontainer;
pub mod textbox;
pub mod ui;
pub trait Draw {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> ggez::GameResult;
}

pub trait Update {
    fn update(&mut self, dt: f32);
}

use ggez::Context;

pub mod background;
pub mod button;
pub mod character;
pub mod screen;
pub mod stackcontainer;
pub mod textbox;
pub mod ui;
pub trait Draw {
    fn draw(&self, ctx: &mut Context) -> ggez::GameResult;
}

pub trait Update {
    fn update(&mut self, dt: f32);
}

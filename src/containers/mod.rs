use ggez::{graphics::DrawParam, Context};

pub mod background;
pub mod button;
pub mod character;
pub mod gamescreen;
pub mod mainmenuscreen;
pub mod stackcontainer;
pub mod textbox;
pub mod ui;

pub trait Update {
    fn update(&mut self, dt: f32);
}

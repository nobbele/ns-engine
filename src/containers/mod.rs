pub mod background;
pub mod button;
pub mod character;
pub mod config_window;
pub mod credits_window;
pub mod gamescreen;
pub mod mainmenuscreen;
pub mod rich_text;
pub mod slider;
pub mod sprite;
pub mod stackcontainer;
pub mod textbox;
pub mod ui;

pub trait Update {
    fn update(&mut self, dt: f32);
}

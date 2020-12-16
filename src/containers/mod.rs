pub mod background;
pub mod button;
pub mod character;
pub mod config_window;
pub mod gamescreen;
pub mod mainmenuscreen;
pub mod slider;
pub mod stackcontainer;
pub mod text_sprite;
pub mod textbox;
pub mod ui;

pub trait Update {
    fn update(&mut self, dt: f32);
}

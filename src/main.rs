use ggez::event;
use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics,
};
use states::{State, game::Resources, splash::SplashState};

mod containers;
mod draw;
mod helpers;
mod node;
mod states;
mod tween;

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("ns-engine", "nobbele")
        .window_setup(WindowSetup::default().title("NS Engine"))
        .window_mode(
            WindowMode::default()
                .dimensions(1280.0, 720.0)
                .resizable(true),
        )
        .add_zipfile_bytes(include_bytes!("../resources.zip").to_vec());
    let (mut ctx, event_loop) = cb.build()?;

    // This will live forever anyway
    let resources = Box::leak(Box::new(Resources {
        text_box: graphics::Image::new(&mut ctx, "/TextBox.png")?,
    }));

    let state = State::Splash(SplashState::new(&mut ctx, resources));
    event::run(ctx, event_loop, state)
}

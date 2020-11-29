use std::io::BufReader;
use ggez::event;
use ggez::filesystem;
use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics,
};
use states::game::{GameState, Resources};

mod containers;
mod draw;
mod helpers;
mod node;
mod tween;
mod states;

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

    let mut novel = novelscript::Novel::new();
    novel
        .add_scene(
            "start".into(),
            BufReader::new(filesystem::open(&mut ctx, "/test.ns").unwrap()),
        )
        .unwrap();

    // This will live forever anyway
    let resources = Box::leak(Box::new(Resources {
        text_box: graphics::Image::new(&mut ctx, "/TextBox.png")?,
    }));

    let state = GameState::new(&mut ctx, novel, resources);
    event::run(ctx, event_loop, state)
}

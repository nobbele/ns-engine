use std::io::Read;

use ggez::event;
use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics,
};
use states::{State, StateManager, game::{CharacterConfig, Config, Resources}, splash::SplashState};

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

    let mut config_file = ggez::filesystem::open(&mut ctx, "/characters.nsconf").unwrap();
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).unwrap();
    let config = nsconfig::parse(&config_content).unwrap();

    let config = Config {
        characters: config
            .into_iter()
            .map(|(name, m)| {
                (
                    name,
                    CharacterConfig {
                        color: match m.get("color").map(|s| s.as_str()).unwrap_or("white") {
                            "blue" => graphics::Color::from_rgb_u32(0x0000FFFF),
                            "white" => graphics::Color::from_rgb_u32(0xFFFFFFFF),
                            _ => panic!(),
                        },
                    },
                )
            })
            .collect(),
    };

    let resources = Box::leak(Box::new(Resources {
        text_box: graphics::Image::new(&mut ctx, "/TextBox.png")?,
        button: graphics::Image::new(&mut ctx, "/Button.png")?,
        config,
    }));

    let state = State::Splash(SplashState::new(&mut ctx, resources));
    let manager = StateManager::new(&mut ctx, state);
    event::run(ctx, event_loop, manager)
}

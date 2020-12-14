use std::io::Read;

use ggez::event;
use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics,
};
use states::{
    game::{CharacterConfig, Config, Resources, UIConfig},
    splash::SplashState,
    State, StateManager,
};

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
    let char_config = nsconfig::parse(&config_content).unwrap();

    let mut config_file = ggez::filesystem::open(&mut ctx, "/engine.nsconf").unwrap();
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).unwrap();
    let engine_config = nsconfig::parse(&config_content).unwrap();

    let ui_config = &engine_config["UI"];

    let config = Config {
        characters: char_config
            .into_iter()
            .map(|(name, m)| {
                (
                    name,
                    CharacterConfig {
                        color: graphics::Color::from_rgb_u32(
                            m.get("color")
                                .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                                .unwrap_or_default(),
                        ),
                    },
                )
            })
            .collect(),
        ui: UIConfig {
            button_color: graphics::Color::from_rgb_u32(
                ui_config
                    .get("button_color")
                    .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                    .unwrap_or_default(),
            ),
            button_pressed_color: graphics::Color::from_rgb_u32(
                ui_config
                    .get("button_pressed_color")
                    .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                    .unwrap_or_default(),
            ),
            button_highlight_color: graphics::Color::from_rgb_u32(
                ui_config
                    .get("button_highlight_color")
                    .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                    .unwrap_or_default(),
            ),
        },
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

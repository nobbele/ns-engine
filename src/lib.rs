use std::{cell::RefCell, rc::Rc};

use config::{CharacterConfig, Config, UIConfig, UserConfig};
use ggez::event;
use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics,
};
use log::error;
use resource_manager::ResourceManager;
use states::{splash::SplashState, State, StateManager};

mod config;
mod containers;
mod draw;
mod helpers;
mod node;
mod resource_manager;
mod states;
mod tween;

pub static mut CREDITS_TEXT: &str = "here are the credits";

pub fn run(resource_data: Option<Vec<u8>>) -> ggez::GameResult {
    simple_logging::log_to_file("run.log", log::LevelFilter::Info).unwrap();
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let loc = panic_info.location().unwrap();
        let loc = format!("{}:{}", loc.file(), loc.line());

        let msg = match panic_info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match panic_info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };
        error!("panic at {}: {}", loc, msg);
        default_hook(panic_info);
    }));

    let mut cb = ggez::ContextBuilder::new("ns-engine", "nobbele")
        .window_setup(WindowSetup::default().title("NS Engine"))
        .window_mode(
            WindowMode::default()
                .dimensions(1280.0, 720.0)
                .resizable(false),
        );
    if let Some(data) = resource_data {
        cb = cb.add_zipfile_bytes(data);
    }
    let (mut ctx, event_loop) = cb.build()?;

    let mut config_file = ggez::filesystem::open(&mut ctx, "/characters.ini").unwrap();
    let char_config = ini::Ini::read_from(&mut config_file).unwrap();

    let mut config_file = ggez::filesystem::open(&mut ctx, "/engine.ini").unwrap();
    let engine_config = ini::Ini::read_from(&mut config_file).unwrap();

    let ui_config = engine_config
        .section(Some("UI"))
        .expect("A UI Section must be declared");

    let user_config = if ggez::filesystem::exists(&ctx, "/config.json") {
        println!("Loading user config");
        let file = ggez::filesystem::open(&mut ctx, "/config.json").unwrap();
        serde_json::from_reader(file).unwrap()
    } else {
        let user_config = UserConfig::default();
        user_config.update_data(&mut ctx);
        user_config
    };

    let config = Config {
        characters: char_config
            .into_iter()
            .map(|(name, m)| {
                (
                    name.expect("No support for nameless characters").to_owned(),
                    CharacterConfig {
                        color: graphics::Color::from_rgb_u32(
                            m.get("color")
                                .map(|s| u32::from_str_radix(s, 16).unwrap())
                                .unwrap_or_default(),
                        ),
                    },
                )
            })
            .collect(),
        ui: UIConfig {
            title: ui_config
                .get("title")
                .map(|s| s.to_owned())
                .unwrap_or_else(|| "Untitled game".to_string()),
            button_color: graphics::Color::from_rgb_u32(
                ui_config
                    .get("button_color")
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .unwrap_or_default(),
            ),
            button_pressed_color: graphics::Color::from_rgb_u32(
                ui_config
                    .get("button_pressed_color")
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .unwrap_or_default(),
            ),
            button_highlight_color: graphics::Color::from_rgb_u32(
                ui_config
                    .get("button_highlight_color")
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .unwrap_or_default(),
            ),
        },
        user: Rc::new(RefCell::new(user_config)),
    };

    let resources = Box::leak(Box::new(ResourceManager::new(config)));

    let state = State::Splash(SplashState::new(&mut ctx, resources));
    let manager = StateManager::new(&mut ctx, state);
    event::run(ctx, event_loop, manager)
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ggez::{
    filesystem::OpenOptions,
    graphics::{self, Color},
    Context,
};

#[derive(Debug, Copy, Clone)]
pub struct CharacterConfig {
    pub color: Color,
}

impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            color: graphics::WHITE,
        }
    }
}

#[derive(Debug)]
pub struct UIConfig {
    pub title: String,
    pub button_color: Color,
    pub button_pressed_color: Color,
    pub button_highlight_color: Color,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Channels(pub HashMap<String, f32>);

impl Default for Channels {
    fn default() -> Self {
        Self(
            [("sfx".into(), 1.0), ("music".into(), 1.0)]
                .iter()
                .cloned()
                .collect(),
        )
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserConfig {
    #[serde(default)]
    pub master_volume: f32,
    #[serde(default)]
    pub channel_volumes: Channels,
}

impl UserConfig {
    pub fn update_data(&self, ctx: &mut Context) {
        if ggez::filesystem::exists(ctx, "/config.json") {
            println!("Updating user config");
            let file = ggez::filesystem::open_options(
                ctx,
                "/config.json",
                OpenOptions::new().write(true).truncate(true),
            )
            .unwrap();
            serde_json::to_writer(file, self).unwrap();
        } else {
            println!("Creating user config");
            let user_config = UserConfig::default();
            let file = ggez::filesystem::create(ctx, "/config.json").unwrap();
            serde_json::to_writer(file, &user_config).unwrap();
        };
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            master_volume: 0.5,
            channel_volumes: Channels::default(),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub characters: HashMap<String, CharacterConfig>,
    pub ui: UIConfig,
    pub user: Rc<RefCell<UserConfig>>,
}

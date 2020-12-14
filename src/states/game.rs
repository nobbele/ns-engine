use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::containers::{
    background::BackgroundContainer, button::Button, character::CharacterContainer,
    gamescreen::Action, gamescreen::GameScreen, stackcontainer::Direction,
    stackcontainer::StackContainer, ui::MenuButtonId, ui::UI, Update,
};
use crate::helpers::Position;
use crate::node::{load_background_tween, load_character_tween};
use ggez::graphics::{self, DrawParam};
use ggez::graphics::{Color, Drawable};
use ggez::{
    self,
    event::{KeyCode, KeyMods, MouseButton},
    Context,
};

use super::{State, StateEventHandler};

pub enum Placement {
    Left,
    Right,
}

pub struct Character {
    pub name: String,
    pub expression: String,
    pub image: graphics::Image,
    pub position: Option<Placement>,
    pub alpha: f32,
}

#[derive(Debug)]
pub struct Background {
    pub name: String,
    pub fade: f32,
    pub image: graphics::Image,
}

impl Background {
    pub fn new(image: graphics::Image, name: String) -> Self {
        Self {
            image,
            name,
            fade: 0.0,
        }
    }
}

#[derive(PartialEq)]
pub enum ContinueMethod {
    Auto(f32),
    Skip(f32),
    Normal,
}

pub struct GameState {
    pub novel: novelscript::Novel,
    pub state: novelscript::NovelState,
    pub resources: &'static Resources,
    pub continue_method: ContinueMethod,
    pub screen: GameScreen,
    pub sfx: Option<ggez::audio::Source>,
    pub music: Option<ggez::audio::Source>,
    pub ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
}

impl GameState {
    pub fn new(
        ctx: &mut Context,
        novel: novelscript::Novel,
        resources: &'static Resources,
    ) -> GameState {
        let mut state = GameState {
            state: novel.new_state("start"),
            novel,
            resources,
            continue_method: ContinueMethod::Normal,
            sfx: None,
            music: None,
            ui_sfx: Rc::new(RefCell::new(None)),
            screen: GameScreen {
                current_background: None,
                current_characters: CharacterContainer {
                    current: Vec::new(),
                },
                action: Action::None,
                ui: UI {
                    menu: StackContainer {
                        children: Vec::new(),
                        position: Position::BottomLeft.add_in(ctx, (10.0, 40.0)),
                        cell_size: (50.0, 30.0),
                        spacing: 5.0,
                        direction: Direction::Horizontal,
                    },
                },
            },
        };
        for (n, d) in [
            ("Save", MenuButtonId::Save),
            ("Load", MenuButtonId::Load),
            ("Auto", MenuButtonId::Auto),
            ("Skip", MenuButtonId::Skip),
        ]
        .iter()
        .enumerate()
        {
            state.screen.ui.menu.children.push(
                Button::new(
                    &resources,
                    &resources.button,
                    state.screen.ui.menu.get_rect_for(n as f32),
                    d.0.into(),
                    d.1,
                    state.ui_sfx.clone(),
                )
                .unwrap(),
            )
        }
        state.continue_text(ctx).unwrap();
        state
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveData {
    pub state: novelscript::NovelState,
    pub current_background: Option<String>,
    pub current_characters: Vec<(String, String)>,
}

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
    pub button_color: Color,
    pub button_pressed_color: Color,
    pub button_highlight_color: Color,
}

#[derive(Debug)]
pub struct Config {
    pub characters: HashMap<String, CharacterConfig>,
    pub ui: UIConfig,
}

pub struct Resources {
    pub text_box: graphics::Image,
    pub button: graphics::Image,
    pub config: Config,
}

impl GameState {
    fn continue_text(&mut self, ctx: &mut Context) -> ggez::GameResult {
        match self.novel.next(&mut self.state) {
            Some(novelscript::SceneNodeUser::Data(node)) => {
                crate::node::load_data_node(
                    ctx,
                    &mut self.screen,
                    node,
                    &self.resources,
                    self.ui_sfx.clone(),
                )?;
            }
            Some(novelscript::SceneNodeUser::Load(node)) => {
                crate::node::load_load_node(
                    ctx,
                    &mut self.screen,
                    node.clone(),
                    &mut self.sfx,
                    &mut self.music,
                )?;
                self.continue_text(ctx).unwrap();
            }
            None => {}
        };
        Ok(())
    }

    pub fn on_save_click(&mut self, ctx: &mut Context) {
        println!("Saving game");
        serde_json::to_writer(
            ggez::filesystem::create(ctx, "/save.json").unwrap(),
            &SaveData {
                state: self.state.clone(), // Must clone to be able to be serialized
                current_characters: self
                    .screen
                    .current_characters
                    .current
                    .iter()
                    .map(|n| {
                        let cur = n.get_current();
                        (cur.name.clone(), cur.expression.clone()) // Must clone to be able to be serialized
                    })
                    .collect(),
                current_background: self
                    .screen
                    .current_background
                    .as_ref()
                    .map(|n| n.current.get_current().1.name.clone()), // Must clone to be able to be serialized
            },
        )
        .unwrap();
        println!("Saved game!");
    }

    pub fn on_load_click(&mut self, ctx: &mut Context) {
        println!("Loading game!");
        if ggez::filesystem::exists(ctx, "/save.json") {
            let file = ggez::filesystem::open(ctx, "/save.json").unwrap();
            let savedata: SaveData = serde_json::from_reader(file).unwrap();
            self.state = savedata.state;
            self.screen.current_characters.current = Vec::new();
            for (name, expression) in savedata.current_characters {
                let character = load_character_tween(ctx, name, expression, "").unwrap();
                self.screen
                    .current_characters
                    .current
                    .push(Box::new(character));
            }
            if let Some(name) = savedata.current_background {
                let background = load_background_tween(ctx, None, name).unwrap();
                self.screen.current_background = Some(BackgroundContainer {
                    current: Box::new(background),
                });
            } else {
                self.screen.current_background = None;
            }

            self.continue_text(ctx).unwrap();
            println!("Loaded game!");
        } else {
            println!("Unable to find save file");
        }
    }

    pub fn change_state(&mut self, _ctx: &mut Context) -> Option<State> {
        None
    }

    fn advance_text(&mut self, ctx: &mut Context) {
        if let Action::Text(text) = &mut self.screen.action {
            if self.continue_method == ContinueMethod::Normal {
                if text.content.0.is_done() {
                    self.continue_text(ctx).unwrap();
                } else {
                    text.content.0.finish();
                }
            }
        }
    }
}

impl StateEventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> ggez::GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        if let Action::Text(textbox) = &self.screen.action {
            match self.continue_method {
                ContinueMethod::Skip(ref mut n) => {
                    *n += dt;
                    if *n >= 0.1 {
                        *n = 0.0;
                        self.continue_text(ctx)?;
                    }
                }
                ContinueMethod::Auto(ref mut n) => {
                    if textbox.content.0.is_done() {
                        *n += dt;
                        if *n >= 1.0 {
                            *n = 0.0;
                            self.continue_text(ctx)?;
                        }
                    }
                }
                _ => {}
            }
        } else if let Action::Choice(..) = self.screen.action {
            if let ContinueMethod::Skip(..) = self.continue_method {
                self.continue_method = ContinueMethod::Normal;
            }
        }
        self.screen.update(dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, param: DrawParam) -> ggez::GameResult {
        self.screen.draw(ctx, param)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Space | KeyCode::Return => {
                self.advance_text(ctx);
            }
            _ => (),
        }
    }

    fn text_input_event(&mut self, ctx: &mut Context, ch: char) {
        if let Action::Choice(choices) = &mut self.screen.action {
            if let Some(n) = ch.to_digit(10) {
                if n >= 1 && n < choices.children.len() as u32 {
                    self.state.set_choice(n as i32);
                    self.continue_text(ctx).unwrap();
                }
            }
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if let Action::Choice(choices) = &mut self.screen.action {
            for button in &mut choices.children {
                button.mouse_motion_event(ctx, x, y);
            }
        }

        for button in &mut self.screen.ui.menu.children {
            button.mouse_motion_event(ctx, x, y);
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        if let Action::Choice(container) = &self.screen.action {
            if let Some(n) = container
                .children
                .iter()
                .find_map(|button| button.click_event(ctx, x, y))
            {
                self.state.set_choice(n as i32 + 1);
                self.continue_text(ctx).unwrap();
            }
        }

        if let Some(e) = self
            .screen
            .ui
            .menu
            .children
            .iter()
            .find_map(|button| button.click_event(ctx, x, y))
        {
            match e {
                MenuButtonId::Save => self.on_save_click(ctx),
                MenuButtonId::Load => self.on_load_click(ctx),
                MenuButtonId::Skip => self.continue_method = ContinueMethod::Skip(0.0),
                MenuButtonId::Auto => self.continue_method = ContinueMethod::Auto(0.0),
            }
        }
    }
}

use crate::containers::{
    background::BackgroundContainer, button::Button, character::CharacterContainer, screen::Action,
    screen::Screen, stackcontainer::Direction, stackcontainer::StackContainer, ui::MenuButtonId,
    ui::UI, Draw, Update,
};
use crate::helpers::Position;
use crate::node::{load_background_tween, load_character_tween};
use ggez::event;
use ggez::graphics;
use ggez::{
    self,
    event::{KeyCode, KeyMods, MouseButton},
    Context,
};

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

pub struct GameState {
    pub novel: novelscript::Novel,
    pub state: novelscript::NovelState,
    pub resources: &'static Resources,
    pub screen: Screen,
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
            screen: Screen {
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
        state.screen.ui.menu.init(
            ctx,
            vec![("Save", MenuButtonId::Save), ("Load", MenuButtonId::Load)],
            |ctx, _idx, d, rect| {
                Button::new(
                    ctx,
                    rect,
                    d.0.into(),
                    d.1,
                )
                .unwrap()
            },
        );
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

pub struct Resources {
    pub text_box: graphics::Image,
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
                )?;
            }
            Some(novelscript::SceneNodeUser::Load(node)) => {
                crate::node::load_load_node(ctx, &mut self.screen, node.clone())?;
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
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> ggez::GameResult {
        self.screen.update(ggez::timer::delta(ctx).as_secs_f32());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.screen.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Space | KeyCode::Escape => {
                if let Action::Text(..) = &mut self.screen.action {
                    self.continue_text(ctx).unwrap();
                }
            }
            _ => (),
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
            }
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}

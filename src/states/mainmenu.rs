use std::{cell::RefCell, io::BufReader, rc::Rc};

use crate::{
    containers::{
        button::Button, mainmenuscreen::MainMenuScreen, mainmenuscreen::MenuButtonId,
        stackcontainer::Direction, stackcontainer::StackContainer, Draw,
    },
    helpers::Position,
};
use ggez::{
    audio::SoundSource,
    event::{self, EventHandler, MouseButton},
    filesystem,
    graphics::{self, drawable_size},
    Context,
};

use super::{
    game::{GameState, Resources},
    State,
};

pub struct MainMenuState {
    pub resources: &'static Resources,
    pub screen: MainMenuScreen,
    pub clicked_event: Option<MenuButtonId>,
    pub music: ggez::audio::Source,
    pub ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
}

impl MainMenuState {
    pub fn new(ctx: &mut Context, resources: &'static Resources) -> Self {
        let mut state = Self {
            resources,
            clicked_event: None,
            screen: MainMenuScreen {
                background: graphics::Image::new(ctx, "/MainMenuBackground.png").unwrap(),
                panel: graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT),
                    graphics::Rect {
                        x: 0.0,
                        y: 0.0,
                        w: drawable_size(ctx).0 / 2.5,
                        h: drawable_size(ctx).1,
                    },
                    graphics::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 0.8,
                    },
                )
                .unwrap(),
                menu: StackContainer {
                    children: Vec::new(),
                    position: Position::TopLeft.add_in(
                        ctx,
                        (
                            drawable_size(ctx).0 / (2.5 * 2.0) - (300.0 / 2.0),
                            drawable_size(ctx).1 / 2.0 - (60.0 * 2.0 / 2.0),
                        ),
                    ),
                    cell_size: (300.0, 60.0),
                    spacing: 5.0,
                    direction: Direction::Vertical,
                },
            },
            music: ggez::audio::Source::new(ctx, "/audio/bgm.mp3").unwrap(),
            ui_sfx: Rc::new(RefCell::new(None)),
        };
        for (n, d) in [("Start", MenuButtonId::Start), ("Quit", MenuButtonId::Quit)]
            .iter()
            .enumerate()
        {
            state.screen.menu.children.push(
                Button::new(
                    &resources.button,
                    state.screen.menu.get_rect_for(n as f32),
                    d.0.into(),
                    d.1,
                    state.ui_sfx.clone(),
                )
                .unwrap(),
            )
        }
        state.music.play(ctx).unwrap();
        state
    }

    pub fn change_state(&mut self, ctx: &mut Context) -> Option<State> {
        if let Some(MenuButtonId::Start) = self.clicked_event {
            let mut novel = novelscript::Novel::new();
            novel
                .add_scene(
                    "start".into(),
                    BufReader::new(filesystem::open(ctx, "/test.ns").unwrap()),
                )
                .unwrap();
            Some(State::Game(GameState::new(ctx, novel, self.resources)))
        } else {
            None
        }
    }
}

impl EventHandler for MainMenuState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let Some(e) = self.clicked_event {
            match e {
                MenuButtonId::Start => {} // Handled in change_state
                MenuButtonId::Quit => {
                    event::quit(ctx);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.screen.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        for button in &mut self.screen.menu.children {
            button.mouse_motion_event(ctx, x, y);
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        if let Some(e) = self
            .screen
            .menu
            .children
            .iter()
            .find_map(|button| button.click_event(ctx, x, y))
        {
            self.clicked_event = Some(e);
        }
    }
}

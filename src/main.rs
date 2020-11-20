use std::io::BufReader;

use ggez::{conf::{WindowMode, WindowSetup}, graphics};
use ggez::nalgebra as na;
use ggez::{
    self,
    event::{KeyCode, KeyMods, MouseButton},
    Context,
};
use ggez::{event, graphics::Color};

struct MainState<'a> {
    it: novelscript::NovelIterator<'a>,
    current_node: Option<&'a novelscript::SceneNodeData>,
    hovered_choice: u32,
}

impl<'a> MainState<'a> {
    fn new(novel: &'a novelscript::Novel) -> MainState<'a> {
        MainState {
            it: novel.iter("start"),
            current_node: None,
            hovered_choice: 0,
        }
    }
}

fn get_item_position(n: f32, max: f32) -> f32 {
    (300.0 - (50.0 * max / 2.0)) + (50.0 * n)
}

fn get_item_index(y: f32, max: f32) -> u32 {
    let start_y = get_item_position(0.0, max);
    ((y - start_y) / 50.0) as u32
}

impl<'a> event::EventHandler for MainState<'a> {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        if let Some(node) = self.current_node {
            if let novelscript::SceneNodeData::Text { speaker, content } = node {
                {
                    let layer = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(5.0, 375.0, 800.0 - 10.0, 600.0 - 380.0),
                        Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 0.25,
                        },
                    )?;
                    graphics::draw(ctx, &layer, (na::Point2::new(0.0, 0.0),))?;
                }
                if let Some(speaker) = speaker {
                    let text = graphics::Text::new(speaker.as_str());
                    graphics::draw(ctx, &text, (na::Point2::new(10.0, 380.0),))?;
                }
                let mut text = graphics::Text::new(content.as_str());
                text.set_bounds(
                    na::Point2::new(800.0 - 10.0, 600.0 - 375.0),
                    graphics::Align::Left,
                );
                graphics::draw(ctx, &text, (na::Point2::new(10.0, 400.0),))?;
            } else if let novelscript::SceneNodeData::Choice(choices) = node {
                for (n, choice) in choices.iter().enumerate() {
                    let mut text = graphics::Text::new(choice.as_str());
                    text.set_bounds(na::Point2::new(200.0, 16.0), graphics::Align::Center);
                    let height = text.height(ctx) as f32;
                    let pos = na::Point2::new(
                        300.0,
                        get_item_position(n as f32, choices.len() as f32) + height / 2.0,
                    );
                    graphics::draw(ctx, &text, (pos,))?;
                    {
                        let layer = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect::new(295.0, pos.y, 210.0, 40.0),
                            if n == self.hovered_choice as usize {
                                Color {
                                    r: 1.0,
                                    g: 1.0,
                                    b: 1.0,
                                    a: 0.25,
                                }
                            } else {
                                Color {
                                    r: 0.25,
                                    g: 0.25,
                                    b: 0.25,
                                    a: 0.25,
                                }
                            },
                        )?;
                        graphics::draw(ctx, &layer, (na::Point2::new(0.0, 0.0),))?;
                    }
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Space => {
                if !matches!(
                    self.current_node,
                    Some(novelscript::SceneNodeData::Choice(..))
                ) {
                    self.current_node = self.it.next();
                }
            }
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, y: f32, _dx: f32, _dy: f32) {
        if let Some(novelscript::SceneNodeData::Choice(choices)) = self.current_node {
            if y > get_item_position(0.0, choices.len() as f32)
                && y < get_item_position(choices.len() as f32, choices.len() as f32)
            {
                self.hovered_choice = get_item_index(y, choices.len() as f32);
            }
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        y: f32,
    ) {
        if let Some(novelscript::SceneNodeData::Choice(choices)) = self.current_node {
            if y > get_item_position(0.0, choices.len() as f32)
                && y < get_item_position(choices.len() as f32, choices.len() as f32)
            {
                let idx = get_item_index(y, choices.len() as f32);
                self.it.set_variable("choice".into(), idx as i32 + 1);
                self.hovered_choice = 0;
                self.current_node = self.it.next();
            }
        }
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("ns-engine", "nobbele")
    .window_setup(WindowSetup::default().title("NS Engine"))
    .window_mode(
        WindowMode::default()
            .dimensions(800.0, 600.0)
            .resizable(false),
    );
    let (ctx, event_loop) = &mut cb.build()?;

    let mut novel = novelscript::Novel::new();
    novel
        .add_scene(
            "start".into(),
            BufReader::new(std::fs::File::open("test.ns").unwrap()),
        )
        .unwrap();

    let state = &mut MainState::new(&novel);
    event::run(ctx, event_loop, state)
}

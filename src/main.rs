use std::io::BufReader;

use ggez::filesystem;
use ggez::mint as na;
use ggez::{
    self,
    event::{KeyCode, KeyMods, MouseButton},
    Context,
};
use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics::{self, drawable_size},
};
use ggez::{event, graphics::Color};

struct MainState {
    novel: novelscript::Novel,
    state: novelscript::NovelState,
    current_node: Option<novelscript::SceneNodeData>,
    hovered_choice: u32,
    resources: Resources,
}

impl MainState {
    fn new(novel: novelscript::Novel, resources: Resources) -> MainState {
        MainState {
            state: novel.new_state("start"),
            novel,
            current_node: None,
            hovered_choice: 0,
            resources,
        }
    }
}

fn get_item_y(ctx: &Context, n: f32, max: f32) -> f32 {
    let offset = (-50.0 * max / 2.0) + (50.0 * n);
    Position::Center.add_in(ctx, (0.0, offset)).y
}

fn get_item_index(ctx: &Context, y: f32, max: f32) -> u32 {
    let start_y = get_item_y(ctx, 0.0, max);
    ((y - start_y) / 50.0) as u32
}

pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Position {
    pub fn add_in(&self, ctx: &Context, offset: (f32, f32)) -> na::Point2<f32> {
        self.add_in_from(
            &graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: drawable_size(ctx).0,
                h: drawable_size(ctx).1,
            },
            offset,
        )
    }

    pub fn add_in_from(&self, rect: &graphics::Rect, offset: (f32, f32)) -> na::Point2<f32> {
        match self {
            Position::TopLeft => [rect.x + offset.0, rect.y + offset.1],
            Position::TopRight => [(rect.x + rect.w) - offset.0, rect.y + offset.1],
            Position::BottomLeft => [rect.x + offset.0, (rect.y + rect.h) - offset.1],
            Position::BottomRight => [(rect.x + rect.w) - offset.0, (rect.y + rect.h) - offset.1],
            Position::Center => [
                (rect.x + rect.w) / 2.0 + offset.0,
                (rect.y + rect.h) / 2.0 + offset.1,
            ],
        }
        .into()
    }
}

fn points_to_rect(a: na::Point2<f32>, b: na::Point2<f32>) -> graphics::Rect {
    graphics::Rect {
        x: a.x,
        y: a.y,
        w: b.x - a.x,
        h: b.y - a.y,
    }
}

struct Resources {
    text_box: graphics::Image,
}

fn draw_text(
    ctx: &mut Context,
    resources: &Resources,
    speaker: &Option<String>,
    content: &str,
) -> ggez::GameResult {
    let layer_bounds = {
        let bounds = points_to_rect(
            Position::BottomLeft.add_in(ctx, (0.0, 200.0)),
            Position::BottomRight.add_in(ctx, (0.0, 0.0)),
        );
        /*let layer = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            bounds,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.25,
            },
        )?;
        graphics::draw(ctx, &layer, (na::Point2::new(0.0, 0.0),))?;*/
        let image = &resources.text_box;
        graphics::draw(
            ctx,
            image,
            graphics::DrawParam::new()
                .dest([bounds.x, bounds.y])
                .scale([
                    bounds.w / image.width() as f32,
                    bounds.h / image.height() as f32,
                ]),
        )?;
        bounds
    };
    if let Some(speaker) = speaker {
        let mut text = graphics::Text::new(speaker.as_str());
        text.set_bounds([f32::INFINITY, f32::INFINITY], graphics::Align::Left);
        graphics::draw(
            ctx,
            &text,
            (Position::TopLeft.add_in_from(&layer_bounds, (15.0, 20.0)),),
        )?;
    }
    let mut text = graphics::Text::new(content);
    text.set_bounds(
        [layer_bounds.w - 10.0, layer_bounds.h - 10.0],
        graphics::Align::Left,
    );
    graphics::draw(
        ctx,
        &text,
        (Position::TopLeft.add_in_from(&layer_bounds, (15.0, 55.0)),),
    )?;
    Ok(())
}

fn draw_choices(ctx: &mut Context, choices: &[String], hovered_choice: u32) -> ggez::GameResult {
    for (n, choice) in choices.iter().enumerate() {
        let layer_bounds = {
            let size = (210.0, 40.0);
            let pos: na::Point2<f32> = [
                Position::Center.add_in(ctx, (-size.0 / 2.0, 0.0)).x,
                get_item_y(ctx, n as f32, choices.len() as f32),
            ]
            .into();
            let bounds = points_to_rect(pos, [pos.x + size.0, pos.y + size.1].into());
            let layer = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                bounds,
                if n == hovered_choice as usize {
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
            graphics::draw(ctx, &layer, ([0.0, 0.0],))?;
            bounds
        };
        let mut text = graphics::Text::new(choice.as_str());
        text.set_bounds(
            [layer_bounds.w - 10.0, layer_bounds.h - 10.0],
            graphics::Align::Center,
        );

        let text_pos = Position::TopLeft.add_in_from(
            &layer_bounds,
            (0.0, layer_bounds.h / 2.0 - text.height(ctx) as f32 / 2.0),
        );
        graphics::draw(ctx, &text, (text_pos,))?;
    }
    Ok(())
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        if let Some(node) = &self.current_node {
            if let novelscript::SceneNodeData::Text { speaker, content } = node {
                draw_text(ctx, &self.resources, speaker, content)?;
            } else if let novelscript::SceneNodeData::Choice(choices) = node {
                draw_choices(ctx, choices, self.hovered_choice)?;
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
                    self.current_node = self.novel.next(&mut self.state);
                }
            }
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, _x: f32, y: f32, _dx: f32, _dy: f32) {
        if let Some(novelscript::SceneNodeData::Choice(choices)) = &self.current_node {
            if y > get_item_y(ctx, 0.0, choices.len() as f32)
                && y < get_item_y(ctx, choices.len() as f32, choices.len() as f32)
            {
                self.hovered_choice = get_item_index(ctx, y, choices.len() as f32);
            }
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        y: f32,
    ) {
        if let Some(novelscript::SceneNodeData::Choice(choices)) = &self.current_node {
            if y > get_item_y(ctx, 0.0, choices.len() as f32)
                && y < get_item_y(ctx, choices.len() as f32, choices.len() as f32)
            {
                let idx = get_item_index(ctx, y, choices.len() as f32);
                self.state.set_variable("choice".into(), idx as i32 + 1);
                self.hovered_choice = 0;
                self.current_node = self.novel.next(&mut self.state);
            }
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}

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

    let resources = Resources {
        text_box: graphics::Image::new(&mut ctx, "/TextBox.png")?,
    };

    let state = MainState::new(novel, resources);
    event::run(ctx, event_loop, state)
}

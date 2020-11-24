use ggez::{graphics, Context};
use novelscript::SceneNodeUser;

use crate::{Background, Character, Placement, Resources, containers::{background::BackgroundContainer, screen::Screen}, draw::{draw_choices, draw_text}, tween::TargetTweener, tween::TransitionTweener};

pub fn load_character_tween(
    ctx: &mut Context,
    name: String,
    expression: String,
    placement: &str,
) -> ggez::GameResult<TargetTweener<Character, impl Fn(&mut Character, f32)>> {
    Ok(TargetTweener {
        time: 0.0,
        target: 0.75,
        update: |cur: &mut Character, progress| {
            cur.alpha = progress;
        },
        current: Character {
            alpha: 0.0,
            image: graphics::Image::new(ctx, format!("/char/{}/{}.png", name, expression))?,
            name,
            expression,
            position: match &placement[..] {
                "Left" => Some(Placement::Left),
                "Right" => Some(Placement::Right),
                _ => None,
            },
        },
    })
}

pub fn load_background_tween(
    ctx: &mut Context,
    prev: Option<Background>,
    name: String,
) -> ggez::GameResult<
    TransitionTweener<Background, impl Fn(&mut Option<Background>, &mut Background, f32)>,
> {
    let prev = prev.map(|mut n| {
        n.fade = 0.0;
        n
    });
    Ok(TransitionTweener {
        time: 0.0,
        target: 0.5,
        set_instantly_if_no_prev: true,
        current: (
            prev,
            Background::new(
                graphics::Image::new(ctx, format!("/bg/{}.png", name))?,
                name,
            ),
        ),
        update: |prev: &mut Option<Background>, to: &mut Background, progress| {
            if let Some(prev) = prev {
                prev.fade = 1.0 / progress;
            }
            to.fade = progress;
        },
    })
}

pub fn load_node(ctx: &mut Context, screen: &mut Screen, node: SceneNodeUser) -> ggez::GameResult {
    if let novelscript::SceneNodeUser::Load(node) = node {
        if let novelscript::SceneNodeLoad::Character {
            character,
            expression,
            placement,
        } = node
        {
            let tween = load_character_tween(ctx, character, expression, &placement)?;
            screen.current_characters.current.insert(
                match tween.current.position {
                    Some(Placement::Left) => 0,
                    Some(Placement::Right) => screen.current_characters.current.len(),
                    None => 0,
                },
                Box::new(tween),
            );
        } else if let novelscript::SceneNodeLoad::Background { name } = node {
            let prev = screen
                .current_background
                .take()
                .map(|n| n.current.take_final_box().1);
                screen.current_background = Some(BackgroundContainer {
                    current: Box::new(load_background_tween(ctx, prev, name)?)
                });
        }
    }
    Ok(())
}

pub fn draw_node(
    ctx: &mut Context,
    screen: &mut Screen,
    current_node: &Option<novelscript::SceneNodeUser>,
    resources: &Resources,
    hovered_choice: u32,
) -> ggez::GameResult {
    let node = current_node.as_ref().unwrap();
    if let novelscript::SceneNodeUser::Data(node) = node {
        if let novelscript::SceneNodeData::Text { speaker, content } = node {
            draw_text(ctx, &resources, speaker, content)?;
        } else if let novelscript::SceneNodeData::Choice(choices) = node {
            draw_choices(ctx, screen, choices, hovered_choice)?;
        }
    }
    Ok(())
}

use ggez::{graphics, Context};

use crate::{
    draw::{draw_choices, draw_text},
    tween::TargetTweener,
    tween::TransitionTweener,
    Character, Background, MainState, Placement,
};

pub fn load_character_tween(
    ctx: &mut Context,
    character: &String,
    expression: &String,
    placement: &String,
) -> ggez::GameResult<TargetTweener<Character, impl Fn(&mut Character, f32)>> {
    Ok(TargetTweener {
        time: 0.0,
        target: 0.75,
        update: |cur: &mut Character, progress| {
            cur.alpha = progress;
        },
        current: Character {
            alpha: 0.0,
            name: character.clone(),
            expression: expression.clone(),
            image: graphics::Image::new(ctx, format!("/char/{}/{}.png", character, expression))?,
            position: match &placement[..] {
                "Left" => Some(Placement::Left),
                "Right" => Some(Placement::Right),
                _ => None,
            },
        },
    })
}

pub fn load_background_tween(ctx: &mut Context, prev_clone: Option<Background>, name: &String) -> ggez::GameResult<TransitionTweener<Background, impl Fn(&mut Option<Background>, &mut Background, f32)>> {
    Ok(TransitionTweener {
        time: 0.0,
        target: 0.5,
        set_instantly_if_no_prev: true,
        current: (
            prev_clone,
            Background::new(name.clone(), graphics::Image::new(ctx, format!("/bg/{}.png", name))?),
        ),
        update: |prev: &mut Option<Background>, to: &mut Background, progress| {
            if let Some(prev) = prev {
                prev.fade = 1.0 / progress;
            }
            to.fade = progress;
        },
    })
}

pub fn draw_node(state: &mut MainState, ctx: &mut Context) -> ggez::GameResult {
    if let Some(node) = &state.current_node {
        if let novelscript::SceneNodeData::Text { speaker, content } = node {
            draw_text(ctx, &state.resources, speaker, content)?;
        } else if let novelscript::SceneNodeData::Choice(choices) = node {
            draw_choices(ctx, choices, state.hovered_choice)?;
        } else if let novelscript::SceneNodeData::LoadCharacter {
            character,
            expression,
            placement,
        } = node
        {
            let tween = load_character_tween(ctx, character, expression, placement)?;
            state.current_characters.insert(
                match tween.current.position {
                    Some(Placement::Left) => 0,
                    Some(Placement::Right) => state.current_characters.len(),
                    None => 0,
                },
                Box::new(tween),
            );
            state.continue_text();
        } else if let novelscript::SceneNodeData::LoadBackground { name } = node {
            let prev_clone = state
                .current_background
                .as_ref()
                .map(|n| n.get_current().1.clone())
                .map(|mut n| {
                    n.fade = 0.0;
                    n
                });
            state.current_background = Some(Box::new(load_background_tween(ctx, prev_clone, name)?));
            state.continue_text();
            draw_node(state, ctx)?;
        }
    }
    Ok(())
}

use std::{cell::RefCell, rc::Rc};

use ggez::Context;
use novelscript::SceneNodeLoad;

use crate::{
    containers::{background::BackgroundContainer, gamescreen::GameScreen},
    containers::{button::Button, gamescreen::Action, stackcontainer::StackContainer},
    draw::load_text,
    helpers::Position,
    resource_manager::ResourceManager,
    states::game::Character,
    states::game::{Background, Placement},
    tween::TargetTweener,
    tween::TransitionTweener,
};

pub fn load_character_tween(
    ctx: &mut Context,
    resources: &'static ResourceManager,
    name: String,
    expression: String,
    placement: Placement,
) -> ggez::GameResult<TargetTweener<Character, impl Fn(&mut Character, f32)>> {
    Ok(TargetTweener::new(
        0.75,
        Character {
            alpha: 0.0,
            image: resources.get_image(ctx, &format!("/char/{}/{}.png", name, expression)),
            name,
            expression,
            position: placement,
        },
        |cur: &mut Character, progress| {
            cur.alpha = progress;
        },
    ))
}

pub fn load_background_tween(
    ctx: &mut Context,
    resources: &'static ResourceManager,
    prev: Option<Background>,
    name: String,
) -> ggez::GameResult<
    TransitionTweener<
        Background,
        Background,
        impl Fn(&mut Option<Background>, &mut Background, f32),
    >,
> {
    let prev = prev.map(|mut n| {
        n.fade = 0.0;
        n
    });
    Ok(TransitionTweener::new(
        true,
        0.5,
        (
            prev,
            Background::new(resources.get_image(ctx, &format!("/bg/{}.png", name)), name),
        ),
        |prev: &mut Option<Background>, to: &mut Background, progress| {
            if let Some(prev) = prev {
                prev.fade = 1.0 / progress;
            }
            to.fade = progress;
        },
    ))
}

pub fn load_load_node(
    ctx: &mut Context,
    resources: &'static ResourceManager,
    screen: &mut GameScreen,
    node: SceneNodeLoad,
    sfx: &mut Option<ggez::audio::Source>,
    music: &mut Option<ggez::audio::Source>,
) -> ggez::GameResult {
    if let novelscript::SceneNodeLoad::Character {
        character,
        expression,
        placement,
    } = node
    {
        // TODO use this until a proper change character command is added to novelscript
        if let Some(c) = screen
            .current_characters
            .current
            .iter_mut()
            .find(|c| c.get_current().name == character)
        {
            let current = c.get_current();
            *c = Box::new(load_character_tween(
                ctx,
                resources,
                character,
                expression.unwrap_or(current.expression.clone()),
                placement
                    .map(|s| Placement::parse(s.to_lowercase().as_str()))
                    .unwrap_or(current.position),
            )?);
            c.finish();
        } else {
            let tween = load_character_tween(
                ctx,
                resources,
                character,
                expression.unwrap_or_else(|| "Normal".to_owned()),
                placement
                    .map(|s| Placement::parse(s.to_lowercase().as_str()))
                    .unwrap_or(Placement::Left),
            )?;
            screen.current_characters.current.insert(
                match tween.current.position {
                    Placement::Left => 0,
                    Placement::Right => screen.current_characters.current.len(),
                },
                Box::new(tween),
            );
        }
    } else if let novelscript::SceneNodeLoad::Background { name } = node {
        let prev = screen
            .current_background
            .take()
            .map(|n| n.current.take_final_box().1);
        screen.current_background = Some(BackgroundContainer {
            current: Box::new(load_background_tween(ctx, resources, prev, name)?),
        });
    } else if let novelscript::SceneNodeLoad::PlaySound { name, channel } = node {
        let src = match channel.as_str() {
            "sfx" => sfx,
            "music" => music,
            _ => panic!("invalid channel `{}` for sound `{}`", channel, name),
        };
        println!("Loading {} {}", name, channel);
        src.replace(resources.get_sound_source(ctx, &format!("/audio/{}", name)));
    } else if let novelscript::SceneNodeLoad::RemoveCharacter { name } = node {
        if let Some(idx) = screen
            .current_characters
            .current
            .iter()
            .position(|c| c.get_current().name == name)
        {
            screen.current_characters.current.remove(idx);
        }
    }
    Ok(())
}

pub fn load_data_node(
    ctx: &mut Context,
    screen: &mut GameScreen,
    node: &novelscript::SceneNodeData,
    resources: &'static ResourceManager,
    ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
) -> ggez::GameResult {
    if let novelscript::SceneNodeData::Text { speaker, content } = node {
        load_text(ctx, screen, resources, speaker, content)?;
    } else if let novelscript::SceneNodeData::Choice(choices) = node {
        let mut stack = StackContainer::new(
            Position::Center.add_in(
                ctx,
                glam::Vec2::new(250.0 / -2.0, 60.0 * choices.len() as f32 / -2.0),
            ),
            5.0,
            (250.0, 50.0),
            crate::containers::stackcontainer::Direction::Vertical,
        );
        for (n, d) in choices.iter().enumerate() {
            stack.children.push((
                Button::new(
                    ctx,
                    resources,
                    stack.get_rect_for(n as f32),
                    d.clone(),
                    ui_sfx.clone(),
                )
                .unwrap(),
                n as u32,
            ))
        }
        screen.action = Action::Choice(stack);
    }
    Ok(())
}

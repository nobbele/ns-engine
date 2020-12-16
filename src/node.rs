use std::{cell::RefCell, rc::Rc};

use ggez::{audio::SoundSource, graphics, Context};
use novelscript::SceneNodeLoad;

use crate::{
    containers::{background::BackgroundContainer, gamescreen::GameScreen},
    containers::{button::Button, gamescreen::Action, stackcontainer::StackContainer},
    draw::load_text,
    helpers::Position,
    states::game::Character,
    states::game::{Background, Config, Placement},
    tween::TargetTweener,
    tween::TransitionTweener,
    Resources,
};

pub fn load_character_tween(
    ctx: &mut Context,
    name: String,
    expression: String,
    placement: &str,
) -> ggez::GameResult<TargetTweener<Character, impl Fn(&mut Character, f32)>> {
    Ok(TargetTweener::new(
        0.75,
        Character {
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
        |cur: &mut Character, progress| {
            cur.alpha = progress;
        },
    ))
}

pub fn load_background_tween(
    ctx: &mut Context,
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
            Background::new(
                graphics::Image::new(ctx, format!("/bg/{}.png", name))?,
                name,
            ),
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
            current: Box::new(load_background_tween(ctx, prev, name)?),
        });
    } else if let novelscript::SceneNodeLoad::PlaySound { name, channel } = node {
        let channel = channel.as_deref().unwrap_or("sfx");
        let src = match channel {
            "sfx" => sfx,
            "music" => music,
            _ => panic!(),
        };
        let mut new_src = ggez::audio::Source::new(ctx, format!("/audio/{}.mp3", name)).unwrap();
        new_src.play(ctx).unwrap();
        *src = Some(new_src);
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
    resources: &'static Resources,
    ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
    config: &'static Config,
) -> ggez::GameResult {
    if let novelscript::SceneNodeData::Text { speaker, content } = node {
        load_text(ctx, screen, resources, speaker, content, config)?;
    } else if let novelscript::SceneNodeData::Choice(choices) = node {
        let mut stack = StackContainer::new(
            Position::Center.add_in(ctx, (250.0 / -2.0, 60.0 * choices.len() as f32 / -2.0)),
            5.0,
            (250.0, 50.0),
            crate::containers::stackcontainer::Direction::Vertical,
        );
        for (n, d) in choices.iter().enumerate() {
            stack.children.push((
                Button::new(
                    &resources.button,
                    stack.get_rect_for(n as f32),
                    d.clone(),
                    ui_sfx.clone(),
                    &config,
                )
                .unwrap(),
                n as u32,
            ))
        }
        screen.action = Action::Choice(stack);
    }
    Ok(())
}

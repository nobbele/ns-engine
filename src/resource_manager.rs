use std::{cell::RefCell, collections::HashMap, sync::Arc};

use ggez::{
    audio::{SoundData, Source},
    graphics::Image,
    Context,
};
use log::warn;

use crate::config::Config;

#[derive(Debug)]
struct ResourceManagerImpl {
    image_cache: HashMap<String, Image>,
    sound_cache: HashMap<String, SoundData>,
    config: Arc<Config>,
}

#[derive(Debug)]
pub struct ResourceManager(RefCell<ResourceManagerImpl>);

impl ResourceManager {
    pub fn new(config: Config) -> Self {
        ResourceManager(RefCell::new(ResourceManagerImpl {
            image_cache: HashMap::new(),
            sound_cache: HashMap::new(),
            config: Arc::new(config),
        }))
    }

    pub fn get_image(&self, ctx: &mut Context, path: &str) -> Image {
        let imp = self.0.borrow();
        if let Some(o) = imp.image_cache.get(path) {
            o.clone()
        } else {
            drop(imp);
            let image = find_image(ctx, path);
            self.0
                .borrow_mut()
                .image_cache
                .insert(path.to_owned(), image.clone());
            image
        }
    }

    pub fn get_sound(&self, ctx: &mut Context, path: &str) -> SoundData {
        let imp = self.0.borrow();
        if let Some(o) = imp.sound_cache.get(path) {
            o.clone()
        } else {
            drop(imp);
            let sound = find_sound(ctx, path);
            self.0
                .borrow_mut()
                .sound_cache
                .insert(path.to_owned(), sound.clone());
            sound
        }
    }

    pub fn get_sound_source(&self, ctx: &mut Context, path: &str) -> Source {
        let data = self.get_sound(ctx, path);
        Source::from_data(ctx, data).unwrap()
    }

    pub fn get_config(&self) -> Arc<Config> {
        self.0.borrow().config.clone()
    }
}

fn find_image(ctx: &mut Context, path: &str) -> Image {
    let mut s: String = path.to_owned();
    if !s.starts_with('/') {
        warn!("Not prepending / in file name wastes performance");
        s.insert(0, '/');
    }
    if s.contains(".") {
        if ggez::filesystem::exists(ctx, &s) {
            return Image::new(ctx, s).unwrap();
        }
    } else {
        s.push_str(".png");
        if ggez::filesystem::exists(ctx, &s) {
            return Image::new(ctx, s).unwrap();
        }
        s.truncate(s.len() - 4);

        s.push_str(".jpg");
        if ggez::filesystem::exists(ctx, &s) {
            return Image::new(ctx, s).unwrap();
        }
        s.truncate(s.len() - 4);
    }
    panic!("Unable to find image");
}

fn find_sound(ctx: &mut Context, path: &str) -> SoundData {
    let mut s: String = path.to_owned();
    s.push_str(".mp3");
    if ggez::filesystem::exists(ctx, &s) {
        return SoundData::new(ctx, s).unwrap();
    }
    s.truncate(s.len() - 4);
    panic!("Unable to find image");
}

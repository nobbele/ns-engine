use ggez::{
    graphics::{self, drawable_size},
    Context,
};

pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Position {
    pub fn add_in(&self, ctx: &Context, offset: glam::Vec2) -> glam::Vec2 {
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

    pub fn add_in_from(&self, rect: &graphics::Rect, offset: glam::Vec2) -> glam::Vec2 {
        match self {
            Position::TopLeft => glam::Vec2::new(rect.top(), rect.left()) + offset,
            Position::TopRight => {
                glam::Vec2::new(rect.top(), rect.right()) + glam::Vec2::new(-offset.x, offset.y)
            }
            Position::BottomLeft => {
                glam::Vec2::new(rect.bottom(), rect.left()) + glam::Vec2::new(offset.x, -offset.y)
            }
            Position::BottomRight => glam::Vec2::new(rect.bottom(), rect.right()) - offset,
            Position::Center => glam::Vec2::new(rect.bottom() / 2.0, rect.right() / 2.0) + offset,
        }
    }
}

pub fn points_to_rect(a: glam::Vec2, b: glam::Vec2) -> graphics::Rect {
    graphics::Rect {
        x: a.x,
        y: a.y,
        w: b.x - a.x,
        h: b.y - a.y,
    }
}
